use bevy::prelude::*;

use crate::physics::{colls, logic, pos, triggers::TriggerKind};

pub struct PhysicsSettingsGeneric<TriggerRxKind: TriggerKind, TriggerTxKind: TriggerKind> {
    _pd: std::marker::PhantomData<(TriggerRxKind, TriggerTxKind)>,
}
impl<TriggerRxKind: TriggerKind, TriggerTxKind: TriggerKind> Default
    for PhysicsSettingsGeneric<TriggerRxKind, TriggerTxKind>
{
    fn default() -> Self {
        Self {
            _pd: std::marker::PhantomData,
        }
    }
}

pub(crate) struct PhysicsPluginGeneric<TriggerRxKind: TriggerKind, TriggerTxKind: TriggerKind> {
    _pd: std::marker::PhantomData<(TriggerRxKind, TriggerTxKind)>,
}
impl<TriggerRxKind: TriggerKind, TriggerTxKind: TriggerKind> Default
    for PhysicsPluginGeneric<TriggerRxKind, TriggerTxKind>
{
    fn default() -> Self {
        Self {
            _pd: std::marker::PhantomData,
        }
    }
}
impl<TriggerRxKind: TriggerKind, TriggerTxKind: TriggerKind> Plugin
    for PhysicsPluginGeneric<TriggerRxKind, TriggerTxKind>
{
    fn build(&self, app: &mut App) {
        colls::register_colls::<TriggerRxKind, TriggerTxKind>(app);
        logic::register_logic::<TriggerRxKind, TriggerTxKind>(app);
        pos::register_pos(app);
    }
}
