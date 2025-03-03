use std::marker::PhantomData;

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use crate::glue::Fx;

use super::{hbox::HBox, pos::Pos};

pub(crate) trait SpatHashKind: Component + Clone + Reflect {}

#[derive(Component, Clone, Reflect)]
pub(crate) struct SpatHashStaticRx;
impl SpatHashKind for SpatHashStaticRx {}
#[derive(Component, Clone, Reflect)]
pub(crate) struct SpatHashStaticTx;
impl SpatHashKind for SpatHashStaticTx {}
#[derive(Component, Clone, Reflect)]
pub(crate) struct SpatHashTriggerRx;
impl SpatHashKind for SpatHashTriggerRx {}
#[derive(Component, Clone, Reflect)]
pub(crate) struct SpatHashTriggerTx;
impl SpatHashKind for SpatHashTriggerTx {}

#[derive(Copy, Clone, PartialEq, Eq, std::hash::Hash, Debug, Reflect)]
pub(crate) struct SpatKey {
    x: i32,
    y: i32,
}
impl SpatKey {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Component, Clone, Reflect)]
#[component(on_remove = on_remove_spat_hash::<K>)]
pub(crate) struct SpatKeys<K: SpatHashKind> {
    keys: HashSet<SpatKey>,
    _pd: Option<K>,
}
impl<K: SpatHashKind> SpatKeys<K> {
    fn new(keys: HashSet<SpatKey>) -> Self {
        Self { keys, _pd: None }
    }
    pub(crate) fn iter(&self) -> bevy::utils::hashbrown::hash_set::Iter<'_, SpatKey> {
        self.keys.iter()
    }
}

#[derive(Resource, Reflect)]
pub(crate) struct SpatHash<K: SpatHashKind> {
    denominator: i32,
    map: HashMap<SpatKey, HashSet<Entity>>,
    _pd: PhantomData<K>,
}
impl<K: SpatHashKind> Default for SpatHash<K> {
    fn default() -> Self {
        Self {
            denominator: 32,
            map: default(),
            _pd: default(),
        }
    }
}
impl<K: SpatHashKind> SpatHash<K> {
    fn inner_insert(&mut self, key: SpatKey, eid: Entity) {
        if !self.map.contains_key(&key) {
            self.map.insert(key, default());
        }
        let inner_set = self.map.get_mut(&key).unwrap();
        inner_set.insert(eid);
    }
    fn inner_remove(&mut self, key: SpatKey, eid: Entity) {
        let mut empty = false;
        if let Some(inner_set) = self.map.get_mut(&key) {
            inner_set.remove(&eid);
            empty = inner_set.is_empty();
        }
        if empty {
            self.map.remove(&key);
        }
    }
}
impl<K: SpatHashKind> SpatHash<K> {
    pub fn get_thbox_keys(&self, hbox: HBox) -> HashSet<SpatKey> {
        let (min_x, max_x) = (
            (hbox.min_x() / Fx::from_num(self.denominator))
                .floor()
                .to_num::<i32>(),
            (hbox.max_x() / Fx::from_num(self.denominator))
                .ceil()
                .to_num::<i32>(),
        );
        let (min_y, max_y) = (
            (hbox.min_y() / Fx::from_num(self.denominator))
                .floor()
                .to_num::<i32>(),
            (hbox.max_y() / Fx::from_num(self.denominator))
                .ceil()
                .to_num::<i32>(),
        );
        let mut result = HashSet::default();
        for x in min_x..max_x {
            for y in min_y..max_y {
                result.insert(SpatKey::new(x, y));
            }
        }
        result
    }

    pub fn get_keys(&self, pos: Pos, hboxes: &[HBox]) -> SpatKeys<K> {
        let hset = HashSet::from_iter(
            hboxes
                .iter()
                .map(|hbox| self.get_thbox_keys(hbox.translated(pos.as_fvec2())))
                .flatten(),
        );
        SpatKeys::new(hset)
    }

    pub fn insert(&mut self, eid: Entity, pos: Pos, hboxes: &[HBox]) -> SpatKeys<K> {
        let keys = self.get_keys(pos, hboxes);
        for key in keys.iter() {
            self.inner_insert(*key, eid);
        }
        keys
    }

    pub fn remove(&mut self, eid: Entity, keys: &SpatKeys<K>) {
        for key in keys.iter() {
            self.inner_remove(*key, eid);
        }
    }

    // Sugar for remove + insert
    pub fn update(
        &mut self,
        eid: Entity,
        old_keys: &SpatKeys<K>,
        pos: Pos,
        hboxes: &[HBox],
    ) -> SpatKeys<K> {
        self.remove(eid, old_keys);
        let new_keys = self.get_keys(pos, hboxes);
        for key in new_keys.iter() {
            self.inner_insert(*key, eid);
        }
        new_keys
    }
}

pub(crate) fn on_remove_spat_hash<K: SpatHashKind>(
    mut world: bevy::ecs::world::DeferredWorld,
    eid: Entity,
    _: bevy::ecs::component::ComponentId,
) {
    let keys = world.get::<SpatKeys<K>>(eid).unwrap().clone();
    world.resource_mut::<SpatHash<K>>().remove(eid, &keys);
}

pub(super) fn register_spat_hash(app: &mut App) {
    macro_rules! handle_thingy {
        ($kind:ty) => {
            app.register_type::<SpatKeys<$kind>>();
            app.insert_resource(SpatHash::<$kind>::default());
        };
    }
    handle_thingy!(SpatHashStaticRx);
    handle_thingy!(SpatHashStaticTx);
    handle_thingy!(SpatHashTriggerRx);
    handle_thingy!(SpatHashTriggerTx);
}
