use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    glue::Fx,
    prelude::{FVec2, Layer, Pos, StaticRxKind, Terp, TerpMode},
};

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

#[derive(Component)]
pub struct Particle {
    pub(super) initial_pos: Pos,
    pub(super) lifetime: Fx,
    pub(super) movement: ParticleMovement,
    pub(super) size: ParticleFxInner,
    pub(super) color: ParticleColorInner,
    pub(super) brightness: Option<ParticleColorInner>,
    pub(super) reflexivity: Option<ParticleColorInner>,
    pub(super) layer: Layer,
}
impl Default for Particle {
    fn default() -> Self {
        Self {
            initial_pos: Pos::default(),
            lifetime: Fx::from_num(1),
            movement: ParticleMovement::default(),
            size: ParticleFxInner::Constant(Fx::ONE),
            color: ParticleColorInner::Constant(Color::WHITE),
            brightness: None,
            reflexivity: None,
            layer: Layer::StaticPixels,
        }
    }
}
impl Particle {
    pub fn new(pos: Pos, lifetime: Fx) -> Self {
        Self {
            initial_pos: pos.with_z(pos.z - Fx::from_num(0.0001)),
            lifetime,
            ..default()
        }
    }
    pub fn with_pos_fuzz(mut self, x: f32, y: f32) -> Self {
        self.initial_pos.x += Fx::from_num(thread_rng().gen_range(-x..x));
        self.initial_pos.y += Fx::from_num(thread_rng().gen_range(-y..y));
        self
    }
    pub fn with_vel(mut self, vel: FVec2) -> Self {
        self.movement.initial_vel = vel;
        self
    }
    pub fn with_vel_fuzz(mut self, x: f32, y: f32) -> Self {
        self.movement.initial_vel.x += Fx::from_num(thread_rng().gen_range(-x..x));
        self.movement.initial_vel.y += Fx::from_num(thread_rng().gen_range(-y..y));
        self
    }
    pub fn with_gravity(mut self, gravity: Fx) -> Self {
        self.movement.gravity = Some(gravity);
        self
    }
    pub fn with_drag(mut self, drag: Fx) -> Self {
        self.movement.drag = Some(drag);
        self
    }
    pub fn with_collision(mut self, collision: StaticRxKind) -> Self {
        self.movement.collision = Some(collision);
        self
    }
    pub fn with_size_constant(mut self, size: Fx) -> Self {
        self.size = ParticleFxInner::Constant(size);
        self
    }
    pub fn with_size_terp(mut self, start: Fx, stop: Fx, mode: TerpMode) -> Self {
        self.size = ParticleFxInner::Terp(Terp::new(start, stop, mode));
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
    pub fn with_brightness_constant(mut self, brightness: Color) -> Self {
        self.brightness = Some(ParticleColorInner::Constant(brightness));
        self
    }
    pub fn with_brightness_terp(mut self, start: Color, stop: Color, mode: TerpMode) -> Self {
        self.brightness = Some(ParticleColorInner::Terp(Terp::new(start, stop, mode)));
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
    pub fn with_layer(mut self, layer: Layer) -> Self {
        self.layer = layer;
        self
    }
}
