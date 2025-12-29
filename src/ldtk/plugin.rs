use bevy::prelude::*;
use bevy_ecs_ldtk::{LevelBackground, LevelSpawnBehavior};

use super::{ldtk_load::LdtkState, ldtk_roots::LdtkRootKind};

#[derive(Default)]
pub struct LdtkSettingsGeneric<R: LdtkRootKind> {
    _pd: std::marker::PhantomData<R>,
}

#[derive(Default)]
pub(crate) struct LdtkPlugin<R: LdtkRootKind> {
    _pd: std::marker::PhantomData<R>,
}
impl<R: LdtkRootKind> Plugin for LdtkPlugin<R> {
    fn build(&self, app: &mut App) {
        app.insert_state(LdtkState::Unloaded);

        super::ldtk_roots::register_ldtk_root::<R>(app);
        super::ldtk_int_cell::register_ldtk_int_cell(app);
        super::ldtk_maint::register_ldtk_maint(app);
        super::ldtk_load::register_load::<R>(app);

        app.add_plugins(bevy_ecs_ldtk::LdtkPlugin)
            .insert_resource(bevy_ecs_ldtk::LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                level_background: LevelBackground::Nonexistent,
                ..default()
            });
    }
}
