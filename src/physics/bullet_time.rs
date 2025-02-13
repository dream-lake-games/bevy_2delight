use std::time::Duration;

use bevy::prelude::*;

use crate::glue::frac::Frac;

pub trait BulletTimeClass:
    std::fmt::Debug + Default + std::marker::Send + std::marker::Sync + Clone + 'static
{
    fn to_factor(&self) -> f32;
}

#[derive(Debug, Default, Clone)]
pub enum BulletTimeClassDefault {
    #[default]
    Normal,
}
impl BulletTimeClass for BulletTimeClassDefault {
    fn to_factor(&self) -> f32 {
        match self {
            Self::Normal => 1.0,
        }
    }
}

#[derive(Debug)]
struct BulletTimeEffect<TimeClass: BulletTimeClass> {
    class: TimeClass,
    time_left: f32,
}

#[derive(Debug, Default)]
struct BulletTimeState<TimeClass: BulletTimeClass> {
    base: TimeClass,
    effects: Vec<BulletTimeEffect<TimeClass>>,
}

impl<TimeClass: BulletTimeClass> BulletTimeState<TimeClass> {
    /// Ticks down any active effects
    fn tick(&mut self, real_time: f32) {
        for effect in &mut self.effects {
            effect.time_left -= real_time;
        }
        self.effects.retain(|effect| effect.time_left > 0.0);
    }

    /// Gets the current time factor. This is the slowest active effect, or base if there are no active effects
    fn to_factor(&self) -> f32 {
        self.effects
            .iter()
            .map(|effect| effect.class.to_factor())
            .reduce(|a, b| a.min(b))
            .unwrap_or_else(|| self.base.to_factor())
    }
}

/// How much in-game time has happened. Basically time but accounts for slowdown.
#[derive(Resource, Debug, Default)]
pub struct BulletTimeGeneric<TimeClass: BulletTimeClass> {
    state: BulletTimeState<TimeClass>,
    duration: std::time::Duration,
}
impl<TimeClass: BulletTimeClass> BulletTimeGeneric<TimeClass> {
    pub fn delta(&self) -> std::time::Duration {
        self.duration
    }
    pub fn delta_secs(&self) -> Frac {
        Frac::ZERO.with_micro(self.duration.as_micros() as i32)
    }
    pub fn get_base(&self) -> TimeClass {
        self.state.base.clone()
    }
    pub fn set_base(&mut self, new_base: TimeClass) {
        self.state.base = new_base;
    }
    pub fn add_effect(&mut self, class: TimeClass, time: f32) {
        self.state.effects.push(BulletTimeEffect {
            class,
            time_left: time,
        });
    }
    pub fn clear_effects(&mut self) {
        self.state.effects.clear();
    }
}

fn update_bullet_time<TimeClass: BulletTimeClass>(
    mut bullet_time: ResMut<BulletTimeGeneric<TimeClass>>,
    time: Res<Time>,
) {
    // If we're at less than 24fps, we'd rather show a slow game than a super jittery one
    // TODO(mork): This should be up to the user to set. Hardcoding for KAMI.
    // TODO(mork): No but really. Adjusting for pyre
    let not_too_fast_time = time.delta().min(Duration::from_secs_f32(1.0 / 55.0));
    bullet_time.state.tick(not_too_fast_time.as_secs_f32());
    bullet_time.duration = not_too_fast_time.mul_f32(bullet_time.state.to_factor());
}

#[derive(Default)]
pub(crate) struct BulletTimePlugin<TimeClass: BulletTimeClass> {
    _pd: std::marker::PhantomData<TimeClass>,
}
impl<TimeClass: BulletTimeClass> Plugin for BulletTimePlugin<TimeClass> {
    fn build(&self, app: &mut App) {
        app.insert_resource(BulletTimeGeneric::<TimeClass>::default());
        app.add_systems(First, update_bullet_time::<TimeClass>);
    }
}
