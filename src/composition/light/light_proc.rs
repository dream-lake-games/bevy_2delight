use bevy::{asset::embedded_asset, prelude::*, sprite::Material2dPlugin};
use rand::{thread_rng, Rng};

use crate::{
    composition::{
        layer::LayerSettings,
        mats::{circle_light_mat::CircleLightMat, light_cutout_mat::LightCutoutMat},
        LightingSet,
    },
    glue::{color_as_vec4, Fx},
    prelude::BulletTime,
};

use super::{
    light_alloc::LightClaim,
    light_interaction::{remove_light_source, LightSource},
};

/// Possible strength values: [base_strength - flicker_strength, base_strength + flicker_strength]
/// Every [interval - interval_strength, interval + interval_strength] seconds,
/// take a step of size [-step_strength, step_strength],
/// and set gradient [-gradient_strength, gradient_strength]
#[derive(Component, Reflect)]
pub struct LightFlicker {
    pub base_strength: f32,
    pub flicker_strength: f32,
    pub step_strength: f32,
    pub gradient_strength: f32,
    pub interval: f32,
    pub interval_strength: f32,
    current_strength: f32,
    current_gradient: f32,
    time_till_delta: f32,
}
impl LightFlicker {
    pub fn new(
        base_strength: f32,
        flicker_strength: f32,
        step_strength: f32,
        gradient_strength: f32,
        interval: f32,
        interval_strength: f32,
    ) -> Self {
        Self {
            base_strength,
            flicker_strength,
            step_strength,
            gradient_strength,
            interval,
            interval_strength,
            current_strength: base_strength,
            current_gradient: 0.0,
            time_till_delta: 0.0,
        }
    }
}

#[derive(Component, Reflect)]
#[component(on_add = on_add_circle_light)]
#[component(on_remove = remove_light_source)]
pub struct CircleLight {
    // The color of the light. Should be opaque (i.e. alpha of one)
    pub color: Color,
    pub strength: f32,
    child: Entity,
}
impl CircleLight {
    pub fn strength(strength: f32) -> Self {
        Self {
            color: Color::WHITE,
            strength,
            child: Entity::PLACEHOLDER,
        }
    }
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}
fn on_add_circle_light(
    mut world: bevy::ecs::world::DeferredWorld,
    eid: Entity,
    _: bevy::ecs::component::ComponentId,
) {
    // Get da claim
    let claim = LightClaim::alloc(&mut world);
    let rl = claim.rl.clone();
    world.commands().entity(eid).insert(LightSource::new(claim));
    let myself = world.get::<CircleLight>(eid).unwrap();
    let mat = CircleLightMat::new(myself.color);

    let screen_size = world.resource::<LayerSettings>().screen_size;
    let mesh_hand = world
        .resource_mut::<Assets<Mesh>>()
        .add(Rectangle::new(screen_size.x as f32, screen_size.y as f32));
    let mat_hand = world.resource_mut::<Assets<CircleLightMat>>().add(mat);

    let child_eid = world
        .commands()
        .spawn((
            Name::new("CircleLightChild"),
            Mesh2d(mesh_hand),
            MeshMaterial2d(mat_hand),
            Transform::from_translation(Vec3::Z * thread_rng().gen_range(0.0..1.0)),
            rl,
        ))
        .set_parent(eid)
        .id();
    let mut myself = world.get_mut::<CircleLight>(eid).unwrap();
    myself.child = child_eid;
}

fn drive_circle_lights(
    mut light_q: Query<(&CircleLight, &mut LightSource)>,
    mat_holders: Query<&MeshMaterial2d<CircleLightMat>>,
    mut color_mats: ResMut<Assets<CircleLightMat>>,
    layer_settings: Res<LayerSettings>,
) {
    const PIXELS_PER_RING: f32 = 2.0;
    for (circle_light, mut light_source) in &mut light_q {
        light_source.radius = Some(Fx::from_num(circle_light.strength));
        let mat_holder = mat_holders
            .get(circle_light.child)
            .expect("Circle light invariant");
        let mat = color_mats
            .get_mut(mat_holder.id())
            .expect("Circle light invariant");
        mat.color = color_as_vec4(circle_light.color);
        let screen_size = layer_settings.screen_size.as_vec2();
        mat.sx_sy_rings_unused = Vec4::new(
            circle_light.strength / screen_size.x,
            circle_light.strength / screen_size.y,
            (circle_light.strength / PIXELS_PER_RING).ceil(),
            0.0,
        );
    }
}

fn drive_flicker(
    mut light_q: Query<(&mut CircleLight, &mut LightFlicker)>,
    bullet_time: Res<BulletTime>,
) {
    for (mut circle_light, mut flicker) in &mut light_q {
        flicker.time_till_delta -= bullet_time.delta_secs().to_num::<f32>();

        if flicker.time_till_delta <= 0.0 {
            flicker.time_till_delta = thread_rng().gen_range(
                flicker.interval - flicker.interval_strength
                    ..flicker.interval + flicker.interval_strength,
            );
            flicker.current_gradient =
                thread_rng().gen_range(-flicker.gradient_strength..flicker.gradient_strength);
            flicker.current_strength +=
                thread_rng().gen_range(-flicker.step_strength..flicker.step_strength);
        }

        flicker.current_strength +=
            flicker.current_gradient * bullet_time.delta_secs().to_num::<f32>();
        flicker.current_strength = flicker.current_strength.clamp(
            flicker.base_strength - flicker.flicker_strength,
            flicker.base_strength + flicker.flicker_strength,
        );
        circle_light.strength = flicker.current_strength;
    }
}

pub(crate) fn register_light_proc(app: &mut App) {
    app.register_type::<CircleLight>();
    app.register_type::<LightFlicker>();

    app.add_systems(
        Update,
        (drive_circle_lights, drive_flicker)
            .chain()
            .in_set(LightingSet),
    );
}
