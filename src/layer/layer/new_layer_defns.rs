use bevy::{
    prelude::*,
    render::{camera::RenderTarget, view::RenderLayers},
    window::WindowResized,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Resource)]
pub(crate) struct LayerRoot(Entity);
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
pub(crate) struct LayerSettings {
    pub(crate) screen_size: UVec2,
}

#[derive(Resource)]
pub struct Lighting {
    pub base_ambient: Color,
    pub base_detail: Color,
    pub brightness_cutoff: f32,
}
impl Default for Lighting {
    fn default() -> Self {
        Self {
            base_ambient: Color::linear_rgb(0.7, 0.7, 0.7),
            base_detail: Color::linear_rgb(0.3, 0.3, 0.3),
            brightness_cutoff: 1.0,
        }
    }
}

pub(crate) enum LayerOrder {
    /// Most layers. These are all things that basically are snapshotting the world.
    PreLight = 1,
    /// Requires all the individual light layers to be rendered first.
    Light = 2,
    /// Advanced processing (probably lit layers) that requires the Light layer to be settled
    PostLight = 3,
}

#[derive(PartialEq, Eq)]
pub(crate) enum LayerPosition {
    /// The camera rendering this is always at the origin
    Fixed,
    /// The camera rendering this follows the DynamicCamera, if it exists
    Dynamic,
}

#[derive(Clone, Copy, Debug, Reflect, PartialEq, Eq, EnumIter)]
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
            // they'll show up here.
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
            | Self::Light
            | Self::AmbientPixels
            | Self::AmbientBrightness
            | Self::AmbientReflexivity
            | Self::DetailPixels
            | Self::DetailBrightness
            | Self::DetailReflexivity
            | Self::Static => LayerPosition::Dynamic,
            _ => LayerPosition::Fixed,
        }
    }

    fn target(&self) -> Handle<Image> {
        Handle::weak_from_u128(self.render_layers().bits()[0] as u128)
    }
}

pub(crate) enum LogicalLayerMode {
    /// Will simply take the produced input handle and render to a sprite in
    /// the final smush layer
    Simple { input: Layer },
    /// Applies all the lighting goodness,
    Lit {
        pixels: Layer,
        brightness: Layer,
        reflexivity: Layer,
    },
}
fn simple(input: Layer) -> LogicalLayerMode {
    LogicalLayerMode::Simple { input }
}
fn lit(pixels: Layer, brightness: Layer, reflexivitiy: Layer) -> LogicalLayerMode {
    LogicalLayerMode::Lit {
        pixels,
        brightness,
        reflexivity: reflexivitiy,
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
        LogicalLayer::new("Bg", simple(Layer::Bg)),
        LogicalLayer::new("Ambience", lit(Layer::AmbientPixels, Layer::AmbientBrightness, Layer::AmbientReflexivity)),
        LogicalLayer::new("Detail", lit(Layer::DetailPixels, Layer::DetailBrightness, Layer::DetailReflexivity)),
        LogicalLayer::new("Static", simple(Layer::Static)),
        LogicalLayer::new("Fg", simple(Layer::Fg)),
        LogicalLayer::new("Menu", simple(Layer::Menu)),
        LogicalLayer::new("Transition", simple(Layer::Transition)),
    ];
}

#[derive(Component)]
struct FollowDynamicCamera;

fn spawn_root(mut commands: Commands, mut root: ResMut<LayerRoot>) {
    root.0 = commands
        .spawn((
            Name::new("LayerRoot"),
            Transform::default(),
            Visibility::Visible,
        ))
        .id();
}

fn setup_physical_layers(mut commands: Commands, root: Res<LayerRoot>) {
    for layer in Layer::iter() {
        let mut comms = commands.spawn((
            Name::new(format!("Camera_{:?}", layer)),
            Transform::default(),
            Visibility::default(),
            Camera2d,
            Camera {
                order: layer.layer_order() as isize,
                target: RenderTarget::Image(layer.target()),
                ..default()
            },
            layer.render_layers(),
        ));
        comms.set_parent(root.eid());
        if layer.layer_position() == LayerPosition::Dynamic {
            comms.insert(FollowDynamicCamera);
        }
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
) {
    for (ix, layer) in LOGICAL_LAYERS.iter().enumerate() {
        match &layer.mode {
            LogicalLayerMode::Simple { input } => {
                commands
                    .spawn((
                        Name::new(format!("LayerSprite_Simple_{:?}", layer.name)),
                        Sprite {
                            image: input.target(),
                            custom_size: Some(layer_settings.screen_size.as_vec2()),
                            ..default()
                        },
                        Transform::from_translation(Vec3::Z * ix as f32),
                        ResizeLayerToWindow,
                    ))
                    .set_parent(root.eid());
            }
            LogicalLayerMode::Lit {
                pixels,
                brightness,
                reflexivity,
            } => {}
        }
    }
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

pub(crate) struct LayerPlugin {
    layer_settings: LayerSettings,
}
impl Plugin for LayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_root, setup_physical_layers, setup_logical_layers),
        );
    }
}
