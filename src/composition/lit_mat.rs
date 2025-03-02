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
    pixels: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    brightness: Handle<Image>,
    #[texture(5)]
    #[sampler(6)]
    reflexivity: Handle<Image>,
    #[texture(7)]
    #[sampler(8)]
    light: Handle<Image>,
    #[uniform(9)]
    pub(crate) base_light: Vec4,
    #[uniform(10)]
    pub(crate) brightness_threshold_unused_unused_unused: Vec4,
}
impl Material2d for LitMat {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_2delight/composition/lit_mat.wgsl".into()
    }
    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}
impl LitMat {
    pub fn new(
        pixels: Handle<Image>,
        brightness: Handle<Image>,
        reflexivity: Handle<Image>,
        light: Handle<Image>,
        base_light: Color,
        brightness_threshold: f32,
    ) -> Self {
        Self {
            pixels,
            brightness,
            reflexivity,
            light,
            base_light: color_as_vec4(base_light),
            brightness_threshold_unused_unused_unused: Vec4::new(
                brightness_threshold,
                0.0,
                0.0,
                0.0,
            ),
        }
    }
}
