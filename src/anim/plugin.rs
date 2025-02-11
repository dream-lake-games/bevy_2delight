use std::marker::PhantomData;

use bevy::prelude::*;

use super::logic::register_logic;
use super::man::AnimMan;
use super::time::{AnimPlaceholderTime, AnimTimeClass, DEFAULT_TIME_CLASS};
use super::traits::{AnimStateMachine, AnimTimeProvider};

#[derive(Default)]
pub struct AnimDefnPlugin<
    StateMachine: AnimStateMachine,
    AnimTime: AnimTimeProvider = AnimPlaceholderTime,
> {
    _pd: PhantomData<(StateMachine, AnimTime)>,
}
impl<StateMachine: AnimStateMachine, AnimTime: AnimTimeProvider> Plugin
    for AnimDefnPlugin<StateMachine, AnimTime>
{
    fn build(&self, app: &mut App) {
        register_logic::<StateMachine, AnimTime>(app);
        app.register_type::<AnimMan<StateMachine>>();
    }
}

#[derive(Clone, Debug, Reflect, Resource)]
pub(crate) struct AnimDefaults {
    pub default_fps: f32,
    pub default_time_class: i32,
}

pub(crate) fn update_placeholder_time(
    time: Res<Time>,
    mut placeholder_time: ResMut<AnimPlaceholderTime>,
) {
    placeholder_time.real_time_delta_us = time.delta().as_micros() as u32;
}

pub struct AnimPlugin<AnimTime: AnimTimeProvider = AnimPlaceholderTime> {
    default_fps: f32,
    default_time_class: AnimTimeClass,
    _pd: PhantomData<AnimTime>,
}
impl AnimPlugin<AnimPlaceholderTime> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<AnimTime: AnimTimeProvider> AnimPlugin<AnimTime> {
    pub fn with_default_fps(mut self, default_fps: f32) -> Self {
        self.default_fps = default_fps;
        self
    }
    pub fn with_default_time_class(mut self, default_time_class: AnimTimeClass) -> Self {
        self.default_time_class = default_time_class;
        self
    }
}
impl<AnimTime: AnimTimeProvider> Default for AnimPlugin<AnimTime> {
    fn default() -> Self {
        Self {
            default_fps: 24.0,
            default_time_class: DEFAULT_TIME_CLASS,
            _pd: default(),
        }
    }
}
impl<AnimTime: AnimTimeProvider> Plugin for AnimPlugin<AnimTime> {
    fn build(&self, app: &mut App) {
        super::collect::register_anim_wizardry(app);

        app.insert_resource(AnimDefaults {
            default_fps: self.default_fps,
            default_time_class: self.default_time_class,
        });
        app.insert_resource(AnimPlaceholderTime::default());

        app.add_systems(Update, update_placeholder_time.in_set(crate::anim::AnimSet));
    }
}
