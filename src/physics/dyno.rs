use bevy::prelude::*;

use crate::glue::{frac::Frac, fvec::FVec2};

#[derive(Component, Clone, Debug, Default)]
#[require(crate::physics::pos::Pos)]
pub struct Dyno {
    pub vel: FVec2,
}
impl Dyno {
    pub fn new(x: Frac, y: Frac) -> Self {
        Self {
            vel: FVec2::new(x, y),
        }
    }
}
