use bevy::{
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

use super::{camera::FollowDynamicCamera, lit_mat::LitMat, prelude::Lighting};

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
                format: TextureFormat::bevy_default(),
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

pub(super) enum LayerOrder {
    /// These are all things that basically are snapshotting the world.
    PreLight = 1,
    /// Requires all the individual light layers to be rendered first.
    Light = 2,
    /// Processing that requires the lighting info
    PostLight = 3,
    /// Requires light info, lit pixels, and brightness/reflexivity info.
    /// Will figure out how bright each pixel is and cull to only what should bloom-blur.
    BrightnessCulling = 4,
    /// Blurs the bloom stuff horizontally
    BlurHorizontal = 5,
    /// Then blurs it vertically
    BlurVertical = 6,
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
    Static,
    Fg,
    Menu,
    Transition,
}
impl Layer {
    pub const fn render_layers(&self) -> RenderLayers {
        match self {
            // We make static 0 (the default) so if we ever forget to attach render layers
            // they'll show up here. Easier to debug than just having the thing not appear
            Self::Static => RenderLayers::layer(0),
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
            Self::Fg => RenderLayers::layer(10),
            Self::Menu => RenderLayers::layer(11),
            Self::Transition => RenderLayers::layer(12),
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
            | Self::Static => LayerPosition::Dynamic,
            // NOTE: Light is included (indirectly) here in fixed because each of the underlying light cameras
            //       will follow the camera, and then render back at the origin
            _ => LayerPosition::Fixed,
        }
    }

    fn target(&self) -> Handle<Image> {
        Handle::weak_from_u128(self.render_layers().bits()[0] as u128)
    }
}

#[derive(Clone, Copy, Debug, Reflect, PartialEq, Eq, EnumIter, std::hash::Hash)]
enum InternalLayer {
    LitAmbientPixels,
    LitDetailPixels,
    BrightAmbientCulled,
    BrightAmbientIntermediate,
    BrightAmbientFinal,
    BrightDetailCulled,
    BrightDetailIntermediate,
    BrightDetailFinal,
}
impl InternalLayer {
    pub const fn render_layers(&self) -> RenderLayers {
        match self {
            Self::LitAmbientPixels => RenderLayers::layer(20),
            Self::LitDetailPixels => RenderLayers::layer(21),
            Self::BrightAmbientCulled => RenderLayers::layer(22),
            Self::BrightAmbientIntermediate => RenderLayers::layer(23),
            Self::BrightAmbientFinal => RenderLayers::layer(24),
            Self::BrightDetailCulled => RenderLayers::layer(25),
            Self::BrightDetailIntermediate => RenderLayers::layer(26),
            Self::BrightDetailFinal => RenderLayers::layer(26),
        }
    }

    const fn layer_order(&self) -> LayerOrder {
        match self {
            Self::LitAmbientPixels => LayerOrder::PostLight,
            Self::LitDetailPixels => LayerOrder::PostLight,
            Self::BrightAmbientCulled => LayerOrder::BrightnessCulling,
            Self::BrightAmbientIntermediate => LayerOrder::BlurHorizontal,
            Self::BrightAmbientFinal => LayerOrder::BlurVertical,
            Self::BrightDetailCulled => LayerOrder::BrightnessCulling,
            Self::BrightDetailIntermediate => LayerOrder::BlurHorizontal,
            Self::BrightDetailFinal => LayerOrder::BlurVertical,
        }
    }

    fn target(&self) -> Handle<Image> {
        Handle::weak_from_u128(self.render_layers().bits()[0] as u128)
    }
}

pub(crate) enum LogicalLayerMode {
    /// Will simply take the produced layer input handle and render to a sprite in the final smush layer
    Simple { input: Layer },
    /// Same as above but works on internal layers
    SimpleInternal { input: InternalLayer },
    /// Applies all the lighting goodness
    Lit { input: Layer, output: InternalLayer },
    /// Applies brightness calc + culling goodness
    Brightness {
        brightness: Layer,
        reflexivity: Layer,
        lit_input: InternalLayer,
        output: InternalLayer,
    },
    /// Blurs horizontally and writes to an intermediate output
    BlurHorizontal {
        input: InternalLayer,
        output: InternalLayer,
    },
    /// Blurs vertical and renders to a sprite in the final smush layer
    BlurVertical {
        input: InternalLayer,
        output: InternalLayer,
    },
}
fn simple(input: Layer) -> LogicalLayerMode {
    LogicalLayerMode::Simple { input }
}
fn simple_internal(input: InternalLayer) -> LogicalLayerMode {
    LogicalLayerMode::SimpleInternal { input }
}
fn lit(input: Layer, output: InternalLayer) -> LogicalLayerMode {
    LogicalLayerMode::Lit { input, output }
}
fn brightness_culling(
    brightness: Layer,
    reflexivity: Layer,
    lit_input: InternalLayer,
    output: InternalLayer,
) -> LogicalLayerMode {
    LogicalLayerMode::Brightness {
        brightness,
        reflexivity,
        lit_input,
        output,
    }
}
fn blur_horizontal(input: InternalLayer, output: InternalLayer) -> LogicalLayerMode {
    LogicalLayerMode::BlurHorizontal { input, output }
}
fn blur_vertical(input: InternalLayer, output: InternalLayer) -> LogicalLayerMode {
    LogicalLayerMode::BlurVertical { input, output }
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
        LogicalLayer::new("AmbiencePixels", lit(Layer::AmbientPixels, InternalLayer::LitAmbientPixels)),
        LogicalLayer::new(
            "AmbientBrightnessCulling",
            brightness_culling(
                Layer::AmbientBrightness,
                Layer::AmbientReflexivity,
                InternalLayer::LitAmbientPixels,
                InternalLayer::BrightAmbientCulled
            )
        ),
        LogicalLayer::new("AmbientBrightnessBlurHorizontal", blur_horizontal(InternalLayer::BrightAmbientCulled, InternalLayer::BrightAmbientIntermediate)),
        LogicalLayer::new("AmbientBrightnessBlurVertical",  blur_vertical(InternalLayer::BrightAmbientIntermediate, InternalLayer::BrightAmbientFinal)),
        LogicalLayer::new("DetailPixels", lit(Layer::DetailPixels, InternalLayer::LitDetailPixels)),
        LogicalLayer::new(
            "DetailBrightnessCulling",
            brightness_culling(
                Layer::DetailBrightness,
                Layer::DetailReflexivity,
                InternalLayer::LitDetailPixels,
                InternalLayer::BrightDetailCulled,
            )
        ),
        LogicalLayer::new("DetailBrightnessBlurHorizontal", blur_horizontal(InternalLayer::BrightDetailCulled, InternalLayer::BrightDetailIntermediate)),
        LogicalLayer::new("DetailBrightnessBlurVertical", blur_vertical(InternalLayer::BrightDetailIntermediate, InternalLayer::BrightDetailFinal)),
        // All the shit that actually produces shit to the smush layer, in order
        LogicalLayer::new("Bg", simple(Layer::Bg)),
        LogicalLayer::new("LitAmbiencePixels", simple_internal(InternalLayer::LitAmbientPixels)),
        LogicalLayer::new("LitDetailPixels", simple_internal(InternalLayer::LitDetailPixels)),
        LogicalLayer::new("Static", simple(Layer::Static)),
        LogicalLayer::new("AmbientBrightnessFinal", simple_internal(InternalLayer::BrightAmbientFinal)),
        LogicalLayer::new("DetailBrightnessFinal", simple_internal(InternalLayer::BrightDetailFinal)),
        LogicalLayer::new("Fg", simple(Layer::Fg)),
        LogicalLayer::new("Menu", simple(Layer::Menu)),
        LogicalLayer::new("Transition", simple(Layer::Transition)),
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
                ..default()
            },
            render_layers,
        ));
        comms.set_parent(root.eid());
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
            false,
        );
    }
}

