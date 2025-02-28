use std::time::Duration;

use bevy::prelude::*;

use crate::glue::frac::Frac;

const FRAMERATE: u32 = 60;

#[derive(Debug)]
struct BulletTimeEffect {
    factor: Frac,
    time_left: Frac,
}

#[derive(Debug)]
struct BulletTimeState {
    base: Frac,
    effects: Vec<BulletTimeEffect>,
}
impl Default for BulletTimeState {
    fn default() -> Self {
        Self {
            base: Frac::ONE,
            effects: vec![],
        }
    }
}

impl BulletTimeState {
    /// Ticks down any active effects
    fn tick(&mut self, amt: Frac) {
        for effect in &mut self.effects {
            effect.time_left -= amt;
        }
        self.effects.retain(|effect| effect.time_left > Frac::ZERO);
    }

    /// Gets the current time factor. This is the slowest active effect, or base if there are no active effects
    fn to_factor(&self) -> Frac {
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
    duration: Frac,
    real_duration: Frac,
}
impl BulletTime {
    pub fn delta_secs(&self) -> Frac {
        self.duration
    }
    pub fn real_delta_secs(&self) -> Frac {
        self.real_duration
    }
    pub fn get_base(&self) -> Frac {
        self.state.base
    }
    pub fn set_base(&mut self, new_base: Frac) {
        self.state.base = new_base;
    }
    pub fn add_effect(&mut self, factor: Frac, time: Frac) {
        self.state.effects.push(BulletTimeEffect {
            factor,
            time_left: time,
        });
    }
    pub fn clear_effects(&mut self) {
        self.state.effects.clear();
    }
}

fn update_bullet_time(mut bullet_time: ResMut<BulletTime>, time: Res<Time>) {
    // If we're at less than 24fps, we'd rather show a slow game than a super jittery one
    // TODO(mork): This should be up to the user to set. Hardcoding for KAMI.
    // TODO(mork): No but really. Adjusting for pyre
    let not_too_fast_time = Frac::whole(1) / Frac::whole(FRAMERATE as i32);
    // let not_too_fast_time = Frac::ZERO.with_micro(time.delta().as_micros() as i32);
    bullet_time.state.tick(not_too_fast_time);
    bullet_time.duration = not_too_fast_time * bullet_time.state.to_factor();
    bullet_time.real_duration = not_too_fast_time;
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
