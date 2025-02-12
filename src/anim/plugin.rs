use std::marker::PhantomData;

use bevy::prelude::*;

use super::logic::register_logic;
use super::man::AnimMan;
use super::time::{AnimTime, AnimTimeClass, AnimTimeSet, DEFAULT_TIME_CLASS};
use super::traits::AnimStateMachine;
use super::AnimSet;

#[derive(Default)]
pub struct AnimDefnPlugin<StateMachine: AnimStateMachine> {
    _pd: PhantomData<StateMachine>,
}
impl<StateMachine: AnimStateMachine> Plugin for AnimDefnPlugin<StateMachine> {
    fn build(&self, app: &mut App) {
        register_logic::<StateMachine>(app);
        app.register_type::<AnimMan<StateMachine>>();
    }
}

#[derive(Clone, Debug, Reflect, Resource)]
pub(crate) struct AnimDefaults {
    pub default_fps: f32,
    pub default_time_class: i32,
}

pub(crate) fn update_default_time(time: Res<Time>, mut anim_time: ResMut<AnimTime>) {
    anim_time.set(DEFAULT_TIME_CLASS, time.delta().as_micros() as u32);
}

pub struct AnimPlugin {
    default_fps: f32,
    default_time_class: AnimTimeClass,
}
impl AnimPlugin {
    pub fn new() -> Self {
        Self::default()
    }
}
impl AnimPlugin {
    pub fn with_default_fps(mut self, default_fps: f32) -> Self {
        self.default_fps = default_fps;
        self
    }
    pub fn with_default_time_class(mut self, default_time_class: AnimTimeClass) -> Self {
        self.default_time_class = default_time_class;
        self
    }
}
impl Default for AnimPlugin {
    fn default() -> Self {
        Self {
            default_fps: 24.0,
            default_time_class: DEFAULT_TIME_CLASS,
        }
    }
}
impl Plugin for AnimPlugin {
    fn build(&self, app: &mut App) {
        super::collect::register_anim_wizardry(app);

        app.insert_resource(AnimDefaults {
            default_fps: self.default_fps,
            default_time_class: self.default_time_class,
        });
        app.insert_resource(AnimTime::default());

        app.add_systems(
            PreUpdate,
            update_default_time.in_set(AnimTimeSet).in_set(AnimSet),
        );
    }
}
