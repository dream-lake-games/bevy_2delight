use bevy::prelude::*;

use crate::{
    glue::Fx,
    prelude::{BulletTime, Dyno, HBox, StaticRx},
};

use super::{particle_defn::ParticleColorInner, prelude::Particle, ParticleSet};

#[derive(Component)]
struct HasParticleSprite;

#[derive(Component, Default)]
struct ParticleLifespan {
    current: Fx,
    brightness_body: Option<Entity>,
    reflexivity_body: Option<Entity>,
}

#[derive(Resource)]
struct ParticleRoot {
    eid: Entity,
}
impl Default for ParticleRoot {
    fn default() -> Self {
        Self {
            eid: Entity::PLACEHOLDER,
        }
    }
}
impl ParticleRoot {
    pub fn eid(&self) -> Entity {
        self.eid
    }
}

fn startup_root(mut commands: Commands, mut particle_root: ResMut<ParticleRoot>) {
    particle_root.eid = commands
        .spawn((
            Name::new("ParticleRoot"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();
}

fn bless_lifespans(
    needs_lifespan: Query<(Entity, &Particle), Without<ParticleLifespan>>,
    root: Res<ParticleRoot>,
    mut commands: Commands,
) {
    for (eid, particle) in &needs_lifespan {
        let mut lifespan = ParticleLifespan::default();

        let mut comms = commands.entity(eid);
        comms.insert(Name::new("Particle"));
        comms.set_parent(root.eid());
        comms.insert(particle.initial_pos);
        comms.insert(Dyno::new(
            particle.movement.initial_vel.x,
            particle.movement.initial_vel.y,
        ));
        let size = particle.size.eval(Fx::ZERO).round().to_num();
        if let Some(srx_kind) = particle.movement.collision {
            comms.insert(StaticRx::single(srx_kind, HBox::new(size, size)));
        }
        comms.insert((
            Sprite {
                color: particle.color.eval(Fx::ZERO),
                custom_size: Some(Vec2::new(size as f32, size as f32)),
                ..default()
            },
            particle.layer.render_layers(),
            HasParticleSprite,
        ));
        if let Some(brightness) = particle.brightness.as_ref() {
            lifespan.brightness_body = Some(
                comms
                    .with_child((
                        Name::new("ParticleBrightness"),
                        Sprite {
                            color: brightness.eval(Fx::ZERO),
                            custom_size: Some(Vec2::new(size as f32, size as f32)),
                            ..default()
                        },
                        particle
                            .layer
                            .associated_brightness_layer()
                            .unwrap()
                            .render_layers(),
                        HasParticleSprite,
                    ))
                    .id(),
            );
        }
        if let Some(reflexivity) = particle.reflexivity.as_ref() {
            lifespan.reflexivity_body = Some(
                comms
                    .with_child((
                        Name::new("ParticleReflexivity"),
                        Sprite {
                            color: reflexivity.eval(Fx::ZERO),
                            custom_size: Some(Vec2::new(size as f32, size as f32)),
                            ..default()
                        },
                        particle
                            .layer
                            .associated_reflexivity_layer()
                            .unwrap()
                            .render_layers(),
                        HasParticleSprite,
                    ))
                    .id(),
            );
        }
        comms.insert(lifespan);
    }
}

fn update_particles(
    mut particle_q: Query<(
        Entity,
        &mut ParticleLifespan,
        &mut Dyno,
        &Particle,
        Option<&StaticRx>,
    )>,
    mut sprite_q: Query<&mut Sprite, With<HasParticleSprite>>,
    bullet_time: Res<BulletTime>,
    mut commands: Commands,
) {
    for (eid, mut lifespan, mut dyno, particle, srx) in &mut particle_q {
        lifespan.current += bullet_time.delta_secs();
        if lifespan.current > particle.lifetime {
            commands.entity(eid).despawn_recursive();
            continue;
        }
        // Appearance
        let current_size = particle.size.eval(lifespan.current).round();
        let handle_sprite = |sprite: &mut Sprite, inner: &ParticleColorInner| {
            sprite.color = inner.eval(lifespan.current);
            sprite.custom_size = Some(Vec2::new(current_size.to_num(), current_size.to_num()));
        };
        let mut color_sprite = sprite_q.get_mut(eid).unwrap();
        handle_sprite(&mut color_sprite, &particle.color);
        if let Some(bbody) = lifespan.brightness_body {
            let mut brightness_sprite = sprite_q.get_mut(bbody).unwrap();
            handle_sprite(
                &mut brightness_sprite,
                particle.brightness.as_ref().unwrap(),
            );
        }
        if let Some(rbody) = lifespan.reflexivity_body {
            let mut reflexivity_sprite = sprite_q.get_mut(rbody).unwrap();
            handle_sprite(
                &mut reflexivity_sprite,
                particle.reflexivity.as_ref().unwrap(),
            );
        }
        // Physics
        if let Some(gravity) = particle.movement.gravity {
            dyno.vel.y -= bullet_time.delta_secs() * gravity;
        }
        if let Some(drag) = particle.movement.drag {
            dyno.vel *= drag;
        }
        if let Some(srx) = srx.as_ref() {
            debug_assert!(srx.comps.len() == 1);
            if srx.comps[0].hbox.get_size().x != current_size.to_num::<u32>() {
                commands.entity(eid).remove::<StaticRx>();
                commands.entity(eid).insert(StaticRx::single(
                    srx.comps[0].kind,
                    HBox::new(current_size.to_num(), current_size.to_num()),
                ));
            }
        }
    }
}

pub(super) fn register_particle_logic(app: &mut App) {
    app.insert_resource(ParticleRoot::default());
    app.add_systems(Startup, startup_root);
    app.add_systems(
        Update,
        (bless_lifespans, update_particles)
            .chain()
            .in_set(ParticleSet),
    );
}
