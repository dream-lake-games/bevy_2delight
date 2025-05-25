use std::collections::VecDeque;

use bevy::prelude::*;

pub const BASE_LIGHT_RENDER_LAYER: usize = 100;
pub const MAX_NUM_LIGHTS: usize = 256;

use bevy::render::camera::RenderTarget;
use bevy::render::view::RenderLayers;
use rand::{thread_rng, Rng};

use crate::composition::camera::FollowDynamicCamera;
use crate::composition::layer::{LayerOrder, LayerSettings, LightRoot};
use crate::prelude::Layer;

use crate::composition::{layer::ScreenMesh, mats::cutout_mat::CutoutMat};

/// Facilitates assigning lights to different render layers so that they don't
/// interfere with each other
#[derive(Resource, Clone, Debug)]
pub(super) struct LightAllocer {
    unused_render_layers: VecDeque<RenderLayers>,
}
impl Default for LightAllocer {
    fn default() -> Self {
        let mut unused_render_layers = VecDeque::new();
        for rl in BASE_LIGHT_RENDER_LAYER..(BASE_LIGHT_RENDER_LAYER + MAX_NUM_LIGHTS) {
            unused_render_layers.push_back(RenderLayers::from_layers(&[rl]));
        }
        Self {
            unused_render_layers,
        }
    }
}
impl LightAllocer {
    /// Claims a render layer. If we're out, we'll get a dummy render layer that won't show up :(
    pub(super) fn alloc(&mut self) -> RenderLayers {
        self.unused_render_layers
            .pop_front()
            .unwrap_or(Layer::Dummy.render_layers())
    }
    /// Returns a render layer back to the usable queue
    pub fn free(&mut self, rl: &RenderLayers) {
        if rl.bits() != Layer::Dummy.render_layers().bits() {
            self.unused_render_layers.push_back(rl.clone());
        }
    }
}

/// Represents a claim to the resources needed for a light source
#[derive(Clone, Debug, Reflect)]
pub(super) struct LightClaim {
    /// The render layer that this light has sole control over
    pub(super) rl: RenderLayers,
    /// The entity with the camera serving this light source
    pub(super) camera_eid: Entity,
    /// The final light mesh produced by this source, to be aggregated with all other lights in the light layer
    pub(super) agg_mesh_eid: Entity,
}
impl Default for LightClaim {
    fn default() -> Self {
        Self {
            rl: Layer::Dummy.render_layers(),
            camera_eid: Entity::PLACEHOLDER,
            agg_mesh_eid: Entity::PLACEHOLDER,
        }
    }
}
impl LightClaim {
    pub(super) fn alloc(world: &mut bevy::ecs::world::DeferredWorld) -> Self {
        let layer_settings = world.resource::<LayerSettings>();
        let image = layer_settings.blank_screen_image();
        let light_root = world.resource::<LightRoot>().eid();

        // Claim a render layer
        let rl = world.resource_mut::<LightAllocer>().alloc();

        // Spawn a camera that is essentially scratch drawing space for light + cutouts
        let mut images = world.resource_mut::<Assets<Image>>();
        let image_hand = images.add(image);
        let camera_eid = world
            .commands()
            .spawn((
                Name::new("LightCamera"),
                Camera2d,
                Camera {
                    order: LayerOrder::PreLight as isize,
                    target: RenderTarget::Image(image_hand.clone().into()),
                    clear_color: ClearColorConfig::Custom(Color::linear_rgba(0.0, 0.0, 0.0, 0.0)),
                    hdr: true,
                    ..default()
                },
                rl.clone(),
                FollowDynamicCamera,
            ))
            .insert(ChildOf(light_root))
            .id();

        // Spawn the mesh which will apply proper cutout shader and chuck output into aggregate light layer
        let mat = world
            .resource_mut::<Assets<CutoutMat>>()
            .add(CutoutMat::new(image_hand));
        let mesh = world.resource::<ScreenMesh>().0.clone();
        let agg_mesh_eid = world
            .commands()
            .spawn((
                Name::new("LightActualMesh"),
                Transform::from_translation(Vec3::Z * thread_rng().gen_range(0.0..1.0)),
                Visibility::Inherited,
                Mesh2d(mesh),
                MeshMaterial2d(mat),
                Layer::Light.render_layers(),
            ))
            .insert(ChildOf(light_root))
            .id();

        LightClaim {
            rl,
            camera_eid,
            agg_mesh_eid,
        }
    }
    pub(super) fn free(&self, world: &mut bevy::ecs::world::DeferredWorld) {
        world.resource_mut::<LightAllocer>().free(&self.rl);
        if let Ok(mut comms) = world.commands().get_entity(self.camera_eid) {
            comms.despawn();
        }
        if let Ok(mut comms) = world.commands().get_entity(self.agg_mesh_eid) {
            comms.despawn();
        }
    }
}
