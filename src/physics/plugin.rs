use bevy::prelude::*;

use crate::physics::{colls, logic, pos, triggers::TriggerKindTrait};

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
impl<TriggerRxKind: TriggerKindTrait, TriggerTxKind: TriggerKindTrait> Plugin
    for PhysicsPluginGeneric<TriggerRxKind, TriggerTxKind>
{
    fn build(&self, app: &mut App) {
        colls::register_colls::<TriggerRxKind, TriggerTxKind>(app);
        logic::register_logic::<TriggerRxKind, TriggerTxKind>(app);
        pos::register_pos(app);
    }
}
