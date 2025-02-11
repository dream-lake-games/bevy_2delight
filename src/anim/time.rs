use bevy::prelude::Resource;

use super::traits::AnimTimeProvider;

pub type AnimTimeClass = i32;
pub const DEFAULT_TIME_CLASS: AnimTimeClass = 0;

#[derive(Resource, Debug, Default)]
pub struct AnimPlaceholderTime {
    pub(crate) real_time_delta_us: u32,
}
impl AnimTimeProvider for AnimPlaceholderTime {
    fn get_delta_us(&self, class: AnimTimeClass) -> u32 {
        match class {
            DEFAULT_TIME_CLASS => self.real_time_delta_us,
            wrong => panic!("Unsupported time class: {wrong}. The placeholder time only supportes one time class, {DEFAULT_TIME_CLASS}. For more, add your own AnimTimeProvider Resource"),
        }
    }
}
