use bevy::{
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};

use crate::layer::plugin::LayerRes;

pub(crate) fn blank_screen_image(res: &LayerRes, is_overlay: bool) -> Image {
    let mult = if is_overlay { res.overlay_growth } else { 1 };
    let target_extent = Extent3d {
        width: res.screen_size.x * mult,
        height: res.screen_size.y * mult,
        ..default()
    };
    // Makes the image
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: target_extent,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    // Fills it with zeros
    image.resize(target_extent);
    image
}

pub fn color_as_vec4(color: Color) -> Vec4 {
    let linear = color.to_linear();
    Vec4::new(linear.red, linear.green, linear.blue, 1.0)
}
