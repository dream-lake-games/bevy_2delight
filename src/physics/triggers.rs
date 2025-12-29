use bevy::{ecs::lifecycle::HookContext, prelude::*};

use crate::physics::{colls::CollKey, hbox::HBox, pos::Pos};

use super::spat_hash::{on_remove_spat_hash, SpatHash, SpatHashTriggerTx};

pub trait TriggerKindTrait:
    Clone + std::fmt::Debug + std::hash::Hash + std::marker::Send + std::marker::Sync + 'static
{
}

pub(crate) struct TriggerRxComp<TriggerRxKind: TriggerKindTrait> {
    pub(crate) kind: TriggerRxKind,
    pub(crate) hbox: HBox,
}
#[derive(Component)]
pub struct TriggerRxGeneric<TriggerRxKind: TriggerKindTrait> {
    pub(crate) comps: Vec<TriggerRxComp<TriggerRxKind>>,
    pub coll_keys: Vec<CollKey>,
}
impl<TriggerRxKind: TriggerKindTrait> TriggerRxGeneric<TriggerRxKind> {
    pub fn single(kind: TriggerRxKind, hbox: HBox) -> Self {
        Self::new(vec![(kind, hbox)])
    }
    pub fn new<I: IntoIterator<Item = (TriggerRxKind, HBox)>>(data: I) -> Self {
        Self {
            comps: data
                .into_iter()
                .map(|(kind, hbox)| TriggerRxComp { kind, hbox })
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

pub(crate) struct TriggerTxComp<TriggerTxKind: TriggerKindTrait> {
    pub(crate) kind: TriggerTxKind,
    pub(crate) hbox: HBox,
}
#[derive(Component)]
#[component(on_add = on_add_trigger_tx::<TriggerTxKind>)]
#[component(on_remove = on_remove_spat_hash::<SpatHashTriggerTx>)]
pub struct TriggerTxGeneric<TriggerTxKind: TriggerKindTrait> {
    pub(crate) comps: Vec<TriggerTxComp<TriggerTxKind>>,
    pub coll_keys: Vec<CollKey>,
}
fn on_add_trigger_tx<TriggerTxKind: TriggerKindTrait>(
    mut world: bevy::ecs::world::DeferredWorld,
    hook: HookContext,
) {
    let pos = world
        .get::<Pos>(hook.entity)
        .expect("TriggerTx needs Pos")
        .clone();
    let hboxes = world
        .get::<TriggerTxGeneric<TriggerTxKind>>(hook.entity)
        .expect("TriggerTx myself")
        .comps
        .iter()
        .map(|c| c.hbox.clone())
        .collect::<Vec<_>>();
    let keys = world
        .resource_mut::<SpatHash<SpatHashTriggerTx>>()
        .insert(hook.entity, pos, hboxes);
    world.commands().entity(hook.entity).insert(keys);
}
impl<TriggerTxKind: TriggerKindTrait> TriggerTxGeneric<TriggerTxKind> {
    pub fn single(kind: TriggerTxKind, hbox: HBox) -> Self {
        Self::new(vec![(kind, hbox)])
    }
    pub fn new<I: IntoIterator<Item = (TriggerTxKind, HBox)>>(data: I) -> Self {
        Self {
            comps: data
                .into_iter()
                .map(|(kind, hbox)| TriggerTxComp { kind, hbox })
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
