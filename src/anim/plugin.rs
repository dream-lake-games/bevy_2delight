use std::marker::PhantomData;

use bevy::prelude::*;

use super::logic::register_logic;
use super::time::{AnimTime, AnimTimeClass, AnimsPaused};
use super::traits::AnimStateMachine;

#[derive(Default)]
pub struct AnimDefnPlugin<StateMachine: AnimStateMachine> {
    _pd: PhantomData<StateMachine>,
}
impl<StateMachine: AnimStateMachine> Plugin for AnimDefnPlugin<StateMachine> {
    fn build(&self, app: &mut App) {
        register_logic::<StateMachine>(app);
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
    }
}
