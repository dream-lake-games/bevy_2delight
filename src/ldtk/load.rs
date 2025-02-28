use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkProjectHandle, LdtkWorldBundle, LevelSelection};

use super::prelude::{LdtkRootKind, LdtkRootResGeneric};

/// The LDTK spawning state
/// The API only exposes commands that will go:
/// - Unloaded -> Loading
/// - Loaded -> Unloading
/// Everything else is handled internally. You should gate your gameplay systems
/// on in_state(LdtkState::Loaded).
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq, Reflect, States)]
pub enum LdtkState {
    #[default]
    Unloaded,
    Loading,
    Loaded,
    Unloading,
}

/// TODO: Find a better way to do this. Rn I basically just stall loading everything
/// ldtk spawns by a certain number of frames, should instead actually look at handles, etc.
#[derive(Component)]
pub(super) struct BlockLdtkLoad {
    ticks: u32,
}
impl BlockLdtkLoad {
    pub(super) fn ticks(ticks: u32) -> Self {
        Self { ticks }
    }
}
impl Default for BlockLdtkLoad {
    fn default() -> Self {
        Self::ticks(10)
    }
}

#[derive(Event)]
pub struct LoadLdtk {
    world_path: String,
    level_lid: String,
}
impl LoadLdtk {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(world_path: S1, level_lid: S2) -> Self {
        Self {
            world_path: world_path.as_ref().to_string(),
            level_lid: level_lid.as_ref().to_string(),
        }
    }
}
fn handle_load_ldtk(
    trigger: Trigger<LoadLdtk>,
    mut commands: Commands,
    state: Res<State<LdtkState>>,
    mut next_state: ResMut<NextState<LdtkState>>,
    ass: Res<AssetServer>,
) {
    if !matches!(state.get(), LdtkState::Unloaded) {
        warn!("Can't issue LoadLdtk when state is {:?}", state.get());
        return;
    };
    commands.spawn((
        Name::new("LdtkRoot"),
        LdtkWorldBundle {
            ldtk_handle: LdtkProjectHandle {
                handle: ass.load(&trigger.event().world_path),
            },
            ..default()
        },
    ));
    commands.insert_resource(LevelSelection::iid(&trigger.event().level_lid));
    next_state.set(LdtkState::Loading);
}

fn update_load_ldtk(
    mut commands: Commands,
    mut next_state: ResMut<NextState<LdtkState>>,
    mut blockers: Query<(Entity, &mut BlockLdtkLoad)>,
) {
    if !blockers.is_empty() {
        for (eid, mut blocker) in &mut blockers {
            if blocker.ticks == 0 {
                commands.entity(eid).remove::<BlockLdtkLoad>();
            } else {
                blocker.ticks -= 1;
            }
        }
    } else {
        next_state.set(LdtkState::Loaded);
    }
}

#[derive(Event, Default)]
pub struct UnloadLdtk<R: LdtkRootKind> {
    _pd: std::marker::PhantomData<R>,
}
fn handle_unload_ldtk<R: LdtkRootKind>(
    _trigger: Trigger<UnloadLdtk<R>>,
    mut commands: Commands,
    state: Res<State<LdtkState>>,
    mut next_state: ResMut<NextState<LdtkState>>,
    existing_roots: Query<Entity, With<LdtkProjectHandle>>,
    roots: Res<LdtkRootResGeneric<R>>,
) {
    if !matches!(state.get(), LdtkState::Loaded) {
        warn!("Can't issue UnloadLdtk when state is {:?}", state.get());
        return;
    }
    for project_root in &existing_roots {
        commands.entity(project_root).despawn_recursive();
    }
    for logical_root in R::iter() {
        commands
            .entity(roots.get_eid(logical_root))
            .despawn_descendants();
    }
    next_state.set(LdtkState::Unloading);
}

fn update_unload_ldtk<R: LdtkRootKind>(
    project_roots: Query<Entity, With<LdtkProjectHandle>>,
    mut next_state: ResMut<NextState<LdtkState>>,
    children_q: Query<&Children>,
    roots: Res<LdtkRootResGeneric<R>>,
) {
    if !project_roots.is_empty() {
        return;
    }
    for logical_root in R::iter() {
        let num_children = children_q
            .get(roots.get_eid(logical_root))
            .map(|c| c.len())
            .unwrap_or(0);
        if num_children > 0 {
            return;
        }
    }
    next_state.set(LdtkState::Unloaded);
}

pub(super) fn register_load<R: LdtkRootKind>(app: &mut App) {
    app.add_observer(handle_load_ldtk);
    app.add_observer(handle_unload_ldtk::<R>);

    app.add_systems(
        Update,
        update_load_ldtk.run_if(in_state(LdtkState::Loading)),
    );
    app.add_systems(
        Update,
        update_unload_ldtk::<R>.run_if(in_state(LdtkState::Unloading)),
    );
}
