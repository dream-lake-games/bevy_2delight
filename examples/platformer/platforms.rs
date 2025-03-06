use bevy::prelude::*;
use bevy_2delight::prelude::*;

use crate::ldtk::{LdtkIntCellValuePlugin, LdtkRoot, LdtkRootRes};

defn_anim!(
    FallingPlatformAnim,
    #[folder("platformer/world/platforms")]
    pub enum FallingPlatformAnim {
        #[default]
        #[tag("falling_still")]
        Still,
    }
);

#[derive(Component)]
struct FallingPlatformSpawner {
    time_till_spawn: Option<Fx>,
}
#[derive(Bundle)]
struct FallingPlatformerSpawnerBundle {
    name: Name,
    pos: Pos,
    marker: FallingPlatformSpawner,
}
impl LdtkIntCellValue<LdtkRoot> for FallingPlatformerSpawnerBundle {
    const ROOT: LdtkRoot = LdtkRoot::Platforms;
    fn from_ldtk(pos: Pos, _value: i32) -> Self {
        Self {
            name: Name::new("FallingPlatformSpawner"),
            pos,
            marker: FallingPlatformSpawner {
                time_till_spawn: Some(Fx::ZERO),
            },
        }
    }
}

#[derive(Component)]
struct FallingPlatform {
    spawned_by: Entity,
}
#[derive(Bundle)]
struct FallingPlatformBundle {
    name: Name,
    marker: FallingPlatform,
    anim: AnimMan<FallingPlatformAnim>,
    stx: StaticTx,
    pos: Pos,
}
impl FallingPlatformBundle {
    fn new(pos: Pos, spawned_by: Entity) -> Self {
        Self {
            name: Name::new("FallingPlatform"),
            marker: FallingPlatform { spawned_by },
            anim: default(),
            stx: StaticTx::single(
                StaticTxKind::PassUp,
                HBox::new(8, 2).with_offset(Fx::ZERO, Fx::from_num(3)),
            ),
            pos,
        }
    }
}

fn update_falling_platform_spawners(
    bullet_time: Res<BulletTime>,
    mut spawners_q: Query<(Entity, &Pos, &mut FallingPlatformSpawner)>,
    mut commands: Commands,
    ldtk_roots: Res<LdtkRootRes>,
) {
    for (eid, pos, mut spawner) in &mut spawners_q {
        let Some(countdown) = spawner.time_till_spawn.as_mut() else {
            continue;
        };
        *countdown -= bullet_time.delta_secs();
        if *countdown > Fx::ZERO {
            continue;
        }
        spawner.time_till_spawn = None;
        commands
            .spawn(FallingPlatformBundle::new(*pos, eid))
            .set_parent(ldtk_roots.get_eid(LdtkRoot::Platforms));
    }
}

fn update_falling_platforms(
    mut spawners: Query<&mut FallingPlatformSpawner>,
    waiting_to_fall: Query<(Entity, &FallingPlatform, &StaticTx), Without<Dyno>>,
    mut falling: Query<&mut Dyno, With<FallingPlatform>>,
    static_colls: Res<StaticColls>,
    mut commands: Commands,
    bullet_time: Res<BulletTime>,
) {
    for (eid, falling_platform, stx) in &waiting_to_fall {
        if static_colls
            .get_refs(&stx.coll_keys)
            .any(|coll| coll.rx_kind == StaticRxKind::Default)
        {
            commands.entity(eid).insert(Dyno::default());
            let mut spawner = spawners.get_mut(falling_platform.spawned_by).unwrap();
            spawner.time_till_spawn = Some(Fx::from_num(1));
        }
    }
    for mut dyno in &mut falling {
        dyno.vel.y -= bullet_time.delta_secs() * Fx::from_num(150);
        dyno.vel.y = dyno.vel.y.max(Fx::from_num(-75));
    }
}

pub(super) fn register_platforms(app: &mut App) {
    app.register_ldtk_int_cell_layer("Platforms", Layer::Dummy);

    app.add_plugins(
        LdtkIntCellValuePlugin::<FallingPlatformerSpawnerBundle>::single("Platforms", 1),
    );

    app.add_systems(
        Update,
        (update_falling_platform_spawners, update_falling_platforms)
            .chain()
            .in_set(DelightedSet),
    );
}
