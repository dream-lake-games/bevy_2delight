use bevy::{color::palettes::tailwind, prelude::*};
use bevy_2delight::prelude::*;

use crate::{
    ldtk::{LdtkEntityPlugin, LdtkRoot, LdtkRootRes},
    TriggerColls, TriggerRx, TriggerRxKind, TriggerTxKind,
};

defn_anim!(
    PlayerAnim,
    #[folder("platformer/lenny")]
    pub enum PlayerAnim {
        #[tag("air")]
        Air,
        #[default]
        #[tag("idle")]
        Idle,
        #[tag("jump")]
        #[next(Air)]
        #[fps(16)]
        Jump,
        #[tag("land")]
        #[next(Idle)]
        Land,
        #[tag("run")]
        #[fps(16)]
        Run,
    }
);

#[derive(Resource, Default)]
struct PlayerInput {
    jump: bool,
    dir: FVec2,
}
fn update_player_input(mut player_input: ResMut<PlayerInput>, keyboard: Res<ButtonInput<KeyCode>>) {
    player_input.jump = keyboard.just_pressed(KeyCode::KeyJ);
    let x_dir = keyboard.pressed(KeyCode::KeyD) as i32 - keyboard.pressed(KeyCode::KeyA) as i32;
    let y_dir = keyboard.pressed(KeyCode::KeyW) as i32 - keyboard.pressed(KeyCode::KeyS) as i32;
    player_input.dir = FVec2::new(x_dir, y_dir);
}

