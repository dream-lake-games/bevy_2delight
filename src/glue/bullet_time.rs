use std::time::Duration;

use bevy::prelude::*;
use fixed::traits::ToFixed;

use crate::fx;

use super::{Deterministic, Fx};

const FRAMERATE: u32 = 60;

#[derive(Debug)]
struct BulletTimeEffect {
    factor: Fx,
    time_left: Fx,
}

#[derive(Debug)]
struct BulletTimeState {
    base: Fx,
    effects: Vec<BulletTimeEffect>,
}
impl Default for BulletTimeState {
    fn default() -> Self {
        Self {
            base: Fx::ONE,
            effects: vec![],
        }
    }
}

impl BulletTimeState {
    /// Ticks down any active effects
    fn tick(&mut self, amt: Fx) {
        for effect in &mut self.effects {
            effect.time_left -= amt;
        }
        self.effects.retain(|effect| effect.time_left > Fx::ZERO);
    }

    /// Gets the current time factor. This is the slowest active effect, or base if there are no active effects
    fn to_factor(&self) -> Fx {
        self.effects
            .iter()
            .map(|effect| effect.factor)
            .reduce(|a, b| a.min(b))
            .unwrap_or_else(|| self.base)
    }
}

/// How much in-game time has happened. Basically time but accounts for slowdown.
#[derive(Resource, Debug, Default)]
pub struct BulletTime {
    state: BulletTimeState,
    duration: Fx,
    real_duration: Fx,
}
impl BulletTime {
    pub fn delta_secs(&self) -> Fx {
        self.duration
    }
    pub fn real_delta_secs(&self) -> Fx {
        self.real_duration
    }
    pub fn get_base(&self) -> Fx {
        self.state.base
    }
    pub fn set_base<Src: ToFixed>(&mut self, new_base: Src) {
        self.state.base = fx!(new_base);
    }
    pub fn add_effect<F: ToFixed, T: ToFixed>(&mut self, factor: F, time: T) {
        self.state.effects.push(BulletTimeEffect {
            factor: fx!(factor),
            time_left: fx!(time),
        });
    }
    pub fn clear_effects(&mut self) {
        self.state.effects.clear();
    }
}

fn update_bullet_time(
    mut bullet_time: ResMut<BulletTime>,
    time: Res<Time>,
    deterministic: Res<Deterministic>,
) {
    let time_fx = if deterministic.0 {
        fx!(1) / fx!(FRAMERATE)
    } else {
        let min_fps = fx!(1) / fx!(FRAMERATE - 4);
        let max_fps = fx!(1) / fx!(FRAMERATE + 4);
        fx!(time.delta_secs()).max(min_fps).min(max_fps)
    };
    bullet_time.state.tick(time_fx);
    bullet_time.duration = time_fx * bullet_time.state.to_factor();
    bullet_time.real_duration = time_fx;
}

#[derive(Default)]
pub(crate) struct BulletTimePlugin;
impl Plugin for BulletTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BulletTime::default());
        app.add_systems(First, update_bullet_time);
        app.add_plugins(bevy_framepace::FramepacePlugin);
        app.insert_resource(bevy_framepace::FramepaceSettings {
            limiter: bevy_framepace::Limiter::Manual(Duration::from_secs_f64(
                1.0 / FRAMERATE as f64,
            )),
        });
    }
}

pub mod prelude {
    pub use super::BulletTime;
}
