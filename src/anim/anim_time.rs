use bevy::prelude::{Reflect, Resource};

use crate::prelude::*;

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

#[derive(Resource, Debug, Default)]
pub struct AnimTime {
    pub(crate) map: HashMap<AnimTimeClass, Fx>,
}
impl AnimTime {
    pub fn clear(&mut self) {
        self.map.clear();
    }
    pub fn set(&mut self, class: AnimTimeClass, frac: Fx) {
        self.map.insert(class, frac);
    }
    pub fn get(&self, class: AnimTimeClass) -> Fx {
        self.map.get(&class).copied().unwrap_or(Fx::ZERO)
    }
}
