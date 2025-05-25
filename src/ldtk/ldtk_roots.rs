use bevy::{prelude::*, reflect::Reflectable};

use crate::prelude::*;

#[derive(Resource)]
pub struct LdtkRootResGeneric<R: LdtkRootKind> {
    map: HashMap<R, Entity>,
}
impl<R: LdtkRootKind> LdtkRootResGeneric<R> {
    pub fn get_eid(&self, key: R) -> Entity {
        self.map.get(&key).copied().unwrap()
    }
    fn insert(&mut self, key: R, eid: Entity) {
        self.map.insert(key, eid);
    }
}

pub trait LdtkRootKind:
    Sized
    + Send
    + Sync
    + 'static
    + std::hash::Hash
    + PartialEq
    + Eq
    + Copy
    + Default
    + FromReflect
    + Reflectable
    + std::fmt::Debug
    + strum::IntoEnumIterator
{
}

fn create_roots<R: LdtkRootKind>(mut commands: Commands, mut roots: ResMut<LdtkRootResGeneric<R>>) {
    for root in R::iter() {
        let eid = commands
            .spawn((
                Name::new(format!("{:?}", root)),
                Transform::default(),
                Visibility::default(),
            ))
            .id();
        roots.insert(root, eid);
    }
}

pub(super) fn register_ldtk_root<R: LdtkRootKind>(app: &mut App) {
    app.insert_resource(LdtkRootResGeneric::<R> { map: default() });

    app.add_systems(Startup, create_roots::<R>);
}
