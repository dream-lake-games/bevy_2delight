use bevy::prelude::*;
use rand::Rng;
use std::ops::RangeInclusive;

use crate::prelude::*;
use crate::prelude::{BulletTime, FVec2, Fx};

/// This is the component that marks the actual camera location in the world.
/// Invariants:
/// - Either 0 or 1 of these must exist at all time
/// - It must have a Pos
#[derive(Component)]
#[require(Pos)]
pub struct DynamicCamera;

/// The individual layer cameras that follow the dynamic camera
#[derive(Component)]
pub(super) struct FollowDynamicCamera;

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
        tran.translation.x = (leader.x + shake_off.x).round().to_num();
        tran.translation.y = (leader.y + shake_off.y).round().to_num();
    }
}

#[derive(Clone, Debug)]
struct CameraShakeSpec {
    time_left: Fx,
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

#[derive(Resource, Debug)]
pub struct CameraShake {
    specs: Vec<CameraShakeSpec>,
    offset: FVec2,
    time_since_last_update: Fx,
}
impl CameraShake {
    pub fn add_shake(
        &mut self,
        time: Fx,
        x_range: RangeInclusive<i32>,
        y_range: RangeInclusive<i32>,
    ) {
        self.specs.push(CameraShakeSpec {
            time_left: time,
            x_range,
            y_range,
        });
    }

    pub fn clear(&mut self) {
        self.specs.clear();
        self.offset = FVec2::ZERO;
    }

    pub(crate) fn get_offset(&self) -> FVec2 {
        self.offset
    }
}

fn update_camera_shake(mut camera_shake: ResMut<CameraShake>, bullet_time: Res<BulletTime>) {
    // Obey SHAKE_EVERY
    camera_shake.time_since_last_update += bullet_time.real_delta_secs();
    let shake_every = Fx::from_num(0.04);
    if camera_shake.time_since_last_update < shake_every {
        return;
    }
    camera_shake.time_since_last_update = Fx::ZERO;

    // Calculate offset
    let mut offset = FVec2::ZERO;
    let mut rng = rand::thread_rng();
    for spec in &mut camera_shake.specs {
        spec.time_left -= shake_every;
        offset.x += Fx::from_num(rng.gen_range(spec.x_range.clone()));
        offset.y += Fx::from_num(rng.gen_range(spec.y_range.clone()));
    }
    camera_shake.offset = offset;

    // Cleanup specs
    camera_shake.specs.retain(|spec| spec.time_left > Fx::ZERO);
}

pub(super) fn register_camera(app: &mut App) {
    app.insert_resource(CameraShake {
        specs: vec![],
        offset: FVec2::ZERO,
        time_since_last_update: Fx::ZERO,
    });

    app.add_systems(
        Update,
        (update_camera_shake, follow_dynamic_camera)
            .chain()
            .after(PhysicsSet)
            .in_set(LayersCameraSet),
    );
}
