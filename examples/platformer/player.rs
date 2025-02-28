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
    let x_dir = Frac::whole(
        keyboard.pressed(KeyCode::KeyD) as i32 - keyboard.pressed(KeyCode::KeyA) as i32,
    );
    let y_dir = Frac::whole(
        keyboard.pressed(KeyCode::KeyW) as i32 - keyboard.pressed(KeyCode::KeyS) as i32,
    );
    player_input.dir = FVec2::new(x_dir, y_dir);
}

#[derive(Component)]
struct Player {
    /// How much time the player has left to input a jump and get a jump
    /// Kinda like a can_jump boolean but w/ coyote frames
    jump_time: Frac,
}
#[derive(Bundle)]
struct PlayerBundle {
    name: Name,
    player: Player,
    anim: AnimMan<PlayerAnim>,
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
                jump_time: Frac::ZERO,
            },
            anim: AnimMan::new(PlayerAnim::Idle),
            pos,
            dyno: Dyno::default(),
            static_rx: StaticRx::single(StaticRxKind::Default, HBox::new(8, 13)),
            trigger_rx: TriggerRx::single(TriggerRxKind::Player, HBox::new(8, 8)),
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
    dyno.vel.y -= bullet_time.delta_secs() * Frac::whole(300);
    // Jump timing
    if scolls
        .get_refs(&srx.coll_keys)
        .any(|coll| coll.push.y > Frac::ZERO && coll.tx_kind == StaticTxKind::Solid)
    {
        player.jump_time = Frac::cent(10);
    } else {
        if player.jump_time > Frac::ZERO {
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
    air_drag_cent: i8,
    ground_speed: i32,
    ground_drag_cent: i8,
    jump_speed: i32,
    max_component_speed: i32,
}
impl Default for MovementConsts {
    fn default() -> Self {
        Self {
            air_speed: 300,
            air_drag_cent: 50,
            ground_speed: 300,
            ground_drag_cent: 0,
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
    let on_ground = scolls
        .get_refs(&srx.coll_keys)
        .any(|coll| coll.push.y > Frac::ZERO && coll.tx_kind == StaticTxKind::Solid);

    // Horizontal movement
    match anim.get_state() {
        PlayerAnim::Air | PlayerAnim::Jump => {
            if input.dir.x < Frac::ZERO {
                if dyno.vel.x > Frac::ZERO {
                    dyno.vel.x *= Frac::cent(consts.air_drag_cent);
                }
                dyno.vel.x -= Frac::whole(consts.air_speed) * bullet_time.delta_secs();
            } else if input.dir.x > Frac::ZERO {
                if dyno.vel.x < Frac::ZERO {
                    dyno.vel.x *= Frac::cent(consts.air_drag_cent);
                }
                dyno.vel.x += Frac::whole(consts.air_speed) * bullet_time.delta_secs();
            } else {
                dyno.vel.x *= Frac::cent(consts.air_drag_cent);
            }
        }
        PlayerAnim::Idle | PlayerAnim::Run | PlayerAnim::Land => {
            if input.dir.x < Frac::ZERO {
                if dyno.vel.x > Frac::ZERO {
                    dyno.vel.x *= Frac::cent(consts.ground_drag_cent);
                }
                dyno.vel.x -= Frac::whole(consts.ground_speed) * bullet_time.delta_secs();
            } else if input.dir.x > Frac::ZERO {
                if dyno.vel.x < Frac::ZERO {
                    dyno.vel.x *= Frac::cent(consts.ground_drag_cent);
                }
                dyno.vel.x += Frac::whole(consts.ground_speed) * bullet_time.delta_secs();
            } else {
                dyno.vel.x *= Frac::cent(consts.ground_drag_cent);
            }
        }
    }
    // Jumping
    if player.jump_time > Frac::ZERO
        && input.jump
        && matches!(
            anim.get_state(),
            PlayerAnim::Idle | PlayerAnim::Run | PlayerAnim::Land
        )
    {
        anim.set_state(PlayerAnim::Jump);
        dyno.vel.y = Frac::whole(consts.jump_speed);
        player.jump_time = Frac::ZERO;
    }
    // State transitions
    match anim.get_state() {
        PlayerAnim::Air => {
            if on_ground {
                anim.set_state(PlayerAnim::Land);
            } else {
                anim.set_flip_x(dyno.vel.x < Frac::ZERO);
            }
        }
        PlayerAnim::Idle => {
            if dyno.vel.x != Frac::ZERO {
                anim.set_state(PlayerAnim::Run);
                anim.set_flip_x(dyno.vel.x < Frac::ZERO);
            }
        }
        PlayerAnim::Run => {
            if input.dir.x == Frac::ZERO {
                anim.set_state(PlayerAnim::Idle);
            } else {
                anim.set_flip_x(dyno.vel.x < Frac::ZERO);
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
    if dyno.vel.x.abs() > Frac::whole(consts.max_component_speed) {
        dyno.vel.x = dyno.vel.x.signum() * Frac::whole(consts.max_component_speed);
    }
    if dyno.vel.y.abs() > Frac::whole(consts.max_component_speed) {
        dyno.vel.y = dyno.vel.y.signum() * Frac::whole(consts.max_component_speed);
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
            .after(PhysicsSet),
    );
}
