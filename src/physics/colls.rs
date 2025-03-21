use bevy::{prelude::*, utils::HashMap};

use crate::{
    glue::fvec::FVec2,
    physics::{
        hbox::HBoxMarker,
        pos::Pos,
        statics::{StaticRx, StaticRxKind, StaticTx, StaticTxKind},
        triggers::{TriggerKindTrait, TriggerRxGeneric, TriggerTxGeneric},
        PhysicsSet,
    },
};

pub type CollKey = u32;

#[derive(Debug, Clone)]
pub struct StaticCollRec {
    pub push: FVec2,
    /// Position of rx at time of collision
    pub rx_pos: Pos,
    /// Before collision, component of rx's velocity in collision normal direction
    pub rx_perp: FVec2,
    /// Before collision, component of rx's velocity perpendicular to normal direction
    /// Name is weird because it's "parallel" to original vel of rx
    pub rx_par: FVec2,
    /// Entity of the control associated with the rx
    pub rx_ctrl: Entity,
    /// The kind of the rx
    pub rx_kind: StaticRxKind,
    /// The marker of the hbox on the rx  triggering this collision
    pub rx_hbox: HBoxMarker,
    /// Position of tx at time of collision
    pub tx_pos: Pos,
    /// Entity of the control associated with the tx
    pub tx_ctrl: Entity,
    /// The kind of the tx
    pub tx_kind: StaticTxKind,
    /// The marker of the hbox on the tx  triggering this collision
    pub tx_hbox: HBoxMarker,
}
#[derive(Resource, Debug)]
pub struct StaticColls {
    pub(crate) map: HashMap<CollKey, StaticCollRec>,
}
impl StaticColls {
    pub(crate) fn insert(&mut self, rec: StaticCollRec) {
        let key = self.map.len() as CollKey;
        self.map.insert(key, rec);
    }
    pub fn get(&self, key: &CollKey) -> Option<&StaticCollRec> {
        self.map.get(key)
    }
    pub fn get_refs<'a, 'b>(
        &'a self,
        coll_keys: &'b [CollKey],
    ) -> std::iter::FilterMap<
        std::slice::Iter<'b, u32>,
        impl FnMut(&'b u32) -> Option<&'a StaticCollRec>,
    > {
        coll_keys.iter().filter_map(|key| self.get(key))
    }
    pub fn all(&self) -> Vec<&StaticCollRec> {
        self.map.values().into_iter().collect()
    }
}

#[derive(Debug, Clone)]
pub struct TriggerCollRecGeneric<TriggerRxKind: TriggerKindTrait, TriggerTxKind: TriggerKindTrait> {
    /// Position of rx at time of collision
    pub rx_pos: Pos,
    /// Entity of the control associated with the rx
    pub rx_ctrl: Entity,
    /// The kind of the rx
    pub rx_kind: TriggerRxKind,
    /// The marker of the hbox on the rx triggering this collision
    pub rx_hbox: HBoxMarker,
    /// Position of tx at time of collision
    pub tx_pos: Pos,
    /// Entity of the control associated with the tx
    pub tx_ctrl: Entity,
    /// The kind of the tx
    pub tx_kind: TriggerTxKind,
    /// The marker of the hbox on the tx triggering this collision
    pub tx_hbox: HBoxMarker,
}
#[derive(Resource, Debug)]
pub struct TriggerCollsGeneric<TriggerRxKind: TriggerKindTrait, TriggerTxKind: TriggerKindTrait> {
    pub(crate) map: HashMap<CollKey, TriggerCollRecGeneric<TriggerRxKind, TriggerTxKind>>,
}
impl<TriggerRxKind: TriggerKindTrait, TriggerTxKind: TriggerKindTrait>
    TriggerCollsGeneric<TriggerRxKind, TriggerTxKind>
{
    pub fn insert(&mut self, rec: TriggerCollRecGeneric<TriggerRxKind, TriggerTxKind>) {
        let key = self.map.len() as CollKey;
        self.map.insert(key, rec);
    }
    pub fn get(
        &self,
        key: &CollKey,
    ) -> Option<&TriggerCollRecGeneric<TriggerRxKind, TriggerTxKind>> {
        self.map.get(key)
    }
    pub fn get_refs<'a, 'b>(
        &'a self,
        coll_keys: &'b [CollKey],
    ) -> std::iter::FilterMap<
        std::slice::Iter<'b, u32>,
        impl FnMut(&'b u32) -> Option<&'a TriggerCollRecGeneric<TriggerRxKind, TriggerTxKind>>,
    > {
        coll_keys.iter().filter_map(|key| self.get(key))
    }
}

/// Helpful trait to categorize collisions by marked hitboxes
pub trait ByHBox<'a, Record> {
    fn by_rx_hbox(self) -> HashMap<HBoxMarker, Vec<&'a Record>>;
    fn by_tx_hbox(self) -> HashMap<HBoxMarker, Vec<&'a Record>>;
}
impl<'a> ByHBox<'a, StaticCollRec> for Vec<&'a StaticCollRec> {
    fn by_rx_hbox(self) -> HashMap<HBoxMarker, Vec<&'a StaticCollRec>> {
        let mut result = HashMap::<HBoxMarker, Vec<&'a StaticCollRec>>::new();
        for rec in self.into_iter() {
            if result.get_mut(&rec.rx_hbox).is_some() {
                result.get_mut(&rec.rx_hbox).unwrap().push(rec);
            } else {
                result.insert(rec.rx_hbox, vec![rec]);
            }
        }
        result
    }
    fn by_tx_hbox(self) -> HashMap<HBoxMarker, Vec<&'a StaticCollRec>> {
        let mut result = HashMap::<HBoxMarker, Vec<&'a StaticCollRec>>::new();
        for rec in self.into_iter() {
            if result.get_mut(&rec.tx_hbox).is_some() {
                result.get_mut(&rec.tx_hbox).unwrap().push(rec);
            } else {
                result.insert(rec.tx_hbox, vec![rec]);
            }
        }
        result
    }
}
impl<'a, TriggerRxKind: TriggerKindTrait, TriggerTxKind: TriggerKindTrait>
    ByHBox<'a, TriggerCollRecGeneric<TriggerRxKind, TriggerTxKind>>
    for Vec<&'a TriggerCollRecGeneric<TriggerRxKind, TriggerTxKind>>
{
    fn by_rx_hbox(
        self,
    ) -> HashMap<HBoxMarker, Vec<&'a TriggerCollRecGeneric<TriggerRxKind, TriggerTxKind>>> {
        let mut result = HashMap::<
            HBoxMarker,
            Vec<&'a TriggerCollRecGeneric<TriggerRxKind, TriggerTxKind>>,
        >::new();
        for rec in self.into_iter() {
            if result.get_mut(&rec.rx_hbox).is_some() {
                result.get_mut(&rec.rx_hbox).unwrap().push(rec);
            } else {
                result.insert(rec.rx_hbox, vec![rec]);
            }
        }
        result
    }
    fn by_tx_hbox(
        self,
    ) -> HashMap<HBoxMarker, Vec<&'a TriggerCollRecGeneric<TriggerRxKind, TriggerTxKind>>> {
        let mut result = HashMap::<
            HBoxMarker,
            Vec<&'a TriggerCollRecGeneric<TriggerRxKind, TriggerTxKind>>,
        >::new();
        for rec in self.into_iter() {
            if result.get_mut(&rec.tx_hbox).is_some() {
                result.get_mut(&rec.tx_hbox).unwrap().push(rec);
            } else {
                result.insert(rec.tx_hbox, vec![rec]);
            }
        }
        result
    }
}

pub(super) fn register_colls<TriggerRxKind: TriggerKindTrait, TriggerTxKind: TriggerKindTrait>(
    app: &mut App,
) {
    app.insert_resource(StaticColls { map: default() });
    app.insert_resource(TriggerCollsGeneric::<TriggerRxKind, TriggerTxKind> { map: default() });
}
