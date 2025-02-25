use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_2delight::prelude::*;

mod bgfg;
mod ldtk;

#[derive(std::hash::Hash, Debug, Clone, TriggerKind)]
enum TriggerRxKind {
    Player,
}

#[derive(std::hash::Hash, Debug, Clone, PartialEq, Eq, TriggerKind)]
enum TriggerTxKind {
    Spikes,
}

type TriggerRx = TriggerRxGeneric<TriggerRxKind>;
type TriggerTx = TriggerTxGeneric<TriggerTxKind>;
type TriggerColls = TriggerCollsGeneric<TriggerRxKind, TriggerTxKind>;
#[expect(dead_code)]
type TriggerCollRec = TriggerCollRecGeneric<TriggerRxKind, TriggerTxKind>;
type PhysicsSettings = PhysicsSettingsGeneric<TriggerRxKind, TriggerTxKind>;

fn main() {
    let mut app = App::new();

    app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
    app.add_plugins(TwoDelightPlugin {
        anim_settings: default(),
        composition_settings: CompositionSettings {
            screen_size: UVec2::new(240, 240),
            ..default()
        },
        ldtk_settings: ldtk::LdtkSettings::default(),
        physics_settings: PhysicsSettings::default(),
    });
    app.add_plugins(
        bevy_inspector_egui::quick::WorldInspectorPlugin::default().run_if(
            bevy::input::common_conditions::input_toggle_active(false, KeyCode::Tab),
        ),
    );

    bgfg::register_bgfg(&mut app);
    ldtk::register_ldtk(&mut app);

    app.run();
}
