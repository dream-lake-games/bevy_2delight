use bevy::prelude::*;

pub(crate) struct ParticlePlugin;
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        super::particle_logic::register_particle_logic(app);
    }
}
