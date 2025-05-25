use bevy::{
    asset::RenderAssetUsages,
    ecs::component::HookContext,
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct LightMeshCacheKey {
    /// Changing this should always invalidate. Should be rounded pos of source.
    source_pos: IVec2,
    occluder_hash: i128,
}
impl LightMeshCacheKey {
    fn from_thboxes(source_pos: IVec2, thboxes: &[HBox]) -> Self {
        let hash_thbox = |thbox: &HBox| -> i64 {
            let rounded_pos = thbox.get_offset().round();
            let size = thbox.get_size();
            rounded_pos.x as i64
                + rounded_pos.y as i64
                + size.x as i64
                + size.y as i64
                + rounded_pos.x as i64 * rounded_pos.y as i64
                + size.x as i64 * size.y as i64
        };
        Self {
            source_pos,
            occluder_hash: thboxes.iter().map(|thbox| hash_thbox(thbox) as i128).sum(),
        }
    }
}

#[derive(Clone)]
struct LightMeshCache {
    meshes: Vec<Entity>,
    key: Option<LightMeshCacheKey>,
}
impl Default for LightMeshCache {
    fn default() -> Self {
        Self {
            meshes: vec![],
            key: None,
        }
    }
}
impl LightMeshCache {
    fn clear(&mut self, commands: &mut Commands) {
        for mesh_eid in &self.meshes {
            commands.entity(*mesh_eid).despawn();
        }
        self.meshes = vec![];
    }
}

#[derive(Component)]
#[component(on_remove = on_remove_source)]
pub(super) struct LightSource {
    pub(super) claim: LightClaim,
    pub(super) radius: Option<Fx>,
    mesh_cache: LightMeshCache,
}
impl LightSource {
    pub(super) fn new(claim: LightClaim) -> Self {
        Self {
            claim,
            radius: None,
            mesh_cache: default(),
        }
    }
}
fn on_remove_source(mut world: bevy::ecs::world::DeferredWorld, hook: HookContext) {
    let source = world.get::<LightSource>(hook.entity).unwrap();
    let claim = source.claim.clone();
    let mut mesh_cache = source.mesh_cache.clone();
    claim.free(&mut world);
    mesh_cache.clear(&mut world.commands());
}
pub(super) fn remove_light_source(mut world: bevy::ecs::world::DeferredWorld, hook: HookContext) {
    world
        .commands()
        .get_entity(hook.entity)
        .map(|mut inner| {
            inner.remove::<LightSource>();
        })
        .ok();
}

/// Marks a component as occluding 2d light.
/// The internal spatial hashes this depends on will only recalculate when one
/// of Pos or OccludeLight on the entity has changed.
/// So, if you to change underlying static HBoxes, be warned.
#[derive(Component, Clone)]
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
    fn get_thboxes(&self, stx: Option<&StaticTx>, pos: Pos) -> Vec<HBox> {
        match self {
            OccludeLight::StaticTx => stx
                .expect("OccludeLight::StaticTx needs StaticTx")
                .comps
                .iter()
                .map(|c| c.hbox.translated(pos.as_fvec2()))
                .collect::<Vec<_>>(),
            OccludeLight::Custom { hboxes } => hboxes
                .iter()
                .map(|hbox| hbox.translated(pos.as_fvec2()))
                .collect::<Vec<_>>(),
        }
    }
}
fn on_add_occlude_light(mut world: bevy::ecs::world::DeferredWorld, hook: HookContext) {
    let stx = world.get::<StaticTx>(hook.entity);
    let hboxes = world
        .get::<OccludeLight>(hook.entity)
        .unwrap()
        .get_hboxes(stx);
    let pos = world
        .get::<Pos>(hook.entity)
        .expect("OccludeLight needs Pos")
        .clone();
    let keys = world
        .resource_mut::<SpatHash<SpatHashOccludeLight>>()
        .insert(hook.entity, pos, hboxes);
    world.commands().entity(hook.entity).insert(keys);
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

fn get_blocked_mesh(light_pos: Pos, occlude_thbox: HBox) -> Mesh {
    let get_blocked = |p: Vec2| -> Vec2 { light_pos.as_vec2() + (p - light_pos.as_vec2()) * 500.0 };
    let occlude_lines = [
        (
            occlude_thbox.bottom_left().as_vec2(),
            occlude_thbox.top_left().as_vec2(),
        ),
        (
            occlude_thbox.top_left().as_vec2(),
            occlude_thbox.top_right().as_vec2(),
        ),
        (
            occlude_thbox.top_right().as_vec2(),
            occlude_thbox.bottom_right().as_vec2(),
        ),
        (
            occlude_thbox.bottom_right().as_vec2(),
            occlude_thbox.bottom_left().as_vec2(),
        ),
    ];

    let mut points = Vec::<Vec2>::new();
    let mut tris = Vec::<u32>::new();

    for (a, b) in occlude_lines {
        let first_ix = points.len() as u32;
        tris.extend([first_ix, first_ix + 1, first_ix + 2]);
        tris.extend([first_ix + 2, first_ix + 3, first_ix]);
        points.extend([a, get_blocked(a), get_blocked(b), b]);
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
    mut source_q: Query<(&Pos, &mut LightSource)>,
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

    for (light_pos, mut source) in &mut source_q {
        let Some(radius) = source.radius else {
            continue;
        };
        let source_hbox = HBox::new((radius * 2).round().to_num(), (radius * 2).round().to_num());
        let occluder_keys = spat_hash_occlude_light.get_keys(*light_pos, vec![source_hbox]);
        let occlude_thboxes = spat_hash_occlude_light
            .get_eids(occluder_keys)
            .iter()
            .filter_map(|eid| occluders.get(*eid).ok())
            .map(|(pos, occlude, stx)| occlude.get_thboxes(stx, *pos))
            .flatten()
            .collect::<Vec<_>>();
        let this_frame_cache_key =
            LightMeshCacheKey::from_thboxes(light_pos.as_ivec2(), &occlude_thboxes);
        if Some(&this_frame_cache_key) == source.mesh_cache.key.as_ref() {
            continue;
        }

        source.mesh_cache.clear(&mut commands);
        source.mesh_cache.key = Some(this_frame_cache_key);

        for thbox in occlude_thboxes {
            let mesh = get_blocked_mesh(*light_pos, thbox);
            let new_eid = commands
                .spawn((
                    Name::new("temporary_mesh"),
                    Mesh2d(meshes.add(mesh).into()),
                    MeshMaterial2d(black_mat.clone()),
                    Transform::from_translation(Vec3::Z * 100.0),
                    Visibility::Visible,
                    source.claim.rl.clone(),
                ))
                .insert(ChildOf(light_occlude_root.eid()))
                .id();
            source.mesh_cache.meshes.push(new_eid);
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
