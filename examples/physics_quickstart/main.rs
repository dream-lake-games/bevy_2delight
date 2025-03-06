use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_2delight::prelude::*;
use strum_macros::EnumIter;

#[derive(std::hash::Hash, Debug, Clone, TriggerKind)]
enum TriggerRxKind {
    Player,
}

#[derive(std::hash::Hash, Debug, Clone, PartialEq, Eq, TriggerKind)]
enum TriggerTxKind {
    Spikes,
}

// I _highly_ recommend you create type aliases here to cut back on some verbosity
type TriggerRx = TriggerRxGeneric<TriggerRxKind>;
type TriggerTx = TriggerTxGeneric<TriggerTxKind>;
type TriggerColls = TriggerCollsGeneric<TriggerRxKind, TriggerTxKind>;
#[allow(dead_code)]
type TriggerCollRec = TriggerCollRecGeneric<TriggerRxKind, TriggerTxKind>;
type PhysicsSettings = PhysicsSettingsGeneric<TriggerRxKind, TriggerTxKind>;

#[derive(Clone, Copy, Debug, Default, EnumIter, Reflect, PartialEq, Eq, std::hash::Hash)]
enum LdtkRoot {
    #[default]
    CatchAll,
}
impl LdtkRootKind for LdtkRoot {}
type LdtkSettings = LdtkSettingsGeneric<LdtkRoot>;

fn main() {
    let mut app = App::new();

    app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
    app.add_plugins(TwoDelightPlugin {
        anim_settings: default(),
        composition_settings: CompositionSettings {
            screen_size: UVec2::new(600, 600),
            ..default()
        },
        ldtk_settings: LdtkSettings::default(),
        physics_settings: PhysicsSettings::default(),
        deterministic: true,
    });

    app.add_systems(Startup, startup);
    app.add_systems(Update, update.in_set(DelightedSet));

    app.run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
#[require(Name(|| Name::new("Ground")))]
struct Ground;
#[derive(Bundle)]
struct GroundBundle {
    ground: Ground,
    pos: Pos,
    sprite: Sprite,
    static_tx: StaticTx,
}
impl GroundBundle {
    fn new(pos: Pos, size: UVec2) -> Self {
        Self {
            ground: Ground,
            pos,
            sprite: Sprite {
                custom_size: Some(size.as_vec2()),
                ..default()
            },
            static_tx: StaticTx::single(StaticTxKind::Solid, HBox::new(size.x, size.y)),
        }
    }
}

#[derive(Component)]
#[require(Name(|| Name::new("Spike")))]
struct Spike;
#[derive(Bundle)]
struct SpikeBundle {
    spike: Spike,
    pos: Pos,
    sprite: Sprite,
    trigger_tx: TriggerTx,
}
impl SpikeBundle {
    fn new(pos: Pos, size: UVec2) -> Self {
        Self {
            spike: Spike,
            pos,
            sprite: Sprite {
                custom_size: Some(size.as_vec2()),
                color: Color::linear_rgb(1.0, 0.0, 0.0),
                ..default()
            },
            trigger_tx: TriggerTx::single(TriggerTxKind::Spikes, HBox::new(size.x, size.y)),
        }
    }
}

fn startup(mut commands: Commands) {
    let player_hbox = HBox::new(36, 36);
    commands.spawn((
        Name::new("Player"),
        Player,
        Sprite {
            custom_size: Some(player_hbox.get_size().as_vec2()),
            color: Color::linear_rgb(0.1, 1.0, 0.1),
            ..default()
        },
        Pos::new(Fx::ZERO, Fx::from_num(-50)),
        Dyno::default(),
        StaticRx::single(StaticRxKind::Default, player_hbox.clone()),
        TriggerRx::single(TriggerRxKind::Player, player_hbox.clone()),
    ));

    commands.spawn(GroundBundle::new(
        Pos::new(Fx::ZERO, Fx::from_num(-300)),
        UVec2::new(800, 72),
    ));
    commands.spawn(GroundBundle::new(
        Pos::new(Fx::from_num(-300), Fx::ZERO),
        UVec2::new(200, 72),
    ));
    commands.spawn(GroundBundle::new(
        Pos::new(Fx::from_num(300), Fx::ZERO),
        UVec2::new(200, 72),
    ));

    commands.spawn(SpikeBundle::new(
        Pos::new(Fx::from_num(-300), Fx::from_num(72)),
        UVec2::new(36, 72),
    ));
    commands.spawn(SpikeBundle::new(
        Pos::new(Fx::from_num(300), Fx::from_num(72)),
        UVec2::new(36, 72),
    ));
}

fn update(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut bullet_time: ResMut<BulletTime>,
    mut player_q: Query<(&mut Pos, &mut Dyno, &mut Sprite, &StaticRx, &TriggerRx), With<Player>>,
    static_colls: Res<StaticColls>,
    trigger_colls: Res<TriggerColls>,
) {
    // Maybe toggle bullet time
    if keyboard.just_pressed(KeyCode::Space) {
        if bullet_time.get_base() == Fx::from_num(1) {
            bullet_time.set_base(Fx::from_num(0.3));
        } else {
            bullet_time.set_base(Fx::from_num(1));
        }
    }

    let (mut pos, mut dyno, mut sprite, srx, trx) = player_q.single_mut();

    // Horizontal movement
    let x_mag = Fx::from_num(200);
    dyno.vel.x = Fx::ZERO;
    if keyboard.pressed(KeyCode::KeyA) {
        dyno.vel.x -= x_mag;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        dyno.vel.x += x_mag;
    }

    // Vertical movement
    let gravity_mag = Fx::from_num(600);
    let jump_mag = Fx::from_num(400);
    dyno.vel.y -= bullet_time.delta_secs() * gravity_mag;
    if keyboard.just_pressed(KeyCode::KeyW) {
        dyno.vel.y = jump_mag;
        // Commenting out bc it feels bad but here's how to add a short-term bullet-time effect
        bullet_time.add_effect(Fx::ZERO, Fx::from_num(0.3));
    }

    // How to check for collisions
    if static_colls
        .get_refs(&srx.coll_keys)
        .any(|coll| coll.tx_kind == StaticTxKind::Solid)
    {
        sprite.color = Color::linear_rgb(0.1, 1.0, 1.0);
    } else {
        sprite.color = Color::linear_rgb(0.1, 1.0, 0.1);
    }
    if trigger_colls
        .get_refs(&trx.coll_keys)
        .any(|coll| coll.tx_kind == TriggerTxKind::Spikes)
    {
        *pos = Pos::default();
    }
}
