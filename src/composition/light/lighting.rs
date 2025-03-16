use bevy::{prelude::*, utils::HashMap};

use crate::{
    composition::{
        mats::{brightness_cull_mat::BrightnessCullMat, lit_mat::LitMat},
        LightingSet,
    },
    glue::color_as_vec4,
    prelude::Layer,
};

#[derive(Resource, Reflect)]
pub struct Lighting {
    pub base_ambient: Color,
    pub base_detail: Color,
    pub brightness_threshold: f32,
    // TODO: It was a little supid to insert the entity instead of just the asset id here.
    //       Oh well. Fix it later.
    pub(crate) lit_asset_map: HashMap<Layer, AssetId<LitMat>>,
    pub(crate) bcull_asset: AssetId<BrightnessCullMat>,
}
impl Default for Lighting {
    fn default() -> Self {
        Self {
            base_ambient: Color::linear_rgb(0.6, 0.6, 0.6),
            brightness_threshold: 1.0,
            base_detail: Color::linear_rgb(0.3, 0.3, 0.3),
            lit_asset_map: default(),
            bcull_asset: default(),
        }
    }
}

fn update_lit_mats(lighting: Res<Lighting>, mut mats: ResMut<Assets<LitMat>>) {
    let ambient = mats
        .get_mut(lighting.lit_asset_map[&Layer::AmbientPixels])
        .expect("Ambient should always exist");
    ambient.base_light = color_as_vec4(lighting.base_ambient);

    let back_detail = mats
        .get_mut(lighting.lit_asset_map[&Layer::BackDetailPixels])
        .expect("Back detail should always exist");
    back_detail.base_light = color_as_vec4(lighting.base_detail);
    let front_detail = mats
        .get_mut(lighting.lit_asset_map[&Layer::FrontDetailPixels])
        .expect("Front detail should always exist");
    front_detail.base_light = color_as_vec4(lighting.base_detail);
}

fn update_brightness_cull_mats(
    lighting: Res<Lighting>,
    mut mats: ResMut<Assets<BrightnessCullMat>>,
) {
    let bcull_mat = mats
        .get_mut(lighting.bcull_asset)
        .expect("Bcull mat should always exist");
    bcull_mat.bthreshold_unused_unused_unused[0] = lighting.brightness_threshold;
}

pub(crate) fn register_lighting(app: &mut App) {
    super::light_collect::register_light_wizardry(app);
    super::light_interaction::register_light_interaction(app);

    app.insert_resource(Lighting::default());
    app.insert_resource(super::light_alloc::LightAllocer::default());

    app.add_systems(
        Update,
        (update_lit_mats, update_brightness_cull_mats).in_set(LightingSet),
    );
}
