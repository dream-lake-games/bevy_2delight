use bevy::prelude::*;

use crate::{
    glue::Fx,
    prelude::{BulletTime, Dyno, HBox, Pos, StaticRx},
};

use super::{particle_defn::ParticleColorInner, prelude::Particle, ParticleSet};

#[derive(Component)]
struct HasParticleSprite;

#[derive(Component)]
struct ParticleLifespan {
    current: Fx,
    pixel_body: Entity,
    brightness_body: Option<Entity>,
    reflexivity_body: Option<Entity>,
}
impl Default for ParticleLifespan {
    fn default() -> Self {
        Self {
            current: Fx::ZERO,
            pixel_body: Entity::PLACEHOLDER,
            brightness_body: None,
            reflexivity_body: None,
        }
    }
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
    mut needs_lifespan: Query<(Entity, &mut Particle), Without<ParticleLifespan>>,
    root: Res<ParticleRoot>,
    mut commands: Commands,
) {
    for (eid, mut particle) in &mut needs_lifespan {
        let mut lifespan = ParticleLifespan::default();
        particle.resolve_fuzz();

        let size = particle.size.eval(Fx::ZERO).round().to_num();

        lifespan.pixel_body = commands
            .spawn((
                Name::new("ParticlePixels"),
                Sprite {
                    color: particle.color.eval(Fx::ZERO),
                    custom_size: Some(Vec2::new(size as f32, size as f32)),
                    anchor: bevy::sprite::Anchor::BottomLeft,
                    ..default()
                },
                particle.layer.render_layers(),
                HasParticleSprite,
            ))
            .insert(ChildOf(eid))
            .id();
        if let Some(brightness) = particle.brightness.as_ref() {
            lifespan.brightness_body = Some(
                commands
                    .spawn((
                        Name::new("ParticleBrightness"),
                        Sprite {
                            color: brightness.eval(Fx::ZERO),
                            custom_size: Some(Vec2::new(size as f32, size as f32)),
                            anchor: bevy::sprite::Anchor::BottomLeft,
                            ..default()
                        },
                        particle
                            .layer
                            .associated_brightness_layer()
                            .unwrap()
                            .render_layers(),
                        HasParticleSprite,
                    ))
                    .insert(ChildOf(eid))
                    .id(),
            );
        }
        if let Some(reflexivity) = particle.reflexivity.as_ref() {
            lifespan.reflexivity_body = Some(
                commands
                    .spawn((
                        Name::new("ParticleReflexivity"),
                        Sprite {
                            color: reflexivity.eval(Fx::ZERO),
                            custom_size: Some(Vec2::new(size as f32, size as f32)),
                            anchor: bevy::sprite::Anchor::BottomLeft,
                            ..default()
                        },
                        particle
                            .layer
                            .associated_reflexivity_layer()
                            .unwrap()
                            .render_layers(),
                        HasParticleSprite,
                    ))
                    .insert(ChildOf(eid))
                    .id(),
            );
        }

        let mut comms = commands.entity(eid);
        comms.insert(Name::new("Particle"));
        comms.insert(ChildOf(root.eid()));
        comms.insert(particle.initial_pos);
        comms.insert(Dyno::new(
            particle.movement.initial_vel.x,
            particle.movement.initial_vel.y,
        ));
        if let Some(srx_kind) = particle.movement.collision {
            comms.insert(StaticRx::single(srx_kind, HBox::new(size, size)));
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
        &mut Pos,
    )>,
    mut sprite_q: Query<(&mut Sprite, &mut Transform), With<HasParticleSprite>>,
    bullet_time: Res<BulletTime>,
    mut commands: Commands,
) {
    for (eid, mut lifespan, mut dyno, particle, srx, mut pos) in &mut particle_q {
        lifespan.current += bullet_time.delta_secs();
        if lifespan.current > particle.lifetime {
            commands.entity(eid).despawn();
            continue;
        }
        // Appearance
        let current_size = particle
            .size
            .eval(lifespan.current / particle.lifetime)
            .round();
        let offset_pixel_perfect = (current_size.round() / 2).floor().to_num::<f32>();
        let mut handle_sprite = |sprite_eid: Entity, inner: &ParticleColorInner| {
            let Ok((mut sprite, mut tran)) = sprite_q.get_mut(sprite_eid) else {
                return;
            };
            sprite.color = inner.eval(lifespan.current / particle.lifetime);
            sprite.custom_size = Some(Vec2::new(current_size.to_num(), current_size.to_num()));
            tran.translation.x = -offset_pixel_perfect;
            tran.translation.y = -offset_pixel_perfect;
        };
        handle_sprite(lifespan.pixel_body, &particle.color);
        if let Some(bbody) = lifespan.brightness_body {
            handle_sprite(bbody, particle.brightness.as_ref().unwrap());
        }
        if let Some(rbody) = lifespan.reflexivity_body {
            handle_sprite(rbody, particle.reflexivity.as_ref().unwrap());
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
        pos.z -= bullet_time.delta_secs() / 100;
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
