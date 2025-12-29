use bevy::{
    prelude::*,
    render::render_resource::{encase::private::WriteInto, AsBindGroup, ShaderSize, ShaderType},
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d},
};

use crate::prelude::Layer;

pub trait ShaderSpec:
    AsBindGroup + Reflect + ShaderType + ShaderSize + WriteInto + Clone + Asset
{
    const SHADER_PATH: &str;
    const DEFAULT_SIZE: UVec2;
    const DEFAULT_LAYER: Layer = Layer::StaticPixels;
    const DEFAULT_LOOP_TIME: f32 = 60.0;
    const DEFAULT_REPS: UVec2 = UVec2::ONE;
}

#[derive(AsBindGroup, Reflect, ShaderType, Asset, Clone, Default)]
pub(super) struct ShaderInput {
    pub(super) bullet_time: f32,
    pub(super) real_time: f32,
    pub(super) loop_time: f32,
    pub(super) rep_x: f32,
    pub(super) rep_y: f32,
}

#[derive(AsBindGroup, Reflect, ShaderType, Clone, Asset)]
pub(super) struct ShaderMat<S: ShaderSpec> {
    #[uniform(0)]
    pub(super) input: ShaderInput,
    #[uniform(1)]
    pub(super) data: S,
}
impl<S: ShaderSpec> ShaderMat<S> {
    pub(super) fn new(data: S) -> Self {
        Self {
            input: default(),
            data,
        }
    }
}
impl<S: ShaderSpec> Material2d for ShaderMat<S> {
    fn fragment_shader() -> ShaderRef {
        S::SHADER_PATH.into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

#[derive(Component, Debug)]
#[require(Transform, Visibility)]
pub struct ShaderMan<S: ShaderSpec> {
    pub data: S,
    pub(super) body_eid: Entity,
    pub(super) size: UVec2,
    pub(super) last_size: Option<UVec2>,
    pub(super) layer: Layer,
    pub(super) last_layer: Option<Layer>,
    pub(super) loop_time: f32,
    pub(super) reps: UVec2,
    pub(super) last_reps: Option<UVec2>,
}
impl<S: ShaderSpec> ShaderMan<S> {
    pub fn new(data: S) -> Self {
        Self {
            data,
            body_eid: Entity::PLACEHOLDER,
            size: S::DEFAULT_SIZE,
            last_size: None,
            layer: S::DEFAULT_LAYER,
            last_layer: None,
            loop_time: S::DEFAULT_LOOP_TIME,
            reps: S::DEFAULT_REPS,
            last_reps: None,
        }
    }

    pub fn set_size(&mut self, size: UVec2) {
        self.size = size;
    }
    pub fn set_layer(&mut self, layer: Layer) {
        self.layer = layer;
    }
    pub fn set_loop_time(&mut self, loop_time: f32) {
        self.loop_time = loop_time;
    }
    pub fn set_reps(&mut self, rep: UVec2) {
        self.reps = rep;
    }
    pub fn with_size(mut self, size: UVec2) -> Self {
        self.set_size(size);
        self
    }
    pub fn with_layer(mut self, layer: Layer) -> Self {
        self.set_layer(layer);
        self
    }
    pub fn with_loop_time(mut self, loop_time: f32) -> Self {
        self.set_loop_time(loop_time);
        self
    }
    pub fn with_reps(mut self, rep: UVec2) -> Self {
        self.set_reps(rep);
        self
    }

    pub(super) fn needs_size_update(&mut self) -> bool {
        if Some(self.size) != self.last_size {
            self.last_size = Some(self.size);
            return true;
        }
        false
    }
    pub(super) fn needs_layer_update(&mut self) -> bool {
        if Some(self.layer) != self.last_layer {
            self.last_layer = Some(self.layer);
            return true;
        }
        false
    }
    pub(super) fn needs_reps_update(&mut self) -> bool {
        if Some(self.reps) != self.last_reps {
            self.last_reps = Some(self.reps);
            return true;
        }
        false
    }
}
