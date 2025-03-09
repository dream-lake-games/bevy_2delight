use bevy::prelude::*;

mod anim_collect;
mod anim_logic;
mod anim_man;
mod anim_plugin;
mod anim_res;
mod anim_time;
mod anim_traits;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct AnimPreSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct AnimPostSet;

pub mod prelude {
    pub use super::{
        anim_collect::_AnimWizardry,
        anim_man::{AnimDelta, AnimMan, AnimNextState},
        anim_plugin::*,
        anim_time::{AnimTime, AnimTimeClass},
        anim_traits::AnimStateMachine,
    };
    pub use crate::defn_anim;
}
