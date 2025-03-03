use bevy::prelude::*;

use crate::physics::{colls, logic, triggers::TriggerKindTrait};

use super::{debug::PhysicsDebugPluginGeneric, spat_hash};

pub struct PhysicsSettingsGeneric<TriggerRxKind: TriggerKindTrait, TriggerTxKind: TriggerKindTrait>
{
    _pd: std::marker::PhantomData<(TriggerRxKind, TriggerTxKind)>,
}
impl<TriggerRxKind: TriggerKindTrait, TriggerTxKind: TriggerKindTrait> Default
    for PhysicsSettingsGeneric<TriggerRxKind, TriggerTxKind>
{
    fn default() -> Self {
        Self {
            _pd: std::marker::PhantomData,
        }
    }
}

pub(crate) struct PhysicsPluginGeneric<
    TriggerRxKind: TriggerKindTrait,
    TriggerTxKind: TriggerKindTrait,
> {
    _pd: std::marker::PhantomData<(TriggerRxKind, TriggerTxKind)>,
}
impl<TriggerRxKind: TriggerKindTrait, TriggerTxKind: TriggerKindTrait> Default
    for PhysicsPluginGeneric<TriggerRxKind, TriggerTxKind>
{
    fn default() -> Self {
        Self {
            _pd: std::marker::PhantomData,
        }
    }
}
impl<TriggerRx: TriggerKindTrait, TriggerTx: TriggerKindTrait> Plugin
    for PhysicsPluginGeneric<TriggerRx, TriggerTx>
{
    fn build(&self, app: &mut App) {
        colls::register_colls::<TriggerRx, TriggerTx>(app);
        logic::register_logic::<TriggerRx, TriggerTx>(app);
        spat_hash::register_spat_hash(app);

        #[cfg(debug_assertions)]
        {
            app.add_plugins(PhysicsDebugPluginGeneric::<TriggerRx, TriggerTx>::default());
        }
    }
}