#[derive(Component)]
struct ResizeLayerToWindow;

const SMUSH_RENDER_ORDER: isize = LayerOrder::PostLight as isize + 1;
const SMUSH_RENDER_LAYERS: RenderLayers = RenderLayers::layer(36);

fn setup_logical_layers(
    mut commands: Commands,
    root: Res<LayerRoot>,
    layer_settings: Res<LayerSettings>,
    mut lit_mats: ResMut<Assets<LitMat>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut lighting: ResMut<Lighting>,
) {
    // Helper function for setting up a simple layer shared for normal and internal varieties
    let do_simple_setup =
        |commands: &mut Commands, name: String, input_target: Handle<Image>, zix: usize| {
            commands
                .spawn((
                    Name::new(name),
                    Sprite {
                        image: input_target,
                        custom_size: Some(layer_settings.screen_size.as_vec2()),
                        ..default()
                    },
                    Transform::from_translation(Vec3::Z * zix as f32),
                    ResizeLayerToWindow,
                    SMUSH_RENDER_LAYERS.clone(),
                ))
                .set_parent(root.eid());
        };

    for (ix, layer) in LOGICAL_LAYERS.iter().enumerate() {
        match &layer.mode {
            LogicalLayerMode::Simple { input } => {
                do_simple_setup(
                    &mut commands,
                    format!("LayerSprite_Simple_{:?}", layer.name),
                    input.target(),
                    ix,
                );
            }
            LogicalLayerMode::SimpleInternal { input } => {
                do_simple_setup(
                    &mut commands,
                    format!("LayerSprite_SimpleInternal_{:?}", layer.name),
                    input.target(),
                    ix,
                );
            }
            LogicalLayerMode::Lit { input, output } => {
                let lit_mat = LitMat::new(input.target(), Layer::Light.target(), Color::BLACK);
                let lit_mat_hand = lit_mats.add(lit_mat);
                let mesh = Mesh::from(Rectangle::new(
                    layer_settings.screen_size.x as f32,
                    layer_settings.screen_size.y as f32,
                ));
                let mesh_hand = meshes.add(mesh);
                let eid = commands
                    .spawn((
                        Name::new(format!("LayerSprite_Lit_{:?}", layer.name)),
                        MeshMaterial2d(lit_mat_hand),
                        Mesh2d(mesh_hand),
                        Transform::from_translation(Vec3::Z * ix as f32),
                        // ResizeLayerToWindow,
                        output.render_layers(),
                    ))
                    .set_parent(root.eid())
                    .id();
                lighting.layer_eid_map.insert(*input, eid);
            }
            _ => {
                // Do nothing for now...
            }
        }
    }
}

fn setup_smush_layer(mut commands: Commands, root: Res<LayerRoot>) {
    commands
        .spawn((
            Name::new("SmushCamera"),
            Camera2d,
            Camera {
                order: SMUSH_RENDER_ORDER,
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
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
            setup_physical_layers,
            setup_logical_layers,
            setup_smush_layer,
        )
            .chain(),
    );
    app.add_systems(Update, resize_layers_as_needed);
}
