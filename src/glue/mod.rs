use bevy::prelude::*;

pub mod bullet_time;
pub mod fvec;

pub type Fx = fixed::types::I32F32;

#[derive(Resource)]
pub struct Deterministic(pub bool);

pub fn color_as_vec4(color: Color) -> Vec4 {
    let linear = color.to_linear();
    Vec4::new(linear.red, linear.green, linear.blue, 1.0)
}

pub mod prelude {
    pub use super::bullet_time::*;
    pub use super::fvec::*;
    pub use super::Deterministic;
    pub use super::Fx;
}
