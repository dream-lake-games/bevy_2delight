use bevy::{asset::embedded_asset, prelude::*, sprite::Material2dPlugin};
use light_alloc::LightAllocer;
use light_interaction::BLACK_MAT_HAND;
use light_mat::{LightApplyMat, LightCutoutMat};

mod light_alloc;
mod light_collect;
mod light_interaction;
pub(crate) mod light_man;
pub(crate) mod light_mat;

fn setup_black_mat(mut mats: ResMut<Assets<ColorMaterial>>) {
    mats.insert(BLACK_MAT_HAND.id(), Color::BLACK.into());
}

pub(crate) struct LayersLightPlugin;
impl Plugin for LayersLightPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "light_apply_mat.wgsl");
        embedded_asset!(app, "light_cutout_mat.wgsl");

        light_collect::register_light_wizardry(app);

        app.add_plugins(Material2dPlugin::<LightApplyMat>::default());
        app.add_plugins(Material2dPlugin::<LightCutoutMat>::default());

        app.insert_resource(LightAllocer::default());

        app.add_systems(Startup, setup_black_mat);
    }
}
