use bevy::{
    ecs::schedule::SystemSet,
    prelude::{Reflect, Resource},
    utils::HashMap,
};

use crate::prelude::Frac;

#[derive(Resource, Default)]
pub struct AnimsPaused(pub bool);

#[derive(Default, Copy, Clone, Debug, Reflect, PartialEq, Eq, std::hash::Hash)]
pub enum AnimTimeClass {
    /// Play the animation respecting bullet time, only when AnimsPaused is false
    #[default]
    BulletUnpaused,
    /// Play the animation respecting bullet time, regardless of AnimsPaused
    BulletAlways,
    /// Play the animation ignoring bullet time, only when AnimsPaused is false
    RealUnpaused,
    /// Play the animation ignoring bullet time, regardless of AnimsPaused
    RealAlways,
}

/// This schedule should update all of the timeclasses
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnimTimeSet;

#[derive(Resource, Debug, Default)]
pub struct AnimTime {
    pub(crate) map: HashMap<AnimTimeClass, Frac>,
}
impl AnimTime {
    pub fn clear(&mut self) {
        self.map.clear();
    }
    pub fn set(&mut self, class: AnimTimeClass, frac: Frac) {
        self.map.insert(class, frac);
    }
    pub fn get(&self, class: AnimTimeClass) -> Frac {
        self.map.get(&class).copied().unwrap_or(Frac::ZERO)
    }
}