#[derive(Component)]
pub struct Player {
    /// How much time the player has left to input a jump and get a jump
    /// Kinda like a can_jump boolean but w/ coyote frames
    jump_time: Fx,
}
#[derive(Bundle)]
struct PlayerBundle {
    name: Name,
    player: Player,
    anim: AnimMan<PlayerAnim>,
    light: CircleLight,
    flicker: LightFlicker,
    pos: Pos,
    dyno: Dyno,
    static_rx: StaticRx,
    trigger_rx: TriggerRx,
}
impl PlayerBundle {
    fn new(pos: Pos) -> Self {
        Self {
            name: Name::new("Player"),
            player: Player {
                jump_time: Fx::ZERO,
            },
            anim: AnimMan::new(PlayerAnim::Idle),
            light: CircleLight::strength(64.0),
            flicker: LightFlicker::new(64.0, 2.0, 2.0, 1.0, 0.15, 0.05),
            pos: pos.with_z(10),
            dyno: Dyno::default(),
            static_rx: StaticRx::single(StaticRxKind::Default, HBox::new(7, 12).with_offset(0, -1)),
            trigger_rx: TriggerRx::single(
                TriggerRxKind::Player,
                HBox::new(7, 12).with_offset(0, -1),
            ),
        }
    }
}
/// Logic that we always perform for the player, regardless of state
fn update_player_always(
    mut player_q: Query<(Entity, &mut Dyno, &mut Player, &StaticRx, &TriggerRx)>,
    bullet_time: Res<BulletTime>,
    scolls: Res<StaticColls>,
    tcolls: Res<TriggerColls>,
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let Ok((eid, mut dyno, mut player, srx, trx)) = player_q.single_mut() else {
        return;
    };
    // Gravity
    dyno.vel.y -= bullet_time.delta_secs() * fx!(300);
    // Jump timing
    if scolls.get_refs(&srx.coll_keys).any(|coll| {
        coll.push.y > Fx::ZERO && matches!(coll.tx_kind, StaticTxKind::Solid | StaticTxKind::PassUp)
    }) {
        player.jump_time = fx!(0.1);
    } else {
        if player.jump_time > Fx::ZERO {
            player.jump_time -= bullet_time.real_delta_secs();
        }
    }
    // Die from spikes
    if tcolls
        .get_refs(&trx.coll_keys)
        .any(|coll| coll.tx_kind == TriggerTxKind::Spikes)
    {
        commands.entity(eid).despawn();
    }
    // Die intentionally
    if keyboard.just_pressed(KeyCode::Backspace) {
        commands.entity(eid).despawn();
    }
}
#[derive(Resource, Reflect)]
struct MovementConsts {
    air_speed: i32,
    air_drag: f32,
    ground_speed: i32,
    ground_drag: f32,
    jump_speed: i32,
    max_component_speed: i32,
}
impl Default for MovementConsts {
    fn default() -> Self {
        Self {
            air_speed: 300,
            air_drag: 0.5,
            ground_speed: 300,
            ground_drag: 0.1,
            jump_speed: 100,
            max_component_speed: 100,
        }
    }
}
/// Logic for updating the player, matched on state
fn update_player_stateful(
    mut player_q: Query<(&mut AnimMan<PlayerAnim>, &mut Dyno, &mut Player, &StaticRx)>,
    scolls: Res<StaticColls>,
    input: Res<PlayerInput>,
    bullet_time: Res<BulletTime>,
    consts: Res<MovementConsts>,
) {
    let Ok((mut anim, mut dyno, mut player, srx)) = player_q.single_mut() else {
        return;
    };
    let on_ground = scolls.get_refs(&srx.coll_keys).any(|coll| {
        coll.push.y > Fx::ZERO && matches!(coll.tx_kind, StaticTxKind::Solid | StaticTxKind::PassUp)
    });

    // Horizontal movement
    match anim.get_state() {
        PlayerAnim::Air | PlayerAnim::Jump => {
            if input.dir.x < Fx::ZERO {
                if dyno.vel.x > Fx::ZERO {
                    dyno.vel.x *= fx!(consts.air_drag);
                }
                dyno.vel.x -= fx!(consts.air_speed) * bullet_time.delta_secs();
            } else if input.dir.x > Fx::ZERO {
                if dyno.vel.x < Fx::ZERO {
                    dyno.vel.x *= fx!(consts.air_drag);
                }
                dyno.vel.x += fx!(consts.air_speed) * bullet_time.delta_secs();
            } else {
                dyno.vel.x *= fx!(consts.air_drag);
            }
        }
        PlayerAnim::Idle | PlayerAnim::Run | PlayerAnim::Land => {
            if input.dir.x < Fx::ZERO {
                if dyno.vel.x > Fx::ZERO {
                    dyno.vel.x *= fx!(consts.ground_drag);
                }
                dyno.vel.x -= fx!(consts.ground_speed) * bullet_time.delta_secs();
            } else if input.dir.x > Fx::ZERO {
                if dyno.vel.x < Fx::ZERO {
                    dyno.vel.x *= fx!(consts.ground_drag);
                }
                dyno.vel.x += fx!(consts.ground_speed) * bullet_time.delta_secs();
            } else {
                dyno.vel.x *= fx!(consts.ground_drag);
            }
        }
    }
    // Jumping
    if player.jump_time > Fx::ZERO
        && input.jump
        && matches!(
            anim.get_state(),
            PlayerAnim::Idle | PlayerAnim::Run | PlayerAnim::Land
        )
    {
        anim.set_state(PlayerAnim::Jump);
        dyno.vel.y = fx!(consts.jump_speed);
        player.jump_time = Fx::ZERO;
    }
    // State transitions
    match anim.get_state() {
        PlayerAnim::Air => {
            if on_ground {
                anim.set_state(PlayerAnim::Land);
            } else if dyno.vel.x != Fx::ZERO {
                anim.set_flip_x(dyno.vel.x < Fx::ZERO);
            }
        }
        PlayerAnim::Idle => {
            if input.dir.x != Fx::ZERO {
                anim.set_state(PlayerAnim::Run);
                anim.set_flip_x(dyno.vel.x < Fx::ZERO);
            }
            if !on_ground {
                anim.set_state(PlayerAnim::Air);
            }
        }
        PlayerAnim::Run => {
            if input.dir.x == Fx::ZERO {
                anim.set_state(PlayerAnim::Idle);
            } else {
                anim.set_flip_x(dyno.vel.x < Fx::ZERO);
            }
            if !on_ground {
                anim.set_state(PlayerAnim::Air);
            }
        }
        PlayerAnim::Jump => {
            // Do nothing. We always finish the jump anim.
        }
        PlayerAnim::Land => {
            // Do nothing. We always finish the land anim unless pulled out to jump above.
        }
    }
    // Limit component speed (TODO: Fixed point square root...)
    if dyno.vel.x.abs() > fx!(consts.max_component_speed) {
        dyno.vel.x = dyno.vel.x.signum() * fx!(consts.max_component_speed);
    }
    if dyno.vel.y.abs() > fx!(consts.max_component_speed) {
        dyno.vel.y = dyno.vel.y.signum() * fx!(consts.max_component_speed);
    }
}

