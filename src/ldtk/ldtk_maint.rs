use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::LdtkSet;

use crate::prelude::*;

#[derive(Resource, Clone, Debug, Reflect)]
pub struct LdtkLevelRects {
    map: HashMap<String, Rect>,
}
impl LdtkLevelRects {
    pub fn get<S: AsRef<str>>(&self, level_lid: S) -> Option<&Rect> {
        self.map.get(level_lid.as_ref())
    }
}

pub(super) fn update_level_rects(
    levels: Query<(&LevelIid, &GlobalTransform)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_rects: ResMut<LdtkLevelRects>,
) {
    let Ok(project) = ldtk_projects.single() else {
        return;
    };
    let Some(ldtk_project) = ldtk_project_assets.get(project) else {
        return;
    };
    level_rects.map.clear();
    for (level_lid, level_transform) in levels.iter() {
        let level = ldtk_project
            .get_raw_level_by_iid(level_lid.get())
            .expect("level should exist in only project");
        let level_bounds = Rect {
            min: Vec2::new(
                level_transform.translation().x,
                level_transform.translation().y,
            ),
            max: Vec2::new(
                level_transform.translation().x + level.px_wid as f32,
                level_transform.translation().y + level.px_hei as f32,
            ),
        };
        level_rects
            .map
            .insert(level_lid.as_str().to_string(), level_bounds);
    }
}

pub(super) fn register_ldtk_maint(app: &mut App) {
    app.insert_resource(LdtkLevelRects { map: default() });
    app.add_systems(Update, update_level_rects.in_set(LdtkSet));
}
