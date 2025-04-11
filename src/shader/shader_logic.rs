use bevy::prelude::*;

use crate::prelude::BulletTime;

use super::{
    prelude::{ShaderMan, ShaderSpec},
    shader_man::ShaderMat,
    ShaderSet,
};

#[derive(Component)]
struct ShaderBody;

fn create_bodies<S: ShaderSpec>(
    mut shader_men: Query<(Entity, &mut ShaderMan<S>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<ShaderMat<S>>>,
) {
    for (eid, mut sman) in &mut shader_men {
        if sman.body_eid != Entity::PLACEHOLDER {
            continue;
        }
        let mesh = meshes.add(Rectangle::new(sman.size.x as f32, sman.size.y as f32));
        let mat = mats.add(ShaderMat::new(sman.data.clone()));
        sman.body_eid = commands
            .spawn((
                Name::new("ShaderBody"),
                ShaderBody,
                Transform::default(),
                Visibility::default(),
                Mesh2d(mesh),
                MeshMaterial2d(mat),
                sman.layer.render_layers(),
            ))
            .set_parent(eid)
            .id();
    }
}

fn update_bodies<S: ShaderSpec>(
    mut shader_men: Query<&mut ShaderMan<S>>,
    shader_bodies: Query<(&Mesh2d, &MeshMaterial2d<ShaderMat<S>>), With<ShaderBody>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<ShaderMat<S>>>,
    bullet_time: Res<BulletTime>,
    mut commands: Commands,
) {
    for mut man in &mut shader_men {
        let (mesh_hand, mat_hand) = shader_bodies
            .get(man.body_eid)
            .expect("shader body should exist");
        if man.needs_size_update() || man.needs_reps_update() {
            let mesh = meshes
                .get_mut(mesh_hand.id())
                .expect("shader body should have mesh");
            *mesh = Rectangle::new(
                man.size.x as f32 * man.reps.x as f32,
                man.size.y as f32 * man.reps.y as f32,
            )
            .into();
        }
        if man.needs_layer_update() {
            commands
                .entity(man.body_eid)
                .insert(man.layer.render_layers());
        }

        let mat = mats
            .get_mut(mat_hand.id())
            .expect("shader mat should exist");
        mat.input.loop_time = man.loop_time;
        mat.input.bullet_time += bullet_time.delta_secs().to_num::<f32>();
        mat.input.bullet_time = mat.input.bullet_time.rem_euclid(man.loop_time);
        mat.input.real_time += bullet_time.real_delta_secs().to_num::<f32>();
        mat.input.real_time = mat.input.real_time.rem_euclid(man.loop_time);
        mat.input.rep_x = man.reps.x as f32;
        mat.input.rep_y = man.reps.y as f32;
    }
}

pub(super) fn register_shader_logic<S: ShaderSpec>(app: &mut App) {
    app.add_systems(
        Update,
        (create_bodies::<S>, update_bodies::<S>)
            .chain()
            .in_set(ShaderSet),
    );
}