#[derive(Component)]
struct PlayerSpawner;
#[derive(Bundle)]
struct PlayerSpawnerBundle {
    name: Name,
    marker: PlayerSpawner,
    pos: Pos,
}
impl LdtkEntity<LdtkRoot> for PlayerSpawnerBundle {
    const ROOT: LdtkRoot = LdtkRoot::Player;
    fn from_ldtk(
        pos: Pos,
        _fields: &bevy::platform::collections::HashMap<String, bevy_ecs_ldtk::prelude::FieldValue>,
        _iid: String,
    ) -> Self {
        Self {
            name: Name::new("PlayerSpawner"),
            marker: PlayerSpawner,
            pos,
        }
    }
}
fn update_player_spawner(
    player_q: Query<Entity, With<Player>>,
    mut commands: Commands,
    spawner_q: Query<&Pos, With<PlayerSpawner>>,
    ldtk_roots: Res<LdtkRootRes>,
) {
    let Ok(spawn_pos) = spawner_q.single() else {
        return;
    };
    if player_q.is_empty() {
        commands
            .spawn(PlayerBundle::new(*spawn_pos))
            .insert(ChildOf(ldtk_roots.get_eid(LdtkRoot::Player)));
    }
}

fn player_juice(
    player_q: Query<(&Pos, &Dyno, &AnimMan<PlayerAnim>), With<Player>>,
    mut commands: Commands,
) {
    let Ok((pos, dyno, anim)) = player_q.single() else {
        return;
    };

    // Glowing trail
    commands.spawn(
        Particle::new(*pos, 0.3)
            .with_pos_fuzz(0.75, 1.5)
            .with_lifetime_fuzz(0.1)
            .with_vel_fuzz(4, 4)
            .with_color_terp(
                Color::srgb_u8(238, 191, 245),
                tailwind::BLUE_400.into(),
                TerpMode::Linear,
            )
            .with_color_fuzz(Color::srgb(0.1, 0.1, 0.1))
            .with_brightness_terp(
                Color::srgb_u8(238, 191, 245),
                tailwind::BLUE_400.into(),
                TerpMode::Linear,
            )
            .with_size_terp(4, 1, TerpMode::Linear)
            .with_size_fuzz(0.5)
            .with_layer(Layer::BackDetailPixels),
    );

    // State transition particles
    let base_ground_particle = Particle::new(*pos - FVec2::new(1, 6), 0.8)
        .with_pos_fuzz(1.0, 0.0)
        .with_lifetime_fuzz(0.1)
        .with_color_constant(Color::srgb_u8(158, 129, 208))
        .with_gravity(150)
        .with_collision(StaticRxKind::Bounce {
            perp: fx!(0),
            par: fx!(0.9),
        })
        .with_layer(Layer::FrontDetailPixels);

    if let Some(anim_state_change) = anim.delta_state() {
        match (anim_state_change.last_frame, anim_state_change.this_frame) {
            (_, PlayerAnim::Land) => {
                let part = base_ground_particle
                    .clone()
                    .with_vel(FVec2::new(dyno.vel.x / 10, 20))
                    .with_vel_fuzz(5.0, 3.0);
                for _ in 0..3 {
                    commands.spawn(part.clone());
                }
            }
            (_, PlayerAnim::Jump) => {
                let part = base_ground_particle
                    .clone()
                    .with_vel(FVec2::new(dyno.vel.x / 10, 30))
                    .with_vel_fuzz(5.0, 3.0);
                for _ in 0..6 {
                    commands.spawn(part.clone());
                }
            }
            _ => (),
        }
    }

    // Mid-animation particles
    match (anim.get_state(), anim.get_ix()) {
        (PlayerAnim::Run, 3) => {
            let part = base_ground_particle
                .clone()
                .with_vel(FVec2::new(dyno.vel.x / 8, 20))
                .with_vel_fuzz(2.0, 2.0);
            for _ in 0..2 {
                commands.spawn(part.clone());
            }
        }
        _ => (),
    }
}

pub(super) fn register_player(app: &mut App) {
    app.add_plugins(LdtkEntityPlugin::<PlayerSpawnerBundle>::new(
        "Entities",
        "PlayerSpawner",
    ));

    app.insert_resource(PlayerInput::default());
    app.insert_resource(MovementConsts::default());
    debug_resource!(app, MovementConsts);

    app.add_systems(
        Update,
        (
            update_player_input,
            update_player_always,
            update_player_stateful,
            update_player_spawner,
            player_juice,
        )
            .chain()
            .in_set(DelightedSet),
    );
}
