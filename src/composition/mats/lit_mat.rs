use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::glue::color_as_vec4;

/// The mat that does the multiplying
#[derive(AsBindGroup, Debug, Clone, Asset, Reflect, PartialEq)]
pub(crate) struct LitMat {
    #[texture(1)]
    #[sampler(2)]
    input: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    light: Handle<Image>,
    #[uniform(5)]
    pub(crate) base_light: Vec4,
}
impl Material2d for LitMat {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_2delight/composition/mats/lit_mat.wgsl".into()
    }
    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}
impl LitMat {
    pub fn new(input: Handle<Image>, light: Handle<Image>, base_light: Color) -> Self {
        Self {
            input,
            light,
            base_light: color_as_vec4(base_light),
        }
    }
}
