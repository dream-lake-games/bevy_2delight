use std::collections::VecDeque;

use bevy::prelude::*;

pub const BASE_LIGHT_RENDER_LAYER: usize = 100;
pub const MAX_NUM_LIGHTS: usize = 256;

use bevy::render::camera::RenderTarget;
use bevy::render::view::RenderLayers;

use crate::layer::camera::FollowDynamicCamera;
use crate::layer::consts::{COLOR_NONE, ZIX_MAX, ZIX_MIN};
use crate::layer::layer::layer_defns::{LightLayer, PRE_LIGHT_RENDER_ORDER};
use crate::layer::layer_utils::blank_screen_image;
use crate::layer::plugin::LayerRes;
use crate::layer::DUMMY_LAYER_USIZE;
use crate::prelude::Layer;

use super::light_mat::LightCutoutMat;

/// Facilitates assigning lights to different render layers so that they don't
/// interfere with each other
#[derive(Resource, Clone, Debug, Reflect)]
pub(super) struct LightAllocer {
    unused_render_layers: VecDeque<usize>,
}
impl Default for LightAllocer {
    fn default() -> Self {
        let mut unused_render_layers = VecDeque::new();
        for rl in BASE_LIGHT_RENDER_LAYER..(BASE_LIGHT_RENDER_LAYER + MAX_NUM_LIGHTS) {
            unused_render_layers.push_back(rl);
        }
        Self {
            unused_render_layers,
        }
    }
}
impl LightAllocer {
    /// Claims a render layer. If we're out, we'll get a dummy render layer that won't show up :(
    pub(super) fn alloc(&mut self) -> usize {
        self.unused_render_layers
            .pop_front()
            .unwrap_or(DUMMY_LAYER_USIZE)
    }
    /// Returns a render layer back to the usable queue
    pub fn free(&mut self, rl: usize) {
        if rl != DUMMY_LAYER_USIZE {
            self.unused_render_layers.push_back(rl);
        }
    }
}

/// Represents a claim to the resources needed for a light source
#[derive(Clone)]
pub(super) struct LightClaim {
    /// The render layer that this light has sole control over
    pub(super) rl_usize: usize,
    /// The entity with the camera serving this light source
    pub(super) camera_eid: Entity,
    /// The final light mesh produced by this source, to be aggregated with all other lights in the light layer
    pub(super) agg_mesh_eid: Entity,
}
impl Default for LightClaim {
    fn default() -> Self {
        Self {
            rl_usize: DUMMY_LAYER_USIZE,
            camera_eid: Entity::PLACEHOLDER,
            agg_mesh_eid: Entity::PLACEHOLDER,
        }
    }
}
impl LightClaim {
    pub(super) fn alloc(world: &mut bevy::ecs::world::DeferredWorld) -> Self {
        let res = world.resource::<LayerRes>().clone();

        // Claim a render layer
        let rl_usize = world.resource_mut::<LightAllocer>().alloc();

        // Spawn a camera that is essentially scratch drawing space for light + cutouts
        let image = blank_screen_image(&res, false);
        let mut images = world.resource_mut::<Assets<Image>>();
        let image_hand = images.add(image);
        let camera_eid = world
            .commands()
            .spawn((
                Name::new("LightCamera"),
                Camera2d,
                Camera {
                    order: PRE_LIGHT_RENDER_ORDER as isize,
                    target: RenderTarget::Image(image_hand.clone()),
                    clear_color: ClearColorConfig::Custom(COLOR_NONE),
                    ..default()
                },
                OrthographicProjection {
                    near: ZIX_MIN,
                    far: ZIX_MAX,
                    scale: 1.0,
                    ..OrthographicProjection::default_2d()
                },
                RenderLayers::from_layers(&[rl_usize]),
                FollowDynamicCamera,
            ))
            .set_parent(res.root_eid())
            .id();

        // Spawn the mesh which will apply proper cutout shader and chuck output into aggregate light layer
        let mesh = Mesh::from(Rectangle::new(
            res.screen_size.x as f32,
            res.screen_size.y as f32,
        ));
        let mesh: Mesh2d = world.resource_mut::<Assets<Mesh>>().add(mesh).into();
        let mat: MeshMaterial2d<LightCutoutMat> = world
            .resource_mut::<Assets<LightCutoutMat>>()
            .add(LightCutoutMat::new(image_hand.clone()))
            .into();
        let agg_mesh_eid = world
            .commands()
            .spawn((
                Name::new("LightActualMesh"),
                mesh,
                mat,
                Transform::default(),
                Visibility::Inherited,
                LightLayer::RENDER_LAYERS,
            ))
            .set_parent(res.root_eid())
            .id();

        LightClaim {
            rl_usize,
            camera_eid,
            agg_mesh_eid,
        }
    }
    pub(super) fn free(&self, world: &mut bevy::ecs::world::DeferredWorld) {
        world.resource_mut::<LightAllocer>().free(self.rl_usize);
        if let Some(comms) = world.commands().get_entity(self.camera_eid) {
            comms.despawn_recursive();
        }
        if let Some(comms) = world.commands().get_entity(self.agg_mesh_eid) {
            comms.despawn_recursive();
        }
    }
}
