use bevy::prelude::*;

mod ldtk_entity;
mod ldtk_int_cell;
mod ldtk_load;
mod ldtk_maint;
mod ldtk_roots;
mod plugin;

/// The set that contains all weird ldtk maintenence
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LdtkSet;

pub mod prelude {
    pub use super::ldtk_entity::{
        LdtkBundleEntity, LdtkBundleEntityPluginGeneric, LdtkEntity, LdtkEntityPluginGeneric,
    };
    pub use super::ldtk_int_cell::{
        LdtkIntCellConsolidate, LdtkIntCellLayerer, LdtkIntCellValue, LdtkIntCellValuePluginGeneric,
    };
    pub use super::ldtk_load::{LdtkState, LoadLdtk, UnloadLdtk};
    pub use super::ldtk_maint::LdtkLevelRects;
    pub use super::ldtk_roots::{LdtkRootKind, LdtkRootResGeneric};
    pub(crate) use super::plugin::LdtkPlugin;
    pub use super::plugin::LdtkSettingsGeneric;
    pub use super::LdtkSet;
}
