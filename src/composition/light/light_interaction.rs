use bevy::prelude::*;

use crate::glue::Fx;

use super::{light_alloc::LightClaim, light_man::LightAnim};

#[derive(Component)]
#[component(on_remove = on_remove_source)]
pub(crate) struct LightSource {
    pub(crate) claim: LightClaim,
    pub(crate) radius: Option<Fx>,
}
impl LightSource {
    pub(crate) fn new(claim: LightClaim) -> Self {
        Self {
            claim,
            radius: None,
        }
    }
}
fn on_remove_source(
    mut world: bevy::ecs::world::DeferredWorld,
    eid: Entity,
    _: bevy::ecs::component::ComponentId,
) {
    let claim = world.get::<LightSource>(eid).unwrap().claim.clone();
    claim.free(&mut world);
}
pub(crate) fn remove_light_source(
    mut world: bevy::ecs::world::DeferredWorld,
    eid: Entity,
    _: bevy::ecs::component::ComponentId,
) {
    world.commands().get_entity(eid).map(|mut inner| {
        inner.remove::<LightSource>();
    });
}

pub(super) fn register_light_interaction<Anim: LightAnim>(_app: &mut App) {}
