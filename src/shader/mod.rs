use bevy::{prelude::*, sprite_render::Material2dPlugin};

mod shader_collect;
mod shader_logic;
mod shader_man;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ShaderSet;

#[derive(Default)]
pub struct ShaderDefnPlugin<S: shader_man::ShaderSpec> {
    _pd: std::marker::PhantomData<S>,
}
impl<S: shader_man::ShaderSpec> Plugin for ShaderDefnPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<shader_man::ShaderMat<S>>::default());
        shader_logic::register_shader_logic::<S>(app);
    }
}

pub(crate) struct ShaderPlugin;
impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        shader_collect::register_shader_wizardry(app);
    }
}

pub mod prelude {
    pub use super::shader_collect::_ShaderWizardry;
    pub use super::shader_man::{ShaderMan, ShaderSpec};
    pub use super::ShaderDefnPlugin;
    pub(crate) use super::ShaderPlugin;
    pub use crate::defn_shader;
}
