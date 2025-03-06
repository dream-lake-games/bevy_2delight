use bevy::{
    asset::embedded_asset,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};
use rand::{thread_rng, Rng};

use crate::{
    composition::{layer::LayerSettings, light::light_cutout::LightCutoutMat, LightingSet},
    glue::{color_as_vec4, Fx},
};

use super::{
    light_alloc::LightClaim,
    light_interaction::{remove_light_source, LightSource},
};

#[derive(Resource)]
pub(super) struct ScreenMesh(pub(super) Handle<Mesh>);
fn startup_screen_mesh(
    mut commands: Commands,
    layer_settings: Res<LayerSettings>,
    mut mesh: ResMut<Assets<Mesh>>,
) {
    let hand = mesh.add(Rectangle::new(
        layer_settings.screen_size.x as f32,
        layer_settings.screen_size.y as f32,
    ));
    commands.insert_resource(ScreenMesh(hand));
}

/// The mat that does the multiplying
#[derive(AsBindGroup, Debug, Clone, Asset, Reflect, PartialEq)]
struct CircleLightMat {
    #[uniform(1)]
    pub(crate) color: Vec4,
    #[uniform(2)]
    pub(crate) sx_sy_unused_unused: Vec4,
}
impl Material2d for CircleLightMat {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_2delight/composition/light/circle_light.wgsl".into()
    }
    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}
impl CircleLightMat {
    pub fn new(color: Color) -> Self {
        Self {
            color: color_as_vec4(color),
            sx_sy_unused_unused: Vec4::ZERO,
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
    const PIXELS_PER_RING: f32 = 16.0;
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
        mat.sx_sy_unused_unused = Vec4::new(
            circle_light.strength / screen_size.x,
            circle_light.strength / screen_size.y,
            (circle_light.strength / PIXELS_PER_RING).ceil(),
            0.0,
        );
    }
}

pub(crate) fn register_light_proc(app: &mut App) {
    app.register_type::<CircleLight>();

    app.add_plugins(Material2dPlugin::<CircleLightMat>::default());
    embedded_asset!(app, "circle_light.wgsl");

    app.add_plugins(Material2dPlugin::<LightCutoutMat>::default());
    embedded_asset!(app, "light_cutout.wgsl");

    app.add_systems(Startup, startup_screen_mesh);
    app.add_systems(Update, drive_circle_lights.in_set(LightingSet));
}
