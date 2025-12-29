use bevy::{
    mesh::MeshVertexBufferLayoutRef,
    prelude::*,
    render::render_resource::{
        AsBindGroup, BlendComponent, BlendFactor, BlendOperation, BlendState,
        RenderPipelineDescriptor, SpecializedMeshPipelineError,
    },
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dKey},
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

/// When multiple lights are on screen, we don't want the covering quads of one to hide another
/// This basically turns black into clear to get around that.
#[derive(AsBindGroup, Debug, Clone, Asset, Reflect, PartialEq)]
pub(crate) struct CutoutMat {
    #[texture(1)]
    #[sampler(2)]
    input: Handle<Image>,
}
impl Material2d for CutoutMat {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_2delight/composition/mats/cutout_mat.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
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
impl CutoutMat {
    pub fn new(input: Handle<Image>) -> Self {
        Self { input }
    }
}
