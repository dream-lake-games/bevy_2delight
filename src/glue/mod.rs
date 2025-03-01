use bevy::ecs::system::Resource;

pub mod bullet_time;
pub mod fvec;

pub type Fx = fixed::types::I32F32;

#[derive(Resource)]
pub struct Deterministic(pub bool);

pub mod prelude {
    pub use super::bullet_time::*;
    pub use super::fvec::*;
    pub use super::Deterministic;
    pub use super::Fx;
}
