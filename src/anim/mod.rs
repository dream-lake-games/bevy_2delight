use bevy::prelude::*;

mod anim_collect;
mod logic;
mod man;
mod plugin;
mod time;
mod traits;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct AnimPreSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct AnimPostSet;

pub mod prelude {
    pub use super::{
        anim_collect::_AnimWizardry,
        man::{AnimDelta, AnimMan, AnimNextState},
        plugin::*,
        time::{AnimTime, AnimTimeClass},
        traits::AnimStateMachine,
    };
    pub use crate::defn_anim;
}
