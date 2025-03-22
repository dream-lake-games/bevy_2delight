use bevy::prelude::*;
use fixed::traits::ToFixed;
use rand::{thread_rng, Rng};

use crate::{
    fx,
    glue::Fx,
    prelude::{FVec2, Layer, Pos, StaticRxKind, Terp, TerpMode},
};

#[derive(Clone)]
pub(super) struct ParticleMovement {
    pub(super) initial_vel: FVec2,
    pub(super) gravity: Option<Fx>,
    pub(super) drag: Option<Fx>,
    pub(super) collision: Option<StaticRxKind>,
}
impl Default for ParticleMovement {
    fn default() -> Self {
        Self {
            initial_vel: FVec2::default(),
            gravity: None,
            drag: None,
            collision: None,
        }
    }
}

#[derive(Clone)]
pub(super) enum ParticleFxInner {
    Constant(Fx),
    Terp(Terp<Fx>),
}
impl ParticleFxInner {
    pub(super) fn eval(&self, frac: Fx) -> Fx {
        match self {
            Self::Constant(num) => *num,
            Self::Terp(terp) => terp.eval(frac),
        }
    }
}

#[derive(Clone)]
pub(super) enum ParticleColorInner {
    Constant(Color),
    Terp(Terp<Color>),
}
impl ParticleColorInner {
    pub(super) fn eval(&self, frac: Fx) -> Color {
        match self {
            Self::Constant(color) => *color,
            Self::Terp(terp) => terp.eval(frac),
        }
    }
}

#[derive(Default, Clone)]
pub struct ParticleFuzz {
    pos: Option<Pos>,
    lifetime: Option<Fx>,
    vel: Option<FVec2>,
    gravity: Option<Fx>,
    drag: Option<Fx>,
    size: Option<Fx>,
    color: Option<Color>,
    brightness: Option<Color>,
    reflexivity: Option<Color>,
}
fn fuzz_rand_fx(fuzz: Fx) -> Fx {
    if fuzz > 0.00001 {
        fx!(thread_rng().gen_range(-fuzz.to_num::<f32>()..fuzz.to_num()))
    } else {
        Fx::ZERO
    }
}
fn fuzz_rand_color(existing: Color, fuzz: Color) -> Color {
    Color::srgb(
        existing.to_srgba().red + thread_rng().gen_range(-fuzz.to_srgba().red..fuzz.to_srgba().red),
        existing.to_srgba().green
            + thread_rng().gen_range(-fuzz.to_srgba().green..fuzz.to_srgba().green),
        existing.to_srgba().blue
            + thread_rng().gen_range(-fuzz.to_srgba().blue..fuzz.to_srgba().blue),
    )
}

