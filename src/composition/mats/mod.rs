use bevy::{asset::embedded_asset, prelude::*, sprite::Material2dPlugin};

pub(super) mod brightness_cull_mat;
pub(super) mod circle_light_mat;
pub(super) mod cutout_mat;
pub(super) mod gaussian_blur_mat;
pub(super) mod lit_mat;

pub(super) fn register_mats(app: &mut App) {
    embedded_asset!(app, "brightness_cull_mat.wgsl");
    app.add_plugins(Material2dPlugin::<brightness_cull_mat::BrightnessCullMat>::default());

    embedded_asset!(app, "circle_light_mat.wgsl");
    app.add_plugins(Material2dPlugin::<circle_light_mat::CircleLightMat>::default());

    embedded_asset!(app, "cutout_mat.wgsl");
    app.add_plugins(Material2dPlugin::<cutout_mat::CutoutMat>::default());

    embedded_asset!(app, "gaussian_blur_mat.wgsl");
    app.add_plugins(Material2dPlugin::<gaussian_blur_mat::GaussianBlurMat>::default());

    embedded_asset!(app, "lit_mat.wgsl");
    app.add_plugins(Material2dPlugin::<lit_mat::LitMat>::default());
}
