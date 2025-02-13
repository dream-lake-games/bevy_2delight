use bevy::prelude::*;

use crate::prelude::{
    AnimPlugin, AnimSettings, BulletTimePlugin, PhysicsPluginGeneric, PhysicsSettingsGeneric,
    TriggerKind,
};

pub struct TwoDelightPlugin<TriggerRxKind: TriggerKind, TriggerTxKind: TriggerKind> {
    pub anim_settings: AnimSettings,
    pub physics_settings: PhysicsSettingsGeneric<TriggerRxKind, TriggerTxKind>,
}
impl<TriggerRxKind: TriggerKind, TriggerTxKind: TriggerKind> Plugin
    for TwoDelightPlugin<TriggerRxKind, TriggerTxKind>
{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AnimPlugin::new(self.anim_settings.clone()),
            BulletTimePlugin::default(),
            PhysicsPluginGeneric::<TriggerRxKind, TriggerTxKind>::default(),
        ));
    }
}
