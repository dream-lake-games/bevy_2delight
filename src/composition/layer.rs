//! I am not mentally well :0

use std::collections::VecDeque;

use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    window::WindowResized,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::{
    camera::FollowDynamicCamera,
    mats::{
        brightness_cull_mat::BrightnessCullMat, cutout_mat::CutoutMat,
        gaussian_blur_mat::GaussianBlurMat, lit_mat::LitMat,
    },
    prelude::Lighting,
};

#[derive(Resource)]
pub(super) struct ScreenMesh(pub(super) Handle<Mesh>);
fn setup_screen_mesh(
    mut commands: Commands,
    layer_settings: Res<LayerSettings>,
    mut mesh: ResMut<Assets<Mesh>>,
) {
    let hand = mesh.add(Rectangle::new(
        layer_settings.screen_size.x as f32,
        layer_settings.screen_size.y as f32,
    ));
    commands.insert_resource(ScreenMesh(hand));
}

#[derive(Resource)]
struct LayerRoot(Entity);
impl Default for LayerRoot {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}
impl LayerRoot {
    pub(crate) fn eid(&self) -> Entity {
        self.0
    }
}

#[derive(Resource)]
pub(super) struct LightRoot(Entity);
impl Default for LightRoot {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}
impl LightRoot {
    pub(crate) fn eid(&self) -> Entity {
        self.0
    }
}
#[derive(Resource)]
pub(super) struct LightOccludeRoot(Entity);
impl Default for LightOccludeRoot {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}
impl LightOccludeRoot {
    pub(crate) fn eid(&self) -> Entity {
        self.0
    }
}

#[derive(Resource)]
pub(super) struct LayerSettings {
    pub(super) screen_size: UVec2,
}
impl LayerSettings {
    pub(crate) fn blank_screen_image(&self) -> Image {
        let target_extent = Extent3d {
            width: self.screen_size.x,
            height: self.screen_size.y,
            ..default()
        };
        // Makes the image
        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size: target_extent,
                dimension: TextureDimension::D2,
                // format: TextureFormat::bevy_default(),
                format: TextureFormat::Rgba16Float,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            ..default()
        };
        // Fills it with zeros
        image.resize(target_extent);
        image
    }
}

#[derive(PartialEq, Eq)]
pub(super) enum LayerOrder {
    /// These are all things that basically are snapshotting the world.
    PreLight = 1,
    /// Requires all the individual light layers to be rendered first.
    Light = 2,
    /// Processing that requires the lighting info
    ApplyLight = 3,
    /// Flattens the lit layers + static layer into a single image.
    /// Important before brightness culling, but also used in the final image
    FlattenMeat = 4,
    /// Requires light info, lit pixels, and brightness/reflexivity info.
    /// Will figure out how bright each pixel is and cull to only what should bloom-blur.
    /// Also handles combining the various layered brightnesses
    BrightnessCull = 5,
    /// Blurs the bloom stuff, claims orders [6,48] for bloom passes
    Blur = 6,
    PostBlur = 49,
    Smush = 50,
}

pub(super) enum LayerCameraMode {
    Hdr,
    Tonemapped(Tonemapping),
}
impl LayerCameraMode {
    fn is_hdr(&self) -> bool {
        match self {
            Self::Hdr => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Eq)]
enum LayerPosition {
    /// The camera rendering this is always at the origin
    Fixed,
    /// The camera rendering this follows the DynamicCamera, if it exists
    Dynamic,
}

#[derive(Clone, Copy, Debug, Reflect, PartialEq, Eq, EnumIter, std::hash::Hash)]
pub enum Layer {
    Dummy,
    Light,
    Bg,
    AmbientPixels,
    AmbientBrightness,
    AmbientReflexivity,
    DetailPixels,
    DetailBrightness,
    DetailReflexivity,
    StaticPixels,
    StaticBrightness,
    StaticReflexivity,
    Fg,
    Menu,
    Transition,
}
impl Layer {
    pub const fn render_layers(&self) -> RenderLayers {
        match self {
            // We make static 0 (the default) so if we ever forget to attach render layers
            // they'll show up here. Easier to debug than just having the thing not appear
            Self::StaticPixels => RenderLayers::layer(0),
            // Otherwise just increment
            Self::Dummy => RenderLayers::layer(1),
            Self::Light => RenderLayers::layer(2),
            Self::Bg => RenderLayers::layer(3),
            Self::AmbientPixels => RenderLayers::layer(4),
            Self::AmbientBrightness => RenderLayers::layer(5),
            Self::AmbientReflexivity => RenderLayers::layer(6),
            Self::DetailPixels => RenderLayers::layer(7),
            Self::DetailBrightness => RenderLayers::layer(8),
            Self::DetailReflexivity => RenderLayers::layer(9),
            Self::StaticBrightness => RenderLayers::layer(10),
            Self::StaticReflexivity => RenderLayers::layer(11),
            Self::Fg => RenderLayers::layer(12),
            Self::Menu => RenderLayers::layer(13),
            Self::Transition => RenderLayers::layer(14),
        }
    }

