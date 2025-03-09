use bevy::{prelude::*, utils::HashMap};

use crate::{
    composition::{mats::lit_mat::LitMat, LightingSet},
    glue::color_as_vec4,
    prelude::Layer,
};

#[derive(Resource, Reflect)]
pub struct Lighting {
    pub base_ambient: Color,
    pub brightness_threshold_ambient: f32,
    pub base_detail: Color,
    pub brightness_threshold_detail: f32,
    pub(crate) layer_eid_map: HashMap<Layer, Entity>,
}
impl Default for Lighting {
    fn default() -> Self {
        Self {
            base_ambient: Color::linear_rgb(0.6, 0.6, 0.6),
            brightness_threshold_ambient: 1.0,
            base_detail: Color::linear_rgb(0.3, 0.3, 0.3),
            brightness_threshold_detail: 1.0,
            layer_eid_map: default(),
        }
    }
}

fn update_lit_mats(
    mat_q: Query<&MeshMaterial2d<LitMat>>,
    lighting: Res<Lighting>,
    mut mats: ResMut<Assets<LitMat>>,
) {
    let ambient_hand = mat_q
        .get(lighting.layer_eid_map[&Layer::AmbientPixels])
        .expect("Ambient hand should always exist");
    let ambient = mats
        .get_mut(ambient_hand)
        .expect("Ambient should always exist");
    ambient.base_light = color_as_vec4(lighting.base_ambient);

    let detail_hand = mat_q
        .get(lighting.layer_eid_map[&Layer::DetailPixels])
        .expect("Detail hand should always exist");
    let detail = mats
        .get_mut(detail_hand)
        .expect("Detail should always exist");
    detail.base_light = color_as_vec4(lighting.base_detail);
}

pub(crate) fn register_lighting(app: &mut App) {
    super::light_collect::register_light_wizardry(app);
    super::light_interaction::register_light_interaction(app);

    app.insert_resource(Lighting::default());
    app.insert_resource(super::light_alloc::LightAllocer::default());

    app.add_systems(Update, update_lit_mats.in_set(LightingSet));
}
