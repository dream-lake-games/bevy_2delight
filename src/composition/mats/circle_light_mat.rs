use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::glue::color_as_vec4;

/// The mat that does the multiplying
#[derive(AsBindGroup, Debug, Clone, Asset, Reflect, PartialEq)]
pub(crate) struct CircleLightMat {
    #[uniform(1)]
    pub(crate) color: Vec4,
    #[uniform(2)]
    pub(crate) sx_sy_rings_unused: Vec4,
}
impl Material2d for CircleLightMat {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_2delight/composition/mats/circle_light_mat.wgsl".into()
    }
    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}
impl CircleLightMat {
    pub fn new(color: Color) -> Self {
        Self {
            color: color_as_vec4(color),
            sx_sy_rings_unused: Vec4::ZERO,
        }
    }
}
