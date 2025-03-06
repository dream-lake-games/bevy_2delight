use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use crate::{
    composition::layer::LightOccludeRoot,
    glue::Fx,
    prelude::{
        on_remove_spat_hash, HBox, LightInteractionSet, Pos, SpatHash, SpatHashOccludeLight,
        SpatKeys, StaticTx,
    },
};

use super::light_alloc::LightClaim;

#[derive(Component)]
#[component(on_remove = on_remove_source)]
pub(super) struct LightSource {
    pub(super) claim: LightClaim,
    pub(super) radius: Option<Fx>,
}
impl LightSource {
    pub(super) fn new(claim: LightClaim) -> Self {
        Self {
            claim,
            radius: None,
        }
    }
}
fn on_remove_source(
    mut world: bevy::ecs::world::DeferredWorld,
    eid: Entity,
    _: bevy::ecs::component::ComponentId,
) {
    let claim = world.get::<LightSource>(eid).unwrap().claim.clone();
    claim.free(&mut world);
}
pub(super) fn remove_light_source(
    mut world: bevy::ecs::world::DeferredWorld,
    eid: Entity,
    _: bevy::ecs::component::ComponentId,
) {
    world.commands().get_entity(eid).map(|mut inner| {
        inner.remove::<LightSource>();
    });
}

/// Marks a component as occluding 2d light.
/// The internal spatial hashes this depends on will only recalculate when one
/// of Pos or OccludeLight on the entity has changed.
/// So, if you to change underlying static HBoxes, be warned.
#[derive(Component)]
#[component(on_add = on_add_occlude_light)]
#[component(on_remove = on_remove_spat_hash::<SpatHashOccludeLight>)]
pub enum OccludeLight {
    /// Occlude light according to the StaticTx hboxes on this entity
    StaticTx,
    /// Provide explicit custom occlusion hboxes
    Custom { hboxes: Vec<HBox> },
}
impl OccludeLight {
    pub fn custom(hboxes: Vec<HBox>) -> Self {
        Self::Custom { hboxes }
    }
    fn get_hboxes(&self, stx: Option<&StaticTx>) -> Vec<HBox> {
        match self {
            OccludeLight::StaticTx => stx
                .expect("OccludeLight::StaticTx needs StaticTx")
                .comps
                .iter()
                .map(|c| c.hbox.clone())
                .collect::<Vec<_>>(),
            OccludeLight::Custom { hboxes } => hboxes.clone(),
        }
    }
}
fn on_add_occlude_light(
    mut world: bevy::ecs::world::DeferredWorld,
    eid: Entity,
    _: bevy::ecs::component::ComponentId,
) {
    let stx = world.get::<StaticTx>(eid);
    let hboxes = world.get::<OccludeLight>(eid).unwrap().get_hboxes(stx);
    let pos = world
        .get::<Pos>(eid)
        .expect("OccludeLight needs Pos")
        .clone();
    let keys = world
        .resource_mut::<SpatHash<SpatHashOccludeLight>>()
        .insert(eid, pos, hboxes);
    world.commands().entity(eid).insert(keys);
}
fn update_occlude_light_spat_hashes(
    mut occlude_light_q: Query<
        (
            Entity,
            &Pos,
            &OccludeLight,
            Option<&StaticTx>,
            &mut SpatKeys<SpatHashOccludeLight>,
        ),
        Or<(Changed<Pos>, Changed<OccludeLight>)>,
    >,
    mut spat_hash_occlude_light: ResMut<SpatHash<SpatHashOccludeLight>>,
) {
    for (eid, pos, occlude_light, stx, mut spat_keys) in &mut occlude_light_q {
        let hboxes = occlude_light.get_hboxes(stx);
        let new_keys = spat_hash_occlude_light.update(eid, &spat_keys, pos.clone(), hboxes);
        *spat_keys = new_keys;
    }
}

