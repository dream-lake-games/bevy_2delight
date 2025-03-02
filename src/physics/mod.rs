use bevy::prelude::*;

pub mod prelude {
    pub use super::colls::{
        ByHBox, StaticCollRec, StaticColls, TriggerCollRecGeneric, TriggerCollsGeneric,
    };
    pub use super::dyno::Dyno;
    pub use super::hbox::{HBox, HBoxMarker};
    pub use super::plugin::*;
    pub use super::pos::Pos;
    pub use super::statics::{StaticRx, StaticRxKind, StaticTx, StaticTxKind};
    pub use super::triggers::{TriggerKindTrait, TriggerRxGeneric, TriggerTxGeneric};
    pub(crate) use super::PhysicsSet;
}

mod colls;
mod debug;
mod dyno;
mod hbox;
mod logic;
mod plugin;
mod pos;
mod statics;
mod triggers;

/// The set that contains all physics related systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PhysicsSet;
