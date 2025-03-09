use bevy::{asset::embedded_asset, prelude::*, sprite::Material2dPlugin};

pub(super) mod circle_light_mat;
pub(super) mod light_cutout_mat;
pub(super) mod lit_mat;

pub(super) fn register_mats(app: &mut App) {
    embedded_asset!(app, "circle_light_mat.wgsl");
    app.add_plugins(Material2dPlugin::<circle_light_mat::CircleLightMat>::default());

    embedded_asset!(app, "light_cutout_mat.wgsl");
    app.add_plugins(Material2dPlugin::<light_cutout_mat::LightCutoutMat>::default());

    embedded_asset!(app, "lit_mat.wgsl");
    app.add_plugins(Material2dPlugin::<lit_mat::LitMat>::default());
}