#[derive(Component, Clone)]
pub struct Particle {
    pub(super) initial_pos: Pos,
    pub(super) lifetime: Fx,
    pub(super) movement: ParticleMovement,
    pub(super) size: ParticleFxInner,
    pub(super) color: ParticleColorInner,
    pub(super) brightness: Option<ParticleColorInner>,
    pub(super) reflexivity: Option<ParticleColorInner>,
    pub(super) layer: Layer,
    pub(super) fuzz: Option<ParticleFuzz>,
}
impl Default for Particle {
    fn default() -> Self {
        Self {
            initial_pos: Pos::default(),
            lifetime: fx!(1),
            movement: ParticleMovement::default(),
            size: ParticleFxInner::Constant(Fx::ONE),
            color: ParticleColorInner::Constant(Color::WHITE),
            brightness: None,
            reflexivity: None,
            layer: Layer::StaticPixels,
            fuzz: None,
        }
    }
}
impl Particle {
    pub fn new<L: ToFixed>(pos: Pos, lifetime: L) -> Self {
        Self {
            initial_pos: pos.with_z(pos.z - fx!(0.0001)),
            lifetime: fx!(lifetime),
            ..default()
        }
    }
    pub fn with_pos_fuzz<X: ToFixed, Y: ToFixed>(mut self, x: X, y: Y) -> Self {
        if self.fuzz.is_none() {
            self.fuzz = Some(ParticleFuzz::default());
        }
        self.fuzz.as_mut().unwrap().pos = Some(Pos::new(fx!(x), fx!(y)));
        self
    }
    pub fn with_lifetime_fuzz<F: ToFixed>(mut self, fuzz: F) -> Self {
        if self.fuzz.is_none() {
            self.fuzz = Some(ParticleFuzz::default());
        }
        self.fuzz.as_mut().unwrap().lifetime = Some(fx!(fuzz));
        self
    }
    pub fn with_vel(mut self, vel: FVec2) -> Self {
        self.movement.initial_vel = vel;
        self
    }
    pub fn with_vel_fuzz<X: ToFixed, Y: ToFixed>(mut self, x: X, y: Y) -> Self {
        if self.fuzz.is_none() {
            self.fuzz = Some(ParticleFuzz::default());
        }
        self.fuzz.as_mut().unwrap().vel = Some(FVec2::new(fx!(x), fx!(y)));
        self
    }
    pub fn with_gravity<G: ToFixed>(mut self, gravity: G) -> Self {
        self.movement.gravity = Some(fx!(gravity));
        self
    }
    pub fn with_gravity_fuzz<F: ToFixed>(mut self, fuzz: F) -> Self {
        if self.fuzz.is_none() {
            self.fuzz = Some(ParticleFuzz::default());
        }
        self.fuzz.as_mut().unwrap().gravity = Some(fx!(fuzz));
        self
    }
    pub fn with_drag<D: ToFixed>(mut self, drag: D) -> Self {
        self.movement.drag = Some(fx!(drag));
        self
    }
    pub fn with_drag_fuzz<F: ToFixed>(mut self, fuzz: F) -> Self {
        if self.fuzz.is_none() {
            self.fuzz = Some(ParticleFuzz::default());
        }
        self.fuzz.as_mut().unwrap().drag = Some(fx!(fuzz));
        self
    }
    pub fn with_collision(mut self, collision: StaticRxKind) -> Self {
        self.movement.collision = Some(collision);
        self
    }
    pub fn with_size_constant<S: ToFixed>(mut self, size: S) -> Self {
        self.size = ParticleFxInner::Constant(fx!(size));
        self
    }
    pub fn with_size_terp<S1: ToFixed, S2: ToFixed>(
        mut self,
        start: S1,
        stop: S2,
        mode: TerpMode,
    ) -> Self {
        self.size = ParticleFxInner::Terp(Terp::new(fx!(start), fx!(stop), mode));
        self
    }
    pub fn with_size_fuzz<F: ToFixed>(mut self, fuzz: F) -> Self {
        if self.fuzz.is_none() {
            self.fuzz = Some(ParticleFuzz::default());
        }
        self.fuzz.as_mut().unwrap().size = Some(fx!(fuzz));
        self
    }
    pub fn with_color_constant(mut self, color: Color) -> Self {
        self.color = ParticleColorInner::Constant(color);
        self
    }
    pub fn with_color_terp(mut self, start: Color, stop: Color, mode: TerpMode) -> Self {
        self.color = ParticleColorInner::Terp(Terp::new(start, stop, mode));
        self
    }
    pub fn with_color_fuzz(mut self, fuzz: Color) -> Self {
        if self.fuzz.is_none() {
            self.fuzz = Some(ParticleFuzz::default());
        }
        self.fuzz.as_mut().unwrap().color = Some(fuzz);
        self
    }
    pub fn with_brightness_constant(mut self, brightness: Color) -> Self {
        self.brightness = Some(ParticleColorInner::Constant(brightness));
        self
    }
    pub fn with_brightness_terp(mut self, start: Color, stop: Color, mode: TerpMode) -> Self {
        self.brightness = Some(ParticleColorInner::Terp(Terp::new(start, stop, mode)));
        self
    }
    pub fn with_brightness_fuzz(mut self, fuzz: Color) -> Self {
        if self.fuzz.is_none() {
            self.fuzz = Some(ParticleFuzz::default());
        }
        self.fuzz.as_mut().unwrap().brightness = Some(fuzz);
        self
    }
    pub fn with_reflexivity_constant(mut self, reflexivity: Color) -> Self {
        self.reflexivity = Some(ParticleColorInner::Constant(reflexivity));
        self
    }
    pub fn with_reflexivity_terp(mut self, start: Color, stop: Color, mode: TerpMode) -> Self {
        self.reflexivity = Some(ParticleColorInner::Terp(Terp::new(start, stop, mode)));
        self
    }
    pub fn with_reflexivity_fuzz(mut self, fuzz: Color) -> Self {
        if self.fuzz.is_none() {
            self.fuzz = Some(ParticleFuzz::default());
        }
        self.fuzz.as_mut().unwrap().reflexivity = Some(fuzz);
        self
    }
    pub fn with_layer(mut self, layer: Layer) -> Self {
        self.layer = layer;
        self
    }

    pub(super) fn resolve_fuzz(&mut self) {
        if let Some(fuzz) = self.fuzz.take() {
            if let Some(fuzz_pos) = fuzz.pos {
                self.initial_pos += FVec2::new(fuzz_rand_fx(fuzz_pos.x), fuzz_rand_fx(fuzz_pos.y));
            }
            if let Some(lifetime) = fuzz.lifetime {
                self.lifetime += fuzz_rand_fx(lifetime);
            }
            if let Some(vel) = fuzz.vel {
                self.movement.initial_vel += FVec2::new(fuzz_rand_fx(vel.x), fuzz_rand_fx(vel.y));
            }
            if let Some(gravity) = fuzz.gravity {
                self.movement.gravity =
                    Some(self.movement.gravity.unwrap() + fuzz_rand_fx(gravity));
            }
            if let Some(drag) = fuzz.drag {
                self.movement.drag = Some(self.movement.drag.unwrap() + fuzz_rand_fx(drag));
            }
            if let Some(size) = fuzz.size {
                self.size = ParticleFxInner::Terp(Terp::new(
                    self.size.eval(Fx::ZERO) + fuzz_rand_fx(size),
                    self.size.eval(Fx::ONE) + fuzz_rand_fx(size),
                    TerpMode::Linear,
                ));
            }
            if let Some(color) = fuzz.color {
                self.color = ParticleColorInner::Terp(Terp::new(
                    fuzz_rand_color(self.color.eval(Fx::ZERO), color),
                    fuzz_rand_color(self.color.eval(Fx::ONE), color),
                    TerpMode::Linear,
                ));
            }
            if let Some(brightness) = fuzz.brightness {
                self.brightness = Some(ParticleColorInner::Terp(Terp::new(
                    fuzz_rand_color(self.brightness.as_ref().unwrap().eval(Fx::ZERO), brightness),
                    fuzz_rand_color(self.brightness.as_ref().unwrap().eval(Fx::ONE), brightness),
                    TerpMode::Linear,
                )));
            }
            if let Some(reflexivity) = fuzz.reflexivity {
                self.reflexivity = Some(ParticleColorInner::Terp(Terp::new(
                    fuzz_rand_color(
                        self.reflexivity.as_ref().unwrap().eval(Fx::ZERO),
                        reflexivity,
                    ),
                    fuzz_rand_color(
                        self.reflexivity.as_ref().unwrap().eval(Fx::ONE),
                        reflexivity,
                    ),
                    TerpMode::Linear,
                )));
            }
        }
    }
}
