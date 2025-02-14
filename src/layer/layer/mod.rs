use bevy::{
    prelude::*,
    render::{camera::RenderTarget, view::RenderLayers},
    window::WindowResized,
};
use layer_defns::{
    BgLayer, FgLayer, LightLayer, MainAmbienceLayer, MainDetailLayer, MainStaticLayer, MenuLayer,
    OverlayLayer, SmushLayer, TransitionLayer,
};

use crate::layer::{
    camera::FollowDynamicCamera,
    consts::{COLOR_NONE, ZIX_MAX, ZIX_MIN},
    layer_utils::blank_screen_image,
    light::light_mat::LightApplyMat,
    plugin::LayerRes,
};

pub(crate) mod layer_defns;

/// Key information defining a layer
pub trait Layer: std::fmt::Debug + Default {
    /// A unique key to be given to this layer
    #[doc(hidden)]
    const _KEY: u32;
    /// Which render layers to attach to things in this layer
    const RENDER_LAYERS: RenderLayers = RenderLayers::layer(Self::_KEY as usize);
}

/// How this layer should react to the camera moving
pub(crate) enum LayerPositionMode {
    /// The camera's position will always be the center of the screen
    /// NOTE: Used for the actual world with game objects
    Follow,
    /// (0.0, 0.0) will always be the center of the screen
    /// NOTE: Used for basically everything else (bg/fg so parallax easy, menu, etc.)
    Fixed,
}

/// What shader should be applied to the output of this layer
pub(crate) enum LayerOutputMode {
    /// Just render to target
    None,
    /// Render to target, then put that target on a sprite in the output RenderLayers
    Unlit { output_rl: RenderLayers },
    /// Render to target, then put that target on a lit material in the output RenderLayers
    Lit {
        base_color: Color,
        output_rl: RenderLayers,
    },
}

/// Marks that the final sprite for this layer needs resizing based on the size of the window
#[derive(Component)]
pub(crate) struct LayerNeedsResizing;

/// Key information about a layer only to be used in this crate
pub(crate) trait LayerInternal: Layer {
    /// A handle to the underlying
    const TARGET: Handle<Image> = Handle::weak_from_u128(Self::_KEY as u128);
    /// In what order should this layer get rendered?
    /// EXAMPLE: Important for things like making sure that certain layers rendered AFTER lighting
    const RENDER_ORDER: u32;
    /// Is this a screen size layer, or an overlay size level
    const IS_OVERLAY: bool;
    /// At the end of the day, all these layers render to images, which we stack on top of each other.
    /// What should be the ZIX of this image?
    const ZIX: u32;
    /// How the layer interacts with our moving dynamic camera
    const LAYER_POSITION_MODE: LayerPositionMode;
    /// Defines the shader to use on the output of this layer
    const LAYER_OUTPUT_MODE: LayerOutputMode;
    /// Potentially a custom clear color
    const CLEAR_COLOR: Color = COLOR_NONE;

    fn setup(
        commands: &mut Commands,
        res: &LayerRes,
        images: &mut ResMut<Assets<Image>>,
        meshes: &mut ResMut<Assets<Mesh>>,
        light_apply_mats: &mut ResMut<Assets<LightApplyMat>>,
    ) {
        let is_smush = Self::_KEY == SmushLayer::_KEY;
        // Render to a target
        let render_target = if is_smush {
            RenderTarget::default()
        } else {
            let image = blank_screen_image(res, Self::IS_OVERLAY);
            images.insert(Self::TARGET.id(), image);
            RenderTarget::Image(Self::TARGET)
        };
        let mut comms = commands.spawn((
            Name::new(format!("LayerTargetCamera_{:?}", Self::default())),
            Camera2d,
            Camera {
                order: Self::RENDER_ORDER as isize,
                target: render_target,
                clear_color: ClearColorConfig::Custom(Self::CLEAR_COLOR),
                ..default()
            },
            OrthographicProjection {
                near: ZIX_MIN,
                far: ZIX_MAX,
                scale: if Self::IS_OVERLAY && !is_smush {
                    1.0 / res.overlay_growth as f32
                } else {
                    1.0
                },
                ..OrthographicProjection::default_2d()
            },
            Self::RENDER_LAYERS,
        ));
        comms.set_parent(res.root_eid());
        if matches!(Self::LAYER_POSITION_MODE, LayerPositionMode::Follow) {
            comms.insert(FollowDynamicCamera);
        }

        // Maybe do other stuff
        match Self::LAYER_OUTPUT_MODE {
            LayerOutputMode::None => (),
            LayerOutputMode::Unlit { output_rl } => {
                // TODO: Handle lit material differently
                commands
                    .spawn((
                        Name::new(format!("LayerUnlitOutput_{:?}", Self::default())),
                        output_rl,
                        Transform::from_translation(Vec3::Z * Self::ZIX as f32),
                        Visibility::default(),
                        Sprite {
                            image: Self::TARGET,
                            custom_size: Some((res.screen_size * res.overlay_growth).as_vec2()),
                            ..default()
                        },
                        LayerNeedsResizing,
                    ))
                    .set_parent(res.root_eid());
            }
            LayerOutputMode::Lit {
                output_rl,
                base_color,
            } => {
                let custom_size = (res.screen_size * res.overlay_growth).as_vec2();
                let mesh = Mesh::from(Rectangle::new(custom_size.x, custom_size.y));
                let mesh_hand = meshes.add(mesh);
                let mat = LightApplyMat::new(Self::TARGET, LightLayer::TARGET, base_color);
                let mat_hand = light_apply_mats.add(mat);
                commands
                    .spawn((
                        Name::new(format!("LayerLitOutput_{:?}", Self::default())),
                        output_rl,
                        Transform::from_translation(Vec3::Z * Self::ZIX as f32),
                        Visibility::default(),
                        Mesh2d(mesh_hand),
                        MeshMaterial2d(mat_hand),
                        LayerNeedsResizing,
                    ))
                    .set_parent(res.root_eid());
            }
        }
    }
}

pub(crate) fn resize_layers_as_needed(
    mut events: EventReader<WindowResized>,
    mut quad_trans: Query<&mut Transform, With<LayerNeedsResizing>>,
    layers_res: Res<LayerRes>,
) {
    let Some(event) = events.read().last() else {
        return;
    };

    let effective_window = (layers_res.screen_size * layers_res.overlay_growth).as_vec2();

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

pub(crate) fn setup_all_layers(
    mut commands: Commands,
    layers_res: Res<LayerRes>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut light_apply_mats: ResMut<Assets<LightApplyMat>>,
) {
    macro_rules! setup_layers_helper {
        ($($layer:ty$(,)?)*) => {
            $(
                <$layer>::setup(
                    &mut commands,
                    &layers_res,
                    &mut images,
                    &mut meshes,
                    &mut light_apply_mats,
                );
            )*
        };
    }
    setup_layers_helper!(
        LightLayer,
        BgLayer,
        MainAmbienceLayer,
        MainDetailLayer,
        MainStaticLayer,
        FgLayer,
        OverlayLayer,
        MenuLayer,
        TransitionLayer,
        SmushLayer
    );
}