    const fn layer_order(&self) -> LayerOrder {
        match self {
            Self::Light => LayerOrder::Light,
            _ => LayerOrder::PreLight,
        }
    }

    const fn layer_position(&self) -> LayerPosition {
        match self {
            Self::Dummy
            | Self::AmbientPixels
            | Self::AmbientBrightness
            | Self::AmbientReflexivity
            | Self::DetailPixels
            | Self::DetailBrightness
            | Self::DetailReflexivity
            | Self::StaticPixels
            | Self::StaticBrightness
            | Self::StaticReflexivity => LayerPosition::Dynamic,
            // NOTE: Light is included (indirectly) here in fixed because each of the underlying light cameras
            //       will follow the camera, and then render back at the origin
            _ => LayerPosition::Fixed,
        }
    }

    const fn camera_mode(&self) -> LayerCameraMode {
        LayerCameraMode::Hdr
    }

    fn target(&self) -> Handle<Image> {
        Handle::weak_from_u128(self.render_layers().bits()[0] as u128)
    }
}

#[derive(Clone, Copy, Debug, Reflect, PartialEq, Eq, EnumIter, std::hash::Hash)]
pub(crate) enum InternalLayer {
    AmbientPixelsLit,
    DetailPixelsLit,
    /// Getting cheeky, but this is just ambient + detail + static combined into one
    Meat,
    /// Intermediate layer needed to combine the brightness from all the meaty parts properly
    IntermediateBrightness,
    /// Intermediate layer needed to combine the reflexivity from all the meaty parts properly
    IntermediateReflexivity,
    BrightnessCulled,
    FinalBloom,
}
impl InternalLayer {
    pub const fn render_layers(&self) -> RenderLayers {
        match self {
            Self::AmbientPixelsLit => RenderLayers::layer(20),
            Self::DetailPixelsLit => RenderLayers::layer(21),
            Self::Meat => RenderLayers::layer(22),
            Self::IntermediateBrightness => RenderLayers::layer(23),
            Self::IntermediateReflexivity => RenderLayers::layer(24),
            Self::BrightnessCulled => RenderLayers::layer(25),
            Self::FinalBloom => RenderLayers::layer(26),
        }
    }

    const fn layer_order(&self) -> LayerOrder {
        match self {
            Self::AmbientPixelsLit => LayerOrder::ApplyLight,
            Self::DetailPixelsLit => LayerOrder::ApplyLight,
            Self::Meat => LayerOrder::FlattenMeat,
            Self::IntermediateBrightness => LayerOrder::FlattenMeat,
            Self::IntermediateReflexivity => LayerOrder::FlattenMeat,
            Self::BrightnessCulled => LayerOrder::BrightnessCull,
            Self::FinalBloom => LayerOrder::PostBlur,
        }
    }

    const fn camera_mode(&self) -> LayerCameraMode {
        match self {
            Self::Meat => LayerCameraMode::Tonemapped(Tonemapping::TonyMcMapface),
            _ => LayerCameraMode::Hdr,
        }
    }

