use bevy::prelude::*;

#[doc(hidden)]
pub struct _LightWizardry {
    pub do_register: fn(app: &mut App) -> (),
}
inventory::collect!(_LightWizardry);

/// Still some stutter, but much less
#[macro_export]
macro_rules! defn_light {
    ($name:ident, $i:item) => {
        paste::paste! {
            #[derive(
                Debug,
                Copy,
                Clone,
                Default,
                Reflect,
                PartialEq,
                Eq,
                Hash,
                bevy_2delight_macros::AnimStateMachine,
            )]
            $i
            #[doc(hidden)]
            #[expect(nonstandard_style)]
            pub(crate) fn [<_wizardry_for_$name>](app: &mut App) {
                app.add_plugins(LightDefnPlugin::<$name>::default());
            }
            inventory::submit! {
                _LightWizardry { do_register: [<_wizardry_for_ $name>] }
            }
        }
    };
}

pub(super) fn register_light_wizardry(app: &mut App) {
    for spell in inventory::iter::<_LightWizardry> {
        (spell.do_register)(app);
    }
}
