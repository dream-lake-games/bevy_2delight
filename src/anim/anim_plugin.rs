use std::marker::PhantomData;

use bevy::prelude::*;

use super::anim_time::{AnimTime, AnimTimeClass, AnimsPaused};
use super::anim_traits::AnimStateMachine;

#[cfg(debug_assertions)]
#[derive(Event)]
pub(super) struct ReloadAnims;
#[cfg(debug_assertions)]
fn watch_for_reload_anims(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Backspace) {
        commands.trigger(ReloadAnims);
    }
}

#[derive(Default)]
pub struct AnimDefnPlugin<StateMachine: AnimStateMachine> {
    _pd: PhantomData<StateMachine>,
}
impl<StateMachine: AnimStateMachine> Plugin for AnimDefnPlugin<StateMachine> {
    fn build(&self, app: &mut App) {
        super::anim_logic::register_anim_logic::<StateMachine>(app);
        super::anim_res::register_anim_res::<StateMachine>(app);
    }
}

#[derive(Clone, Debug, Reflect)]
pub struct AnimSettings {
    pub default_fps: u32,
    pub default_time_class: AnimTimeClass,
}
impl Default for AnimSettings {
    fn default() -> Self {
        Self {
            default_fps: 24,
            default_time_class: default(),
        }
    }
}

#[derive(Clone, Debug, Reflect, Resource)]
pub(crate) struct AnimDefaults {
    pub(crate) settings: AnimSettings,
}

pub(crate) struct AnimPlugin {
    pub(crate) settings: AnimSettings,
}
impl AnimPlugin {
    pub fn new(settings: AnimSettings) -> Self {
        Self { settings }
    }
}
impl Plugin for AnimPlugin {
    fn build(&self, app: &mut App) {
        super::anim_collect::register_anim_wizardry(app);

        app.insert_resource(AnimDefaults {
            settings: self.settings.clone(),
        });
        app.insert_resource(AnimTime::default());
        app.insert_resource(AnimsPaused::default());

        #[cfg(debug_assertions)]
        {
            app.add_systems(Update, watch_for_reload_anims);
        }
    }
}
