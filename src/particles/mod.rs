use bevy::ecs::schedule::SystemSet;

mod particle_defn;
mod particle_logic;
mod particle_plugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ParticleSet;

pub mod prelude {
    pub use super::particle_defn::Particle;
    pub(crate) use super::particle_plugin::ParticlePlugin;
}
