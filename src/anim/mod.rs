use bevy::prelude::*;

mod collect;
mod logic;
mod man;
mod plugin;
mod time;
mod traits;

/// A schedule set containing all logic for updating and playing animations
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnimSet;

pub mod prelude {
    pub use super::{
        collect::_AnimWizardry,
        man::{AnimDelta, AnimMan, AnimNextState},
        plugin::{AnimDefnPlugin, AnimPlugin},
        time::{AnimPlaceholderTime, AnimTimeClass, DEFAULT_TIME_CLASS},
        traits::{AnimStateMachine, AnimTimeProvider},
        AnimSet,
    };
    pub use crate::defn_anim;
}
