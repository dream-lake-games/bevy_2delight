use bevy::prelude::*;

use crate::physics::{colls::CollKey, hbox::HBox, pos::Pos};

pub trait TriggerKind:
    Clone + std::fmt::Debug + std::hash::Hash + std::marker::Send + std::marker::Sync + 'static
{
}

pub(crate) struct TriggerRxComp<TriggerRxKind: TriggerKind> {
    pub(crate) kind: TriggerRxKind,
    pub(crate) hbox: HBox,
}
#[derive(Component)]
pub struct TriggerRxGeneric<TriggerRxKind: TriggerKind> {
    pub(crate) comps: Vec<TriggerRxComp<TriggerRxKind>>,
    pub coll_keys: Vec<CollKey>,
}
impl<TriggerRxKind: TriggerKind> TriggerRxGeneric<TriggerRxKind> {
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
            .map(|comp| comp.hbox.translated(pos.x, pos.y))
            .collect()
    }
}

pub(crate) struct TriggerTxComp<TriggerTxKind: TriggerKind> {
    pub(crate) kind: TriggerTxKind,
    pub(crate) hbox: HBox,
}
#[derive(Component)]
pub struct TriggerTxGeneric<TriggerTxKind: TriggerKind> {
    pub(crate) comps: Vec<TriggerTxComp<TriggerTxKind>>,
    pub coll_keys: Vec<CollKey>,
}
impl<TriggerTxKind: TriggerKind> TriggerTxGeneric<TriggerTxKind> {
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
            .map(|comp| comp.hbox.translated(pos.x, pos.y))
            .collect()
    }
}
