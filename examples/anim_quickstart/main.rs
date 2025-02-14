use bevy::prelude::*;
use bevy_2delight::prelude::*;

// You also could write:
// #[derive(Debug, Copy, Clone, Default, Reflect, PartialEq, Eq, Hash, AnimStateMachine)]
// but that's a lot to remember.
defn_anim!(
    SlowCircleAnim,
    // The folder containing the tag sheets + jsons
    #[folder("anim_quickstart/circle")]
    // What should the z-index of this animation be?
    // OPTIONAL: Defaults to 0.0
    // NOTE: If you have multiple animations on an entity and notice flickering/overlapping, it's likely because
    //       they all have the same zix. Give them an explicit ordering.
    #[zix(10.0)]
    // What should the time behavior of this anim be?
    #[time_class(BulletUnpaused)]
    // How many times to repeat (tile) the anim in each direction?
    #[rep(2, 3)]
    pub enum SlowCircleAnim {
        #[default]
        // The tag of the animation
        #[tag("spin")]
        // What is the FPS of this animation?
        // OPTIONAL: Defaults to the value in `AnimPlugin`.
        #[fps(3)]
        // Should the animation be offset from it's parent?
        // OPTIONAL: Defaults to (0.0, 0.0)
        #[offset(10, 0)]
        // After this animation completes, what should happen?
        // OPTIONAL: Defaults to looping. You can provide the name of another variant (like 'Spin'), Despawn, or Remove.
        #[next(Despawn)]
        Spin,
    }
);

defn_anim!(
    FastCircleAnim,
    #[folder("anim_quickstart/circle")]
    #[zix(10.0)]
    #[time_class(BulletAlways)]
    pub enum FastCircleAnim {
        #[default]
        #[tag("spin")]
        #[fps(24)]
        #[offset(-40, 0)]
        Spin,
    }
);

#[derive(std::hash::Hash, Debug, Clone, TriggerKind)]
enum TriggerRxKind {}

#[derive(std::hash::Hash, Debug, Clone, TriggerKind)]
enum TriggerTxKind {}
type PhysicsSettings = PhysicsSettingsGeneric<TriggerRxKind, TriggerTxKind>;

fn startup(mut commands: Commands) {
    commands.spawn((Name::new("camera"), Camera2d::default()));
    commands.spawn((
        Name::new("slow_circle"),
        AnimMan::<SlowCircleAnim>::default(),
        Visibility::default(),
    ));
    commands.spawn((
        Name::new("fast_circle"),
        AnimMan::<FastCircleAnim>::default(),
        Visibility::default(),
    ));
}

fn main() {
    let mut app = App::new();

    app.add_plugins(TwoDelightPlugin {
        anim_settings: AnimSettings::default(),
        physics_settings: PhysicsSettings::default(),
        layer_settings: LayerSettings {
            screen_size: UVec2::new(240, 240),
            overlay_growth: 4,
            window: Window {
                resizable: true,
                title: "Anim Quickstart".to_string(),
                mode: bevy::window::WindowMode::Windowed,
                ..default()
            },
            asset_plugin: AssetPlugin {
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            },
        },
    });

    app.add_systems(Startup, startup);

    app.run();
}
