use bevy::prelude::*;

use crate::{
    anim::{AnimPostSet, AnimPreSet},
    composition::LightingSet,
    glue::Deterministic,
    input::InputSet,
    ldtk::LdtkSet,
    particles::ParticleSet,
    physics::PhysicsSet,
    prelude::{
        AnimPlugin, AnimSettings, BulletTimePlugin, CompositionPlugin, CompositionSettings,
        InputPlugin, LayersCameraSet, LdtkPlugin, LdtkRootKind, LdtkSettingsGeneric, LightAnimSet,
        LightInteractionSet, ParticlePlugin, PhysicsPluginGeneric, PhysicsSettingsGeneric,
        ShaderPlugin, TriggerKindTrait,
    },
    shader::ShaderSet,
};

/// You should add all actual game logic to this set.
/// Everything in this set will run:
/// - AFTER the default animation progression logic
/// - AFTER the physics system
/// - BEFORE we actually drive the animation sprites
/// - BEFORE we move any cameras
/// - BEFORE we trigger any anim state change behavior
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DelightedSet;

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
        // Configurations between internal systems and other internal systems
        app.configure_sets(
            Update,
            (
                LightAnimSet.before(AnimPostSet),
                LightInteractionSet.after(PhysicsSet),
            ),
        );

        // Configurations between internal systems and user systems
        app.configure_sets(
            Update,
            (
                // Pre-delighted
                AnimPreSet.before(DelightedSet),
                LdtkSet.before(DelightedSet),
                LightInteractionSet.before(DelightedSet),
                PhysicsSet.before(DelightedSet),
                InputSet.before(DelightedSet),
                // Post-delighted
                AnimPostSet.after(DelightedSet),
                LayersCameraSet.after(DelightedSet),
                LightAnimSet.after(DelightedSet),
                LightingSet.after(DelightedSet),
                ParticleSet.after(DelightedSet),
                ShaderSet.after(DelightedSet),
            ),
        );

        app.add_plugins((
            CompositionPlugin::new(self.composition_settings.clone()),
            AnimPlugin::new(self.anim_settings.clone()),
            BulletTimePlugin::default(),
            LdtkPlugin::<LdtkRoot>::default(),
            ParticlePlugin,
            PhysicsPluginGeneric::<TriggerRxKind, TriggerTxKind>::default(),
            ShaderPlugin,
            InputPlugin,
        ));
        app.insert_resource(Deterministic(self.deterministic));
    }
}
