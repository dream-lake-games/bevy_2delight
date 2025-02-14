use bevy::prelude::*;

use crate::{
    layer::{camera::DynamicCamera, LayersCameraSet},
    prelude::{Frac, Pos},
};

#[derive(Component)]
#[require(Pos)]
pub struct ParallaxX {
    mult: Frac,
    wrap_size: Option<Frac>,
}
impl ParallaxX {
    pub fn new_wrapped(mult: Frac, wrap: Frac) -> Self {
        Self {
            mult,
            wrap_size: Some(wrap),
        }
    }
    pub fn new_unwrapped(mult: Frac) -> Self {
        Self {
            mult,
            wrap_size: None,
        }
    }
}

#[derive(Component)]
#[require(Pos)]
pub struct ParallaxY {
    mult: Frac,
    wrap_size: Option<Frac>,
}
impl ParallaxY {
    pub fn new_wrapped(mult: Frac, wrap: Frac) -> Self {
        Self {
            mult,
            wrap_size: Some(wrap),
        }
    }
    pub fn new_unwrapped(mult: Frac) -> Self {
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
            diff += wrap_size / Frac::whole(2);
            diff = diff.rem_euclid(wrap_size);
            diff -= wrap_size / Frac::whole(2);
        }
        tran.translation.x = diff.as_f32();
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
            diff += wrap_size / Frac::whole(2);
            diff = diff.rem_euclid(wrap_size);
            diff -= wrap_size / Frac::whole(2);
        }
        tran.translation.y = diff.as_f32();
    }
}

pub(crate) struct LayersParallaxPlugin;
impl Plugin for LayersParallaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (reposition_parallax_x, reposition_parallax_y).after(LayersCameraSet),
        );
    }
}
