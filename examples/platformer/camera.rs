use bevy::prelude::*;
use bevy_2delight::prelude::*;

use crate::player::Player;

fn startup_camera(mut commands: Commands) {
    commands.spawn((Name::new("DynamicCamera"), DynamicCamera, Pos::default()));
}

fn update_camera(
    mut cam_pos_q: Query<&mut Pos, With<DynamicCamera>>,
    player_pos_q: Query<&Pos, (With<Player>, Without<DynamicCamera>)>,
) {
    let mut cam_pos = cam_pos_q.single_mut();
    if let Ok(player_pos) = player_pos_q.get_single() {
        *cam_pos = *player_pos;
    }
}

pub(super) fn register_camera(app: &mut App) {
    app.add_systems(Startup, startup_camera);
    app.add_systems(Update, update_camera.in_set(DelightedSet));
}
