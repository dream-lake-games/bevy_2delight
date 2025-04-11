mod anim;
mod composition;
mod glue;
mod ldtk;
mod particles;
mod physics;
mod plugin;
mod shader;

#[macro_export]
macro_rules! debug_resource {
    ($app:expr, $resource:ty) => {{
        #[cfg(debug_assertions)]
        {
            $app.add_plugins(
                bevy_inspector_egui::quick::ResourceInspectorPlugin::<$resource>::new().run_if(
                    bevy::input::common_conditions::input_toggle_active(false, KeyCode::Tab),
                ),
            );
        }
    }};
}

pub mod prelude {
    pub use super::anim::prelude::*;
    pub use super::composition::prelude::*;
    pub use super::debug_resource;
    pub use super::glue::prelude::*;
    pub use super::ldtk::prelude::*;
    pub use super::particles::prelude::*;
    pub use super::physics::prelude::*;
    pub use super::plugin::{DelightedSet, TwoDelightPlugin};
    pub use super::shader::prelude::*;
    pub use bevy_2delight_macros::TriggerKind;
}
