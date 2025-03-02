use bevy::prelude::*;

mod entity;
mod int_cell;
mod ldtk_maint;
mod ldtk_roots;
mod load;
mod plugin;

/// The set that contains all weird ldtk maintenence
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct LdtkSet;

pub mod prelude {
    pub use super::entity::{LdtkEntity, LdtkEntityPluginGeneric};
    pub use super::int_cell::{
        LdtkIntCellLayerer, LdtkIntCellValue, LdtkIntCellValuePluginGeneric,
    };
    pub use super::ldtk_maint::LdtkLevelRects;
    pub use super::ldtk_roots::{LdtkRootKind, LdtkRootResGeneric};
    pub use super::load::{LdtkState, LoadLdtk, UnloadLdtk};
    pub(crate) use super::plugin::LdtkPlugin;
    pub use super::plugin::LdtkSettingsGeneric;
    pub(crate) use super::LdtkSet;
}
