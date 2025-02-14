use bevy::prelude::*;

use crate::prelude::{
    AnimPlugin, AnimSettings, BulletTimePlugin, LayerPlugin, LayerSettings, PhysicsPluginGeneric,
    PhysicsSettingsGeneric, TriggerKindTrait,
};

pub struct TwoDelightPlugin<TriggerRxKind: TriggerKindTrait, TriggerTxKind: TriggerKindTrait> {
    pub anim_settings: AnimSettings,
    pub physics_settings: PhysicsSettingsGeneric<TriggerRxKind, TriggerTxKind>,
    pub layer_settings: LayerSettings,
}
impl<TriggerRxKind: TriggerKindTrait, TriggerTxKind: TriggerKindTrait> Plugin
    for TwoDelightPlugin<TriggerRxKind, TriggerTxKind>
{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LayerPlugin::new(self.layer_settings.clone()),
            AnimPlugin::new(self.anim_settings.clone()),
            BulletTimePlugin::default(),
            PhysicsPluginGeneric::<TriggerRxKind, TriggerTxKind>::default(),
        ));
    }
}
