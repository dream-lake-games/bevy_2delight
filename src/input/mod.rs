use bevy::ecs::schedule::SystemSet;

mod input_data;
mod input_logic;
mod input_plugin;

/// The set that contains all physics related systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct InputSet;

pub mod prelude {
    pub use super::input_data::*;
    pub(crate) use super::input_plugin::*;
    pub(crate) use super::InputSet;
}
