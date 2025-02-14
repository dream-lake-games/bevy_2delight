use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        primitives::Aabb,
        view::RenderLayers,
    },
    sprite::ColorMaterial,
};

use crate::{
    layer::plugin::LayerRes,
    physics::PhysicsSet,
    prelude::{AnimMan, FVec2, Frac, HBox, Pos, StaticTx},
};

use super::light_man::{LightMan, LightStateMachine};

pub(super) const BLACK_MAT_HAND: Handle<ColorMaterial> = Handle::weak_from_u128(12398129387129837);
const MAX_LIGHT_EXTENT: Frac = Frac::const_whole(500);

#[derive(Component)]
struct TemporaryLightMesh;

struct SolidLine {
    a: FVec2,
    b: FVec2,
}

fn hbox_to_solid_lines(hbox: &HBox) -> [SolidLine; 4] {
    let offset = hbox.get_offset();
    let size = FVec2::new(
        Frac::whole(hbox.get_size().x as i32),
        Frac::whole(hbox.get_size().y as i32),
    );
    let left = FVec2::X * size.x / Frac::TWO;
    let up = FVec2::Y * size.y / Frac::TWO;
    [
        SolidLine {
            a: offset - left - up,
            b: offset - left + up,
        },
        SolidLine {
            a: offset - left + up,
            b: offset + left + up,
        },
        SolidLine {
            a: offset + left + up,
            b: offset + left - up,
        },
        SolidLine {
            a: offset + left - up,
            b: offset - left - up,
        },
    ]
}

fn hbox_to_blocked_mesh(source: FVec2, hbox: &HBox) -> (Mesh, Aabb) {
    let get_blocked = |p: FVec2| -> FVec2 { p + (p - source) * MAX_LIGHT_EXTENT };

    let mut points = Vec::<FVec2>::new();
    let mut tris = Vec::<u32>::new();

    for line in hbox_to_solid_lines(hbox) {
        let first_ix = points.len() as u32;
        tris.extend([first_ix, first_ix + 1, first_ix + 2]);
        tris.extend([first_ix + 2, first_ix + 3, first_ix]);
        points.extend([line.a, get_blocked(line.a), get_blocked(line.b), line.b]);
    }

    let min_x = points.iter().map(|p| p.x).min_by(|a, b| a.cmp(b)).unwrap();
    let min_y = points.iter().map(|p| p.y).min_by(|a, b| a.cmp(b)).unwrap();
    let max_x = points.iter().map(|p| p.x).max_by(|a, b| a.cmp(b)).unwrap();
    let max_y = points.iter().map(|p| p.y).max_by(|a, b| a.cmp(b)).unwrap();
    let get_frac = |x: f32, min: f32, max: f32| (x - min) / (max - min);

    let mut inserted_positions = vec![];
    let mut inserted_uvs = vec![];
    let mut inserted_normals = vec![];

    for point in points.into_iter() {
        inserted_positions.push([point.x.as_f32(), point.y.as_f32(), 0.0]);
        inserted_uvs.push([
            get_frac(point.x.as_f32(), min_x.as_f32(), max_x.as_f32()),
            get_frac(point.y.as_f32(), min_y.as_f32(), max_y.as_f32()),
        ]);
        inserted_normals.push([0.0, 0.0, 1.0]);
    }

    (
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, inserted_positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, inserted_uvs)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, inserted_normals)
        .with_inserted_indices(Indices::U32(tris)),
        Aabb::enclosing([
            Vec3::new(min_x.as_f32(), min_y.as_f32(), 0.0),
            Vec3::new(max_x.as_f32(), max_y.as_f32(), 0.0),
        ])
        .unwrap(),
    )
}

fn block_lights<Anim: LightStateMachine>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    layers_res: Res<LayerRes>,
    // NOTE: Because we reuse meshes (which might be stupid) we have to take in a dummy exclusive
    //       ref to the `TemporaryLightMesh` so different variants of this system never run in parallel
    old_q: Query<(Entity, &Mesh2d, &mut TemporaryLightMesh)>,
    source_q: Query<(&Pos, &LightMan<Anim>, &AnimMan<Anim>)>,
    blocker_q: Query<(&Pos, &StaticTx)>,
) {
    let mut old_iter = old_q.iter();
    let mut make_or_reuse_mesh = |rl: RenderLayers, mesh: Mesh, aabb: Aabb| {
        if let Some((eid, emesh, _)) = old_iter.next() {
            let Some(mr) = meshes.get_mut(emesh.id()) else {
                return;
            };
            *mr = mesh;
            commands.entity(eid).insert((aabb, rl));
        } else {
            commands
                .spawn((
                    Name::new("temporary_mesh"),
                    Mesh2d(meshes.add(mesh).into()),
                    MeshMaterial2d(BLACK_MAT_HAND.clone()),
                    Transform::from_translation(Vec3::Z * 100.0),
                    Visibility::Inherited,
                    rl,
                    TemporaryLightMesh,
                ))
                .set_parent(layers_res.root_eid());
        }
    };

    for (source_pos, light, anim) in &source_q {
        let Some(light_radius) = anim.get_state().light_radius() else {
            // Returning none here means the light is intentionally off/not active for this variant
            continue;
        };
        let source_v2 = source_pos.as_fvec2();
        for (blocker_pos, stx) in &blocker_q {
            for blocker_hbox in stx.get_thboxes(*blocker_pos) {
                if blocker_hbox.manhattan_distance_to_point(source_v2) > light_radius {
                    continue;
                }
                if blocker_hbox.manhattan_distance_to_point(source_v2) <= Frac::cent(10) {
                    // If a source is inside a box we want to ignore that box, useful for passup
                    continue;
                }
                let (mesh, aabb) = hbox_to_blocked_mesh(source_v2, &blocker_hbox);
                make_or_reuse_mesh(
                    RenderLayers::from_layers(&[light.claim.rl_usize]),
                    mesh,
                    aabb,
                );
            }
        }
    }

    drop(make_or_reuse_mesh);
    while let Some((eid, _, _)) = old_iter.next() {
        commands.entity(eid).despawn_recursive();
    }
}

pub(super) fn register_light_interaction<Anim: LightStateMachine>(app: &mut App) {
    app.add_systems(Update, block_lights::<Anim>.after(PhysicsSet));
}
