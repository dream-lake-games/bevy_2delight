use bevy::prelude::*;
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
    let x_dir = Fx::from_num(
        keyboard.pressed(KeyCode::KeyD) as i32 - keyboard.pressed(KeyCode::KeyA) as i32,
    );
    let y_dir = Fx::from_num(
        keyboard.pressed(KeyCode::KeyW) as i32 - keyboard.pressed(KeyCode::KeyS) as i32,
    );
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
            pos: pos.with_z(Fx::from_num(10)),
            dyno: Dyno::default(),
            static_rx: StaticRx::single(
                StaticRxKind::Default,
                HBox::new(7, 12).with_offset(Fx::from_num(0), Fx::from_num(-1)),
            ),
            trigger_rx: TriggerRx::single(
                TriggerRxKind::Player,
                HBox::new(7, 12).with_offset(Fx::from_num(0), Fx::from_num(-1)),
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
    let Ok((eid, mut dyno, mut player, srx, trx)) = player_q.get_single_mut() else {
        return;
    };
    // Gravity
    dyno.vel.y -= bullet_time.delta_secs() * Fx::from_num(300);
    // Jump timing
    if scolls.get_refs(&srx.coll_keys).any(|coll| {
        coll.push.y > Fx::ZERO && matches!(coll.tx_kind, StaticTxKind::Solid | StaticTxKind::PassUp)
    }) {
        player.jump_time = Fx::from_num(0.1);
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
        commands.entity(eid).despawn_recursive();
    }
    // Die intentionally
    if keyboard.just_pressed(KeyCode::Backspace) {
        commands.entity(eid).despawn_recursive();
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
    let Ok((mut anim, mut dyno, mut player, srx)) = player_q.get_single_mut() else {
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
                    dyno.vel.x *= Fx::from_num(consts.air_drag);
                }
                dyno.vel.x -= Fx::from_num(consts.air_speed) * bullet_time.delta_secs();
            } else if input.dir.x > Fx::ZERO {
                if dyno.vel.x < Fx::ZERO {
                    dyno.vel.x *= Fx::from_num(consts.air_drag);
                }
                dyno.vel.x += Fx::from_num(consts.air_speed) * bullet_time.delta_secs();
            } else {
                dyno.vel.x *= Fx::from_num(consts.air_drag);
            }
        }
        PlayerAnim::Idle | PlayerAnim::Run | PlayerAnim::Land => {
            if input.dir.x < Fx::ZERO {
                if dyno.vel.x > Fx::ZERO {
                    dyno.vel.x *= Fx::from_num(consts.ground_drag);
                }
                dyno.vel.x -= Fx::from_num(consts.ground_speed) * bullet_time.delta_secs();
            } else if input.dir.x > Fx::ZERO {
                if dyno.vel.x < Fx::ZERO {
                    dyno.vel.x *= Fx::from_num(consts.ground_drag);
                }
                dyno.vel.x += Fx::from_num(consts.ground_speed) * bullet_time.delta_secs();
            } else {
                dyno.vel.x *= Fx::from_num(consts.ground_drag);
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
        dyno.vel.y = Fx::from_num(consts.jump_speed);
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
    if dyno.vel.x.abs() > Fx::from_num(consts.max_component_speed) {
        dyno.vel.x = dyno.vel.x.signum() * Fx::from_num(consts.max_component_speed);
    }
    if dyno.vel.y.abs() > Fx::from_num(consts.max_component_speed) {
        dyno.vel.y = dyno.vel.y.signum() * Fx::from_num(consts.max_component_speed);
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
        _fields: &bevy::utils::HashMap<String, bevy_ecs_ldtk::prelude::FieldValue>,
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
    let Ok(spawn_pos) = spawner_q.get_single() else {
        return;
    };
    if player_q.is_empty() {
        commands
            .spawn(PlayerBundle::new(*spawn_pos))
            .set_parent(ldtk_roots.get_eid(LdtkRoot::Player));
    }
}

pub(super) fn regiser_player(app: &mut App) {
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
        )
            .chain()
            .in_set(DelightedSet),
    );
}
