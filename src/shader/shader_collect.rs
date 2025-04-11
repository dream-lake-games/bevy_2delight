use bevy::prelude::*;

#[doc(hidden)]
pub struct _ShaderWizardry {
    pub do_register: fn(app: &mut App) -> (),
}
inventory::collect!(_ShaderWizardry);

/// Still some stutter, but much less
#[macro_export]
macro_rules! defn_shader {
    ($name:ident, $i:item) => {
        paste::paste! {
            #[derive(
                bevy::render::render_resource::AsBindGroup,
                Reflect,
                bevy::render::render_resource::ShaderType,
                Asset,
                Clone,
                Default,
                bevy_2delight_macros::ShaderSpec,
            )]
            $i
            #[doc(hidden)]
            #[expect(nonstandard_style)]
            pub(crate) fn [<_wizardry_for_$name>](app: &mut App) {
                app.add_plugins(ShaderDefnPlugin::<$name>::default());
            }
            inventory::submit! {
                _ShaderWizardry { do_register: [<_wizardry_for_ $name>] }
            }
        }
    };
}

pub(super) fn register_shader_wizardry(app: &mut App) {
    for spell in inventory::iter::<_ShaderWizardry> {
        (spell.do_register)(app);
    }
}
