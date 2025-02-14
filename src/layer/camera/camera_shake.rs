use bevy::prelude::*;
use rand::Rng;

use std::ops::RangeInclusive;

use crate::prelude::{BulletTime, FVec2, Frac};

const SHAKE_EVERY: Frac = Frac::const_cent(4);

#[derive(Clone, Debug, Reflect)]
struct CameraShakeSpec {
    time_left: Frac,
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

#[derive(Resource, Debug, Reflect)]
pub struct CameraShake {
    specs: Vec<CameraShakeSpec>,
    offset: FVec2,
    time_since_last_update: Frac,
}
impl CameraShake {
    pub fn add_shake(
        &mut self,
        time: Frac,
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
    if camera_shake.time_since_last_update < SHAKE_EVERY {
        return;
    }
    camera_shake.time_since_last_update = Frac::ZERO;

    // Calculate offset
    let mut offset = FVec2::ZERO;
    let mut rng = rand::thread_rng();
    for spec in &mut camera_shake.specs {
        spec.time_left -= SHAKE_EVERY;
        offset.x += Frac::whole(rng.gen_range(spec.x_range.clone()));
        offset.y += Frac::whole(rng.gen_range(spec.y_range.clone()));
    }
    camera_shake.offset = offset;

    // Cleanup specs
    camera_shake
        .specs
        .retain(|spec| spec.time_left > Frac::ZERO);
}

pub(super) fn register_camera_shake(app: &mut App) {
    app.insert_resource(CameraShake {
        specs: vec![],
        offset: FVec2::ZERO,
        time_since_last_update: Frac::ZERO,
    });

    app.add_systems(Update, update_camera_shake);
}
