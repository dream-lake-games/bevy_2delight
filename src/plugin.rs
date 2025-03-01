use bevy::prelude::*;

use crate::{
    glue::Deterministic,
    prelude::{
        AnimPlugin, AnimSettings, BulletTimePlugin, CompositionPlugin, CompositionSettings,
        LdtkPlugin, LdtkRootKind, LdtkSettingsGeneric, PhysicsPluginGeneric,
        PhysicsSettingsGeneric, TriggerKindTrait,
    },
};

pub struct TwoDelightPlugin<
    LdtkRoot: LdtkRootKind,
    TriggerRxKind: TriggerKindTrait,
    TriggerTxKind: TriggerKindTrait,
> {
    pub anim_settings: AnimSettings,
    pub composition_settings: CompositionSettings,
    pub ldtk_settings: LdtkSettingsGeneric<LdtkRoot>,
    pub physics_settings: PhysicsSettingsGeneric<TriggerRxKind, TriggerTxKind>,
    pub deterministic: bool,
}
impl<LdtkRoot: LdtkRootKind, TriggerRxKind: TriggerKindTrait, TriggerTxKind: TriggerKindTrait>
    Plugin for TwoDelightPlugin<LdtkRoot, TriggerRxKind, TriggerTxKind>
{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CompositionPlugin::new(self.composition_settings.clone()),
            AnimPlugin::new(self.anim_settings.clone()),
            BulletTimePlugin::default(),
            LdtkPlugin::<LdtkRoot>::default(),
            PhysicsPluginGeneric::<TriggerRxKind, TriggerTxKind>::default(),
        ));
        app.insert_resource(Deterministic(self.deterministic));
    }
}
