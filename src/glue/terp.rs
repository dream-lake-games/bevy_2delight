use bevy::prelude::*;

use super::Fx;

pub enum TerpMode {
    Linear,
    EaseInQuadratic,
    EaseOutQuadratic,
}
impl TerpMode {
    fn to_mul(&self, frac: Fx) -> Fx {
        match self {
            Self::Linear => frac,
            Self::EaseInQuadratic => frac * frac,
            Self::EaseOutQuadratic => frac.sqrt(),
        }
    }
}

pub trait Terpable: Sized {
    fn terp(start: &Self, stop: &Self, mode: &TerpMode, frac: Fx) -> Self;
}

pub struct Terp<T: Terpable> {
    start: T,
    stop: T,
    mode: TerpMode,
}
impl<T: Terpable> Terp<T> {
    pub fn new(start: T, stop: T, mode: TerpMode) -> Self {
        Self { start, stop, mode }
    }
    pub fn eval(&self, frac: Fx) -> T {
        T::terp(&self.start, &self.stop, &self.mode, frac)
    }
}

impl Terpable for Fx {
    fn terp(start: &Self, stop: &Self, mode: &TerpMode, frac: Fx) -> Self {
        *start + (*stop - *start) * mode.to_mul(frac)
    }
}
impl Terpable for Color {
    fn terp(start: &Self, stop: &Self, mode: &TerpMode, frac: Fx) -> Self {
        let frac = mode.to_mul(frac);
        let start = start.to_linear();
        let stop = stop.to_linear();
        Color::linear_rgb(
            start.red + (stop.red - start.red) * frac.to_num::<f32>(),
            start.green + (stop.green - start.green) * frac.to_num::<f32>(),
            start.blue + (stop.blue - start.blue) * frac.to_num::<f32>(),
        )
    }
}
