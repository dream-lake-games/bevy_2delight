use bevy::{
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d},
};

/// The mat that does the multiplying
#[derive(AsBindGroup, Debug, Clone, Asset, Reflect, PartialEq)]
pub(crate) struct BrightnessCullMat {
    #[texture(1)]
    #[sampler(2)]
    brightness: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    reflexivity: Handle<Image>,
    #[texture(5)]
    #[sampler(6)]
    light: Handle<Image>,
    #[texture(7)]
    #[sampler(8)]
    input_pixels: Handle<Image>,
    #[uniform(9)]
    pub(crate) bthreshold_unused_unused_unused: Vec4,
}
impl Material2d for BrightnessCullMat {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_2delight/composition/mats/brightness_cull_mat.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}
impl BrightnessCullMat {
    pub fn new(
        brightness: Handle<Image>,
        reflexivity: Handle<Image>,
        light: Handle<Image>,
        input_pixels: Handle<Image>,
    ) -> Self {
        Self {
            brightness,
            reflexivity,
            light,
            input_pixels,
            bthreshold_unused_unused_unused: Vec4::new(1000.0, 0.0, 0.0, 0.0),
        }
    }
}