fn get_blocked_mesh(light_pos: Pos, occlude_pos: Pos, occlude_hbox: HBox) -> Mesh {
    let get_blocked =
        |p: Pos| -> Vec2 { light_pos.as_vec2() + (p.as_vec2() - light_pos.as_vec2()) * 500.0 };
    let occlude_lines = [
        (
            occlude_pos + occlude_hbox.bottom_left(),
            occlude_pos + occlude_hbox.top_left(),
        ),
        (
            occlude_pos + occlude_hbox.top_left(),
            occlude_pos + occlude_hbox.top_right(),
        ),
        (
            occlude_pos + occlude_hbox.top_right(),
            occlude_pos + occlude_hbox.bottom_right(),
        ),
        (
            occlude_pos + occlude_hbox.bottom_right(),
            occlude_pos + occlude_hbox.bottom_left(),
        ),
    ];

    let mut points = Vec::<Vec2>::new();
    let mut tris = Vec::<u32>::new();

    for (a, b) in occlude_lines {
        let first_ix = points.len() as u32;
        tris.extend([first_ix, first_ix + 1, first_ix + 2]);
        tris.extend([first_ix + 2, first_ix + 3, first_ix]);
        points.extend([a.as_vec2(), get_blocked(a), get_blocked(b), b.as_vec2()]);
    }

    let min_x = points
        .iter()
        .map(|p| p.x)
        .min_by(|a, b| a.total_cmp(b))
        .unwrap();
    let min_y = points
        .iter()
        .map(|p| p.y)
        .min_by(|a, b| a.total_cmp(b))
        .unwrap();
    let max_x = points
        .iter()
        .map(|p| p.x)
        .max_by(|a, b| a.total_cmp(b))
        .unwrap();
    let max_y = points
        .iter()
        .map(|p| p.y)
        .max_by(|a, b| a.total_cmp(b))
        .unwrap();
    let get_frac = |x: f32, min: f32, max: f32| (x - min) / (max - min);

    let mut inserted_positions = vec![];
    let mut inserted_uvs = vec![];
    let mut inserted_normals = vec![];

    for point in points.into_iter() {
        inserted_positions.push([point.x, point.y, 0.0]);
        inserted_uvs.push([
            get_frac(point.x, min_x, max_x),
            get_frac(point.y, min_y, max_y),
        ]);
        inserted_normals.push([0.0, 0.0, 1.0]);
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, inserted_positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, inserted_uvs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, inserted_normals)
    .with_inserted_indices(Indices::U32(tris))
}

fn block_lights(
    source_q: Query<(&Pos, &LightSource)>,
    occluders: Query<(&Pos, &OccludeLight, Option<&StaticTx>)>,
    spat_hash_occlude_light: Res<SpatHash<SpatHashOccludeLight>>,
    light_occlude_root: Res<LightOccludeRoot>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
    mut black_mat: Local<Option<Handle<ColorMaterial>>>,
) {
    if black_mat.is_none() {
        *black_mat = Some(colors.add(ColorMaterial::from(Color::BLACK)));
    }
    let black_mat = black_mat.as_ref().unwrap();

    // One of these days I'll figure out if this is actually slow
    commands
        .entity(light_occlude_root.eid())
        .despawn_descendants();
    for (light_pos, source) in &source_q {
        let Some(radius) = source.radius else {
            continue;
        };
        let source_hbox = HBox::new((radius * 2).round().to_num(), (radius * 2).round().to_num());
        let occluder_keys = spat_hash_occlude_light.get_keys(*light_pos, vec![source_hbox]);
        for (occlude_pos, occlude, stx) in spat_hash_occlude_light
            .get_eids(occluder_keys)
            .iter()
            .filter_map(|eid| occluders.get(*eid).ok())
        {
            let hboxes = occlude.get_hboxes(stx);
            for hbox in hboxes {
                let mesh = get_blocked_mesh(*light_pos, *occlude_pos, hbox);
                commands
                    .spawn((
                        Name::new("temporary_mesh"),
                        Mesh2d(meshes.add(mesh).into()),
                        MeshMaterial2d(black_mat.clone()),
                        Transform::from_translation(Vec3::Z * 100.0),
                        Visibility::Visible,
                        source.claim.rl.clone(),
                    ))
                    .set_parent(light_occlude_root.eid());
            }
        }
    }
}

pub(super) fn register_light_interaction(app: &mut App) {
    app.add_systems(
        Update,
        (update_occlude_light_spat_hashes, block_lights)
            .chain()
            .in_set(LightInteractionSet),
    );
}
