use bevy::prelude::*;
use camera_shake::CameraShake;

use crate::{
    layer::{
        layer::{layer_defns::SmushLayer, resize_layers_as_needed, Layer, LayerInternal},
        plugin::LayerRes,
        LayersCameraSet,
    },
    physics::prelude::{PhysicsSet, Pos},
};

pub(crate) mod camera_shake;

/// This is the component that marks the actual camera location in the world.
/// Invariants:
/// - Either 0 or 1 of these must exist at all time
/// - It must have a Pos
#[derive(Component)]
#[require(Pos)]
pub struct DynamicCamera;

/// This should be put on a camera for a layer that uses world space
#[derive(Component)]
pub(crate) struct FollowDynamicCamera;

pub(crate) fn setup_smush_camera(mut commands: Commands, layers_res: Res<LayerRes>) {
    commands
        .spawn((
            Name::new("camera"),
            Camera2d,
            Camera {
                order: SmushLayer::RENDER_ORDER as isize + 1,
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            SmushLayer::RENDER_LAYERS,
        ))
        .set_parent(layers_res.root_eid());
}

fn follow_dynamic_camera(
    dynamic_camera: Query<&Pos, With<DynamicCamera>>,
    mut followers: Query<&mut Transform, (With<FollowDynamicCamera>, Without<DynamicCamera>)>,
    camera_shake: Res<CameraShake>,
) {
    let Ok(leader) = dynamic_camera.get_single() else {
        return;
    };
    let shake_off = camera_shake.get_offset();
    for mut tran in &mut followers {
        tran.translation.x = (leader.x + shake_off.x).round() as f32;
        tran.translation.y = (leader.y + shake_off.y).round() as f32;
    }
}

pub(crate) struct LayersCameraPlugin;
impl Plugin for LayersCameraPlugin {
    fn build(&self, app: &mut App) {
        camera_shake::register_camera_shake(app);

        app.add_systems(
            Update,
            (follow_dynamic_camera, resize_layers_as_needed)
                .after(PhysicsSet)
                .in_set(LayersCameraSet),
        );
    }
}
