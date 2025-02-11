use bevy::prelude::*;

#[doc(hidden)]
pub struct _AnimWizardry {
    pub do_register: fn(app: &mut App) -> (),
}
inventory::collect!(_AnimWizardry);

/// Still some stutter, but much less
#[macro_export]
macro_rules! defn_anim {
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
                app.add_plugins(AnimDefnPlugin::<$name>::default());
            }
            inventory::submit! {
                _AnimWizardry { do_register: [<_wizardry_for_ $name>] }
            }
        }
    };
}

pub(super) fn register_anim_wizardry(app: &mut App) {
    for spell in inventory::iter::<_AnimWizardry> {
        (spell.do_register)(app);
    }
}
