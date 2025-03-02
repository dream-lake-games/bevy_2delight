use bevy::prelude::*;

use crate::prelude::*;

use super::camera::follow_dynamic_camera;

#[derive(Component)]
#[require(Pos)]
pub struct ParallaxX {
    mult: Fx,
    wrap_size: Option<Fx>,
}
impl ParallaxX {
    pub fn wrapped(mult: Fx, wrap: Fx) -> Self {
        Self {
            mult,
            wrap_size: Some(wrap),
        }
    }
    pub fn new_unwrapped(mult: Fx) -> Self {
        Self {
            mult,
            wrap_size: None,
        }
    }
}

#[derive(Component)]
#[require(Pos)]
pub struct ParallaxY {
    mult: Fx,
    wrap_size: Option<Fx>,
}
impl ParallaxY {
    pub fn new_wrapped(mult: Fx, wrap: Fx) -> Self {
        Self {
            mult,
            wrap_size: Some(wrap),
        }
    }
    pub fn new_unwrapped(mult: Fx) -> Self {
        Self {
            mult,
            wrap_size: None,
        }
    }
}

fn reposition_parallax_x(
    mut px_q: Query<(&Pos, &ParallaxX, &mut Transform)>,
    cam_q: Query<&Pos, With<DynamicCamera>>,
) {
    let Ok(cam_pos) = cam_q.get_single() else {
        return;
    };
    for (px_pos, px_def, mut tran) in &mut px_q {
        let mut diff = (px_pos.x - cam_pos.x) * px_def.mult;
        if let Some(wrap_size) = px_def.wrap_size {
            diff += wrap_size / Fx::from_num(2);
            diff = diff.rem_euclid(wrap_size);
            diff -= wrap_size / Fx::from_num(2);
        }
        tran.translation.x = diff.to_num();
    }
}

fn reposition_parallax_y(
    mut py_q: Query<(&Pos, &ParallaxY, &mut Transform)>,
    cam_q: Query<&Pos, With<DynamicCamera>>,
) {
    let Ok(cam_pos) = cam_q.get_single() else {
        return;
    };
    for (py_pos, py_def, mut tran) in &mut py_q {
        let mut diff = (py_pos.y - cam_pos.y) * py_def.mult;
        if let Some(wrap_size) = py_def.wrap_size {
            diff += wrap_size / Fx::from_num(2);
            diff = diff.rem_euclid(wrap_size);
            diff -= wrap_size / Fx::from_num(2);
        }
        tran.translation.y = diff.to_num();
    }
}

pub(super) fn register_parallax(app: &mut App) {
    app.add_systems(
        Update,
        (reposition_parallax_x, reposition_parallax_y)
            .after(follow_dynamic_camera)
            .in_set(LayersCameraSet),
    );
}