    fn target(&self) -> Handle<Image> {
        Handle::weak_from_u128(self.render_layers().bits()[0] as u128)
    }
}

/// fuckkkk I was so close to avoiding nasty hacks :(
/// I guess this one isn't that bad. These should be separate enums, and idk if a trait
/// would actually be any better
/// You know what, this is fine.
#[derive(Debug, Clone)]
pub(crate) enum MetaLayer {
    Normal(Layer),
    Internal(InternalLayer),
}
impl MetaLayer {
    fn target(&self) -> Handle<Image> {
        match self {
            Self::Normal(layer) => layer.target(),
            Self::Internal(layer) => layer.target(),
        }
    }
}

/// For some semi-hard-coded bullshit
/// SOMETHING means reflexivity or brightness
/// The idea is that for culling we:
/// - Draw ambient SOMETHING
/// - Draw a black-masked detail pixels on top of that
/// - Draw detail SOMETHING
/// - Draw a black-masked static pixels on top of that
/// - Draw static SOMETHING
/// ...
#[derive(Debug)]
pub(crate) enum BrightnessCullStage {
    Mask(Layer),
    Show(Layer),
}

pub(crate) enum LogicalLayerMode {
    /// Applies all the lighting goodness
    Lit { input: Layer, output: InternalLayer },
    /// Flattens shit into da meat
    FlattenMeat {
        stages: Vec<MetaLayer>,
        output: InternalLayer,
    },
    /// Applies brightness calc + culling goodness (also combines shit and masks with stages)
    BrightnessCull {
        brightness_stages: Vec<BrightnessCullStage>,
        brightness_intermediate: InternalLayer,
        reflexivity_stages: Vec<BrightnessCullStage>,
        reflexivity_intermediate: InternalLayer,
        input_pixels: InternalLayer,
        output: InternalLayer,
    },
    /// Blurs and shit
    GaussianBlur {
        input: InternalLayer,
        output: InternalLayer,
        passes: u32,
    },
}
fn lit(input: Layer, output: InternalLayer) -> LogicalLayerMode {
    LogicalLayerMode::Lit { input, output }
}
fn flatten_meat(stages: Vec<MetaLayer>, output: InternalLayer) -> LogicalLayerMode {
    LogicalLayerMode::FlattenMeat { stages, output }
}
fn brightness_cull(
    brightness_stages: Vec<BrightnessCullStage>,
    brightness_intermediate: InternalLayer,
    reflexivity_stages: Vec<BrightnessCullStage>,
    reflexivity_intermediate: InternalLayer,
    pixel_input: InternalLayer,
    output: InternalLayer,
) -> LogicalLayerMode {
    LogicalLayerMode::BrightnessCull {
        brightness_stages,
        brightness_intermediate,
        reflexivity_stages,
        reflexivity_intermediate,
        input_pixels: pixel_input,
        output,
    }
}
fn gaussian_blur(input: InternalLayer, output: InternalLayer, passes: u32) -> LogicalLayerMode {
    LogicalLayerMode::GaussianBlur {
        input,
        output,
        passes,
    }
}

struct LogicalLayer {
    name: String,
    mode: LogicalLayerMode,
}
impl LogicalLayer {
    fn new(name: &str, mode: LogicalLayerMode) -> Self {
        Self {
            name: name.to_string(),
            mode,
        }
    }
}
lazy_static::lazy_static! {
    static ref LOGICAL_LAYERS: Vec<LogicalLayer> = vec![
        // All of our logic
        LogicalLayer::new("AmbiencePixels", lit(Layer::AmbientPixels, InternalLayer::AmbientPixelsLit)),
        LogicalLayer::new("DetailPixels", lit(Layer::DetailPixels, InternalLayer::DetailPixelsLit)),
        LogicalLayer::new(
            "FlattenMeat",
            flatten_meat(
                vec![
                    MetaLayer::Internal(InternalLayer::AmbientPixelsLit),
                    MetaLayer::Internal(InternalLayer::DetailPixelsLit),
                    MetaLayer::Normal(Layer::StaticPixels),
                ],
                InternalLayer::Meat
            )
        ),
        LogicalLayer::new(
            "BrightnessCombine",
            brightness_cull(
                vec![
                    BrightnessCullStage::Show(Layer::AmbientBrightness),
                    BrightnessCullStage::Mask(Layer::DetailPixels),
                    BrightnessCullStage::Show(Layer::DetailBrightness),
                    BrightnessCullStage::Mask(Layer::StaticPixels),
                    BrightnessCullStage::Show(Layer::StaticBrightness),
                ],
                InternalLayer::IntermediateBrightness,
                vec![
                    BrightnessCullStage::Show(Layer::AmbientReflexivity),
                    BrightnessCullStage::Mask(Layer::DetailPixels),
                    BrightnessCullStage::Show(Layer::DetailReflexivity),
                    BrightnessCullStage::Mask(Layer::StaticPixels),
                    BrightnessCullStage::Show(Layer::StaticReflexivity),
                ],
                InternalLayer::IntermediateReflexivity,
                InternalLayer::Meat,
                InternalLayer::BrightnessCulled,
            ),
        ),
        LogicalLayer::new(
            "BrightnessBlur",
            gaussian_blur(InternalLayer::BrightnessCulled, InternalLayer::FinalBloom, 3),
        ),
    ];
}

// The final things that end up in the smush layer
struct ProjectionLayer {
    input: MetaLayer,
    use_cutout: bool,
}
impl ProjectionLayer {
    fn normal(layer: Layer) -> Self {
        Self {
            input: MetaLayer::Normal(layer),
            use_cutout: false,
        }
    }
    fn internal(layer: InternalLayer) -> Self {
        Self {
            input: MetaLayer::Internal(layer),
            use_cutout: false,
        }
    }
    fn with_use_cutout(mut self, val: bool) -> Self {
        self.use_cutout = val;
        self
    }
}
lazy_static::lazy_static! {
    static ref PROJECTION_LAYERS: Vec<ProjectionLayer> = vec![
        ProjectionLayer::normal(Layer::Bg),
        ProjectionLayer::internal(InternalLayer::Meat),
        ProjectionLayer::internal(InternalLayer::BrightnessCulled),
        ProjectionLayer::internal(InternalLayer::FinalBloom).with_use_cutout(true),
        ProjectionLayer::normal(Layer::Fg),
        ProjectionLayer::normal(Layer::Menu),
        ProjectionLayer::normal(Layer::Transition),
    ];
}

fn spawn_roots(
    mut commands: Commands,
    mut layer_root: ResMut<LayerRoot>,
    mut light_root: ResMut<LightRoot>,
    mut light_occlude_root: ResMut<LightOccludeRoot>,
) {
    layer_root.0 = commands
        .spawn((
            Name::new("LayerRoot"),
            Transform::default(),
            Visibility::Visible,
        ))
        .id();
    light_root.0 = commands
        .spawn((
            Name::new("LightRoot"),
            Transform::default(),
            Visibility::Visible,
        ))
        .set_parent(layer_root.eid())
        .id();
    light_occlude_root.0 = commands
        .spawn((
            Name::new("LightOccludeRoot"),
            Transform::default(),
            Visibility::Visible,
        ))
        .set_parent(light_root.eid())
        .id();
}

fn setup_physical_layers(
    mut commands: Commands,
    root: Res<LayerRoot>,
    layer_settings: Res<LayerSettings>,
    mut images: ResMut<Assets<Image>>,
) {
    let do_shared_setup = |commands: &mut Commands,
                           images: &mut ResMut<Assets<Image>>,
                           name: String,
                           target: Handle<Image>,
                           layer_order: LayerOrder,
                           render_layers: RenderLayers,
                           camera_mode: LayerCameraMode,
                           follow_dynamic: bool| {
        images.insert(target.id(), layer_settings.blank_screen_image());
        let mut comms = commands.spawn((
            Name::new(name),
            Transform::default(),
            Visibility::default(),
            Camera2d,
            Camera {
                order: layer_order as isize,
                target: RenderTarget::Image(target),
                clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                hdr: camera_mode.is_hdr(),
                ..default()
            },
            render_layers,
        ));
        comms.set_parent(root.eid());
        if let LayerCameraMode::Tonemapped(tonemapping) = camera_mode {
            comms.insert(tonemapping);
        }
        if follow_dynamic {
            comms.insert(FollowDynamicCamera);
        }
    };

    for layer in Layer::iter() {
        do_shared_setup(
            &mut commands,
            &mut images,
            format!("Camera_{:?}", layer),
            layer.target(),
            layer.layer_order(),
            layer.render_layers(),
            layer.camera_mode(),
            layer.layer_position() == LayerPosition::Dynamic,
        );
    }
    for internal_layer in InternalLayer::iter() {
        do_shared_setup(
            &mut commands,
            &mut images,
            format!("Camera_{:?}", internal_layer),
            internal_layer.target(),
            internal_layer.layer_order(),
            internal_layer.render_layers(),
            internal_layer.camera_mode(),
            false,
        );
    }
}

fn setup_logical_layers(
    mut commands: Commands,
    mut lit_mats: ResMut<Assets<LitMat>>,
    mut cutout_mats: ResMut<Assets<CutoutMat>>,
    mut bcull_mats: ResMut<Assets<BrightnessCullMat>>,
    mut blur_mats: ResMut<Assets<GaussianBlurMat>>,
    mut lighting: ResMut<Lighting>,
    screen_mesh: Res<ScreenMesh>,
    root: Res<LayerRoot>,
    layer_settings: Res<LayerSettings>,
    mut images: ResMut<Assets<Image>>,
) {
    for (ix, layer) in LOGICAL_LAYERS.iter().enumerate() {
        match &layer.mode {
            LogicalLayerMode::Lit { input, output } => {
                let lit_mat = LitMat::new(input.target(), Layer::Light.target(), Color::BLACK);
                let lit_mat_hand = lit_mats.add(lit_mat);
                lighting.lit_asset_map.insert(*input, lit_mat_hand.id());
                let mesh_hand = screen_mesh.0.clone();
                commands
                    .spawn((
                        Name::new(format!("LayerSprite_Lit_{:?}", layer.name)),
                        MeshMaterial2d(lit_mat_hand),
                        Mesh2d(mesh_hand),
                        Transform::from_translation(Vec3::Z * ix as f32),
                        output.render_layers(),
                    ))
                    .set_parent(root.eid());
            }
            LogicalLayerMode::FlattenMeat { stages, output } => {
                for (stage_ix, stage) in stages.iter().enumerate() {
                    commands
                        .spawn((
                            Name::new(format!("FlattenMeat_{:?}", stage)),
                            Sprite {
                                image: stage.target(),
                                custom_size: Some(layer_settings.screen_size.as_vec2()),
                                ..default()
                            },
                            Transform::from_translation(Vec3::Z * stage_ix as f32),
                            output.render_layers().clone(),
                        ))
                        .set_parent(root.eid());
                }
            }
            LogicalLayerMode::BrightnessCull {
                brightness_stages,
                brightness_intermediate,
                reflexivity_stages,
                reflexivity_intermediate,
                input_pixels,
                output,
            } => {
                let handle_stages = |name: &str,
                                     commands: &mut Commands,
                                     stages: &Vec<BrightnessCullStage>,
                                     rl: RenderLayers| {
                    for (stage_ix, stage) in stages.iter().enumerate() {
                        let (image, color) = match stage {
                            &BrightnessCullStage::Show(layer) => (layer.target(), Color::WHITE),
                            &BrightnessCullStage::Mask(layer) => (layer.target(), Color::BLACK),
                        };
                        commands
                            .spawn((
                                Name::new(format!("BrightnessCull_Combine_{:?}", name)),
                                Sprite {
                                    image,
                                    custom_size: Some(layer_settings.screen_size.as_vec2()),
                                    color,
                                    ..default()
                                },
                                Transform::from_translation(Vec3::Z * stage_ix as f32),
                                rl.clone(),
                            ))
                            .set_parent(root.eid());
                    }
                };
                handle_stages(
                    "Brightness",
                    &mut commands,
                    brightness_stages,
                    brightness_intermediate.render_layers(),
                );
                handle_stages(
                    "Reflexivity",
                    &mut commands,
                    reflexivity_stages,
                    reflexivity_intermediate.render_layers(),
                );
                let bcull_mat = BrightnessCullMat::new(
                    brightness_intermediate.target(),
                    reflexivity_intermediate.target(),
                    Layer::Light.target(),
                    input_pixels.target(),
                );
                let bcull_mat_hand = bcull_mats.add(bcull_mat);
                lighting.bcull_asset = bcull_mat_hand.id();
                commands
                    .spawn((
                        Name::new(&layer.name),
                        MeshMaterial2d(bcull_mat_hand),
                        Mesh2d(screen_mesh.0.clone()),
                        Transform::default(),
                        output.render_layers(),
                    ))
                    .set_parent(root.eid());
            }
            LogicalLayerMode::GaussianBlur {
                input,
                output,
                passes,
            } => {
                let cutout_mat = CutoutMat::new(input.target());
                let cutout_mat_hand = cutout_mats.add(cutout_mat);
                debug_assert!((LayerOrder::Blur as isize) < LayerOrder::PostBlur as isize);
                debug_assert!(*passes > 0);
                let mut available = ((LayerOrder::Blur as usize + 1)
                    ..(LayerOrder::PostBlur as usize))
                    .into_iter()
                    .collect::<VecDeque<_>>();
                let inner_passes = *passes * 2;
                commands
                    .spawn((
                        Name::new(format!("LayerSprite_BrightnessCull_{:?}", layer.name)),
                        MeshMaterial2d(cutout_mat_hand),
                        Mesh2d(screen_mesh.0.clone()),
                        RenderLayers::from_layers(&[300 + *available.front().unwrap()]),
                    ))
                    .set_parent(root.eid());
                for pass in 0..inner_passes {
                    let last_pass = pass + 1 == inner_passes;
                    let input_order = available.pop_front().unwrap();
                    let input_rl = RenderLayers::from_layers(&[300 + input_order]);
                    let output_rl = if last_pass {
                        output.render_layers()
                    } else {
                        RenderLayers::from_layers(&[300 + *available.front().unwrap()])
                    };
                    let image_hand = images.add(layer_settings.blank_screen_image());
                    commands
                        .spawn((
                            Name::new(format!("GaussianBlurPass_{pass}_Camera")),
                            Camera2d,
                            Camera {
                                order: input_order as isize,
                                target: RenderTarget::Image(image_hand.clone()),
                                clear_color: ClearColorConfig::Custom(Color::linear_rgba(
                                    0.0, 0.0, 0.0, 1.0,
                                )),
                                hdr: true,
                                ..default()
                            },
                            Transform::default(),
                            input_rl.clone(),
                        ))
                        .set_parent(root.eid());
                    let blur_mat_hand =
                        blur_mats.add(GaussianBlurMat::new(image_hand, pass % 2 == 0, 16));
                    commands
                        .spawn((
                            Name::new(format!("GaussianBlurPass_{pass}_Mesh")),
                            MeshMaterial2d(blur_mat_hand),
                            Mesh2d(screen_mesh.0.clone()),
                            Transform::default(),
                            output_rl,
                        ))
                        .set_parent(root.eid());
                }
            }
        }
    }
}

#[derive(Component)]
struct ResizeLayerToWindow;

const SMUSH_RENDER_LAYERS: RenderLayers = RenderLayers::layer(63);

fn setup_projection_layers(
    mut commands: Commands,
    layer_settings: Res<LayerSettings>,
    root: Res<LayerRoot>,
    mut cutout_mats: ResMut<Assets<CutoutMat>>,
    screen_mesh: Res<ScreenMesh>,
) {
    for (ix, layer) in PROJECTION_LAYERS.iter().enumerate() {
        if layer.use_cutout {
            commands.spawn((
                Name::new(format!("ProjectionLayer_{:?}", layer.input)),
                Mesh2d(screen_mesh.0.clone()),
                MeshMaterial2d(cutout_mats.add(CutoutMat::new(layer.input.target()))),
                Transform::from_translation(Vec3::Z * ix as f32),
                ResizeLayerToWindow,
                SMUSH_RENDER_LAYERS.clone(),
            ));
        } else {
            commands
                .spawn((
                    Name::new(format!("ProjectionLayer_{:?}", layer.input)),
                    Sprite {
                        image: layer.input.target(),
                        custom_size: Some(layer_settings.screen_size.as_vec2()),
                        ..default()
                    },
                    Transform::from_translation(Vec3::Z * ix as f32),
                    ResizeLayerToWindow,
                    SMUSH_RENDER_LAYERS.clone(),
                ))
                .set_parent(root.eid());
        }
    }
}

fn setup_smush_layer(mut commands: Commands, root: Res<LayerRoot>) {
    commands
        .spawn((
            Name::new("SmushCamera"),
            Camera2d,
            Camera {
                order: LayerOrder::Smush as isize,
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            Tonemapping::TonyMcMapface,
            SMUSH_RENDER_LAYERS.clone(),
        ))
        .set_parent(root.eid());
}

fn resize_layers_as_needed(
    mut events: EventReader<WindowResized>,
    mut quad_trans: Query<&mut Transform, With<ResizeLayerToWindow>>,
    layer_settings: Res<LayerSettings>,
) {
    let Some(event) = events.read().last() else {
        return;
    };

    let effective_window = layer_settings.screen_size.as_vec2();

    // Mult is smallest to fill either vertically or horizontally
    // A.k.a don't cut anything off
    let width_mult = event.width / effective_window.x;
    let height_mult = event.height / effective_window.y;
    let mult = width_mult.min(height_mult);

    // Then update the layering quads
    for mut tran in &mut quad_trans {
        tran.scale = (Vec2::ONE * mult).extend(1.0);
    }
}

pub(super) fn register_layer(app: &mut App, screen_size: UVec2) {
    app.insert_resource(LayerRoot::default());
    app.insert_resource(LightRoot::default());
    app.insert_resource(LightOccludeRoot::default());
    app.insert_resource(LayerSettings { screen_size });

    app.add_systems(
        Startup,
        (
            spawn_roots,
            setup_screen_mesh,
            setup_physical_layers,
            setup_logical_layers,
            setup_projection_layers,
            setup_smush_layer,
        )
            .chain(),
    );
    app.add_systems(Update, resize_layers_as_needed);
}
