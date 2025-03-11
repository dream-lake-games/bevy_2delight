use bevy::{
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            AsBindGroup, BlendComponent, BlendFactor, BlendOperation, BlendState,
            RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dKey},
};

const BLEND_ADD: BlendState = BlendState {
    color: BlendComponent {
        src_factor: BlendFactor::SrcAlpha,
        dst_factor: BlendFactor::One,
        operation: BlendOperation::Add,
    },

    alpha: BlendComponent {
        src_factor: BlendFactor::SrcAlpha,
        dst_factor: BlendFactor::One,
        operation: BlendOperation::Add,
    },
};

/// NOTE: Despite its name, this material actually only does ONE PASS of gaussian blurring.
///       SOoooo you probably want two of 'em in your pipeline.
#[derive(AsBindGroup, Debug, Clone, Asset, Reflect, PartialEq)]
pub(crate) struct GaussianBlurMat {
    #[texture(1)]
    #[sampler(2)]
    input: Handle<Image>,
    #[uniform(3)]
    pub(crate) horizontal_ksize_unused_unused: IVec4,
}
impl Material2d for GaussianBlurMat {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_2delight/composition/mats/gaussian_blur_mat.wgsl".into()
    }
    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(fragment) = &mut descriptor.fragment {
            if let Some(target_state) = &mut fragment.targets[0] {
                target_state.blend = Some(BLEND_ADD);
            }
        }
        Ok(())
    }
}
impl GaussianBlurMat {
    pub fn new(input: Handle<Image>, horizontal: bool, ksize: i32) -> Self {
        Self {
            input,
            horizontal_ksize_unused_unused: IVec4::new(if horizontal { 1 } else { 0 }, ksize, 0, 0),
        }
    }
}
