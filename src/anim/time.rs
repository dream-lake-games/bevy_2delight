use bevy::{ecs::schedule::SystemSet, prelude::Resource, utils::HashMap};

pub type AnimTimeClass = i32;
pub const DEFAULT_TIME_CLASS: AnimTimeClass = -1;

/// This schedule should update all of the timeclasses
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnimTimeSet;

#[derive(Resource, Debug, Default)]
pub struct AnimTime {
    pub(crate) map: HashMap<i32, u32>,
}
impl AnimTime {
    pub fn clear(&mut self) {
        self.map.clear();
    }
    pub fn set(&mut self, class: i32, us: u32) {
        self.map.insert(class, us);
    }
    pub fn get(&self, class: i32) -> u32 {
        self.map.get(&class).copied().unwrap_or(0)
    }
}
