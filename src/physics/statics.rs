use bevy::prelude::*;

use crate::{
    physics::{colls::CollKey, hbox::HBox, pos::Pos},
    prelude::OccludeLight,
};

use super::spat_hash::{on_remove_spat_hash, SpatHash, SpatHashStaticTx};

#[derive(Clone, Copy, Debug, Reflect, PartialEq, Eq, std::hash::Hash)]
pub enum StaticRxKind {
    /// Pushes the rx ctrl out of tx comps, sets vel to zero along plane of intersection
    Default,
    /// Observes collisions but does nothing to respond
    Observe,
}
#[derive(Clone, Copy, Debug, Reflect, PartialEq, Eq, std::hash::Hash)]
pub enum StaticTxKind {
    /// Standard solid thing. Stops stuff
    Solid,
    /// Will let you go up, but not down
    PassUp,
}

pub(crate) struct StaticRxComp {
    pub(crate) kind: StaticRxKind,
    pub(crate) hbox: HBox,
}
#[derive(Component)]
pub struct StaticRx {
    pub(crate) comps: Vec<StaticRxComp>,
    pub coll_keys: Vec<CollKey>,
}
impl StaticRx {
    pub fn single(kind: StaticRxKind, hbox: HBox) -> Self {
        Self::new(vec![(kind, hbox)])
    }
    pub fn new<I: IntoIterator<Item = (StaticRxKind, HBox)>>(data: I) -> Self {
        Self {
            comps: data
                .into_iter()
                .map(|(kind, hbox)| StaticRxComp { kind, hbox })
                .collect(),
            coll_keys: vec![],
        }
    }
    pub fn get_thboxes(&self, pos: Pos) -> Vec<HBox> {
        self.comps
            .iter()
            .map(|comp| comp.hbox.translated(pos.as_fvec2()))
            .collect()
    }
}

pub(crate) struct StaticTxComp {
    pub(crate) kind: StaticTxKind,
    pub(crate) hbox: HBox,
}
#[derive(Component)]
#[component(on_add = on_add_static_tx)]
#[component(on_remove = on_remove_static_tx)]
pub struct StaticTx {
    pub(crate) comps: Vec<StaticTxComp>,
    pub coll_keys: Vec<CollKey>,
}
fn on_add_static_tx(
    mut world: bevy::ecs::world::DeferredWorld,
    eid: Entity,
    _: bevy::ecs::component::ComponentId,
) {
    let pos = world.get::<Pos>(eid).expect("StaticTx needs Pos").clone();
    let hboxes = world
        .get::<StaticTx>(eid)
        .expect("StaticTx myself")
        .comps
        .iter()
        .map(|c| c.hbox.clone())
        .collect::<Vec<_>>();
    let keys = world
        .resource_mut::<SpatHash<SpatHashStaticTx>>()
        .insert(eid, pos, hboxes);
    world.commands().entity(eid).insert(keys);
}
fn on_remove_static_tx(
    mut world: bevy::ecs::world::DeferredWorld,
    eid: Entity,
    cid: bevy::ecs::component::ComponentId,
) {
    let occlude_light = world.get::<OccludeLight>(eid).cloned();
    if let Some(OccludeLight::StaticTx) = occlude_light {
        world.commands().entity(eid).remove::<OccludeLight>();
    }
    on_remove_spat_hash::<SpatHashStaticTx>(world, eid, cid);
}
impl StaticTx {
    pub fn single(kind: StaticTxKind, hbox: HBox) -> Self {
        Self::new(vec![(kind, hbox)])
    }
    pub fn new<I: IntoIterator<Item = (StaticTxKind, HBox)>>(data: I) -> Self {
        Self {
            comps: data
                .into_iter()
                .map(|(kind, hbox)| StaticTxComp { kind, hbox })
                .collect(),
            coll_keys: vec![],
        }
    }
    pub fn get_thboxes(&self, pos: Pos) -> Vec<HBox> {
        self.comps
            .iter()
            .map(|comp| comp.hbox.translated(pos.as_fvec2()))
            .collect()
    }
}
