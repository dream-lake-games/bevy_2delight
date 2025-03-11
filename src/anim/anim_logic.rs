use bevy::prelude::*;
use bevy::render::view::RenderLayers;

use crate::prelude::{BulletTime, Fx, Layer};

use super::anim_man::{AnimMan, AnimNextState, AnimObserveStateChanges};
use super::anim_plugin::AnimDefaults;
use super::anim_res::AnimRes;
use super::anim_time::{AnimTime, AnimTimeClass, AnimsPaused};
use super::anim_traits::AnimStateMachine;
use super::{AnimPostSet, AnimPreSet};

fn update_anim_time(
    anims_paused: Res<AnimsPaused>,
    bullet_time: Res<BulletTime>,
    mut anim_time: ResMut<AnimTime>,
) {
    if anims_paused.0 {
        anim_time.set(AnimTimeClass::BulletUnpaused, Fx::ZERO);
        anim_time.set(AnimTimeClass::RealUnpaused, Fx::ZERO);
    } else {
        anim_time.set(AnimTimeClass::BulletUnpaused, bullet_time.delta_secs());
        anim_time.set(AnimTimeClass::RealUnpaused, bullet_time.real_delta_secs());
    }
    anim_time.set(AnimTimeClass::BulletAlways, bullet_time.delta_secs());
    anim_time.set(AnimTimeClass::RealAlways, bullet_time.real_delta_secs());
}

#[derive(Component)]
struct AnimBody;

#[derive(Bundle)]
struct AnimBodyBundle {
    name: Name,
    marker: AnimBody,
    transform: Transform,
    sprite: Sprite,
    render_layers: RenderLayers,
}
impl AnimBodyBundle {
    pub fn new(
        name: &str,
        image: Handle<Image>,
        size: UVec2,
        offset: IVec2,
        flip_x: bool,
        flip_y: bool,
        render_layers: RenderLayers,
    ) -> Self {
        // TODO: If we ever make offset editable at runtime we'll have to tweak this I think
        let mut corrected_offset = offset.as_vec2();
        if size.x % 2 == 1 {
            corrected_offset.x += 0.5;
        }
        if size.y % 2 == 1 {
            corrected_offset.y += 0.5;
        }

        Self {
            name: Name::new(format!("AnimBody_{name}")),
            marker: AnimBody,
            transform: Transform::from_translation(corrected_offset.extend(0.0)),
            sprite: Sprite {
                custom_size: Some(size.as_vec2()),
                rect: Some(Rect::from_corners(Vec2::ZERO, size.as_vec2())),
                image,
                flip_x,
                flip_y,
                image_mode: SpriteImageMode::Tiled {
                    tile_x: true,
                    tile_y: true,
                    stretch_value: 1.0,
                },
                ..default()
            },
            render_layers,
        }
    }
}

/// This system progresses actively running animations. This happens during PreUpdate.
/// It ONLY updates state in AnimMan and DOES NOT update any body sprites.
fn progress_animations<StateMachine: AnimStateMachine>(
    mut commands: Commands,
    mut anims: Query<(Entity, &mut AnimMan<StateMachine>)>,
    defaults: Res<AnimDefaults>,
    anim_time: Res<AnimTime>,
    anim_res: Res<AnimRes<StateMachine>>,
) {
    let time_class = StateMachine::TIME_CLASS.unwrap_or(defaults.settings.default_time_class);
    let time_delta_us = anim_time.get(time_class);

    for (anim_eid, mut anim_man) in &mut anims {
        if anim_man.pixel_body == Entity::PLACEHOLDER {
            continue;
        }

        anim_man.last_frame = Some(anim_man.this_frame.clone());

        let get_spf = |current_state: &StateMachine| -> Fx {
            let fps = current_state.get_fps();
            Fx::from_num(1) / Fx::from_num(fps as i32)
        };

        // Transition through ixs and states
        anim_man.time += time_delta_us;
        while anim_man.time > get_spf(&anim_man.this_frame.state) {
            anim_man.this_frame.ix += 1;
            let dec = get_spf(&anim_man.this_frame.state);
            let length = anim_res.get_length(anim_man.this_frame.state);
            anim_man.time -= dec;
            if anim_man.this_frame.ix >= length {
                match anim_man.this_frame.state.get_next() {
                    AnimNextState::Stay => {
                        anim_man.this_frame.ix = 0;
                    }
                    AnimNextState::Some(next_state) => {
                        anim_man.this_frame.state = next_state;
                        anim_man.this_frame.ix = 0;
                    }
                    AnimNextState::Despawn => {
                        if let Some(comms) = commands.get_entity(anim_eid) {
                            comms.despawn_recursive();
                        }
                        break;
                    }
                    AnimNextState::Remove => {
                        commands.entity(anim_man.pixel_body).despawn_recursive();
                        commands.entity(anim_eid).remove::<AnimMan<StateMachine>>();
                        commands
                            .entity(anim_eid)
                            .remove::<AnimObserveStateChanges>();
                        break;
                    }
                }
            }
        }
    }
}

/// Constructs the handle map, and spawns bodies on entities with newly added AnimMans
/// Also happens in PreUpdate, but _after_ progress animations so that the first frame
/// an AnimMan exists it's last state is seen as None
fn bless_animations<StateMachine: AnimStateMachine>(
    mut commands: Commands,
    mut anims: Query<(Entity, &mut AnimMan<StateMachine>), Added<AnimMan<StateMachine>>>,
    ass: Res<AssetServer>,
    anim_res: Res<AnimRes<StateMachine>>,
) {
    for (eid, mut anim_man) in &mut anims {
        anim_man.pixel_handle_map = StateMachine::make_pixel_handle_map(&ass);
        anim_man.pixel_body = commands
            .spawn(AnimBodyBundle::new(
                "pixels",
                anim_man.pixel_handle_map[&anim_man.this_frame.state].clone(),
                anim_res.get_size() * StateMachine::REP,
                anim_man.get_state().get_offset(),
                anim_man.get_flip_x(),
                anim_man.get_flip_y(),
                anim_man.render_layers.clone(),
            ))
            .set_parent(eid)
            .id();

        if anim_res.has_brightness() {
            anim_man.brightness_handle_map = StateMachine::make_brightness_handle_map(&ass);
            anim_man.brightness_body = commands
                .spawn(AnimBodyBundle::new(
                    "brightness",
                    anim_man.brightness_handle_map[&anim_man.this_frame.state].clone(),
                    anim_res.get_size() * StateMachine::REP,
                    anim_man.get_state().get_offset(),
                    anim_man.get_flip_x(),
                    anim_man.get_flip_y(),
                    if anim_man.render_layers == Layer::AmbientPixels.render_layers() {
                        Layer::AmbientBrightness.render_layers()
                    } else if anim_man.render_layers == Layer::DetailPixels.render_layers() {
                        Layer::DetailBrightness.render_layers()
                    } else if anim_man.render_layers == Layer::StaticPixels.render_layers() {
                        Layer::StaticBrightness.render_layers()
                    } else {
                        panic!(
                            "Trying to apply brightness in render_layers: {:?}",
                            anim_man.render_layers
                        );
                    },
                ))
                .set_parent(eid)
                .id();
        }
    }
}

/// Actually updates the sprites, during PostUpdate
fn drive_animations<StateMachine: AnimStateMachine>(
    mut anims: Query<&AnimMan<StateMachine>>,
    mut bodies: Query<&mut Sprite, With<AnimBody>>,
    anim_res: Res<AnimRes<StateMachine>>,
) {
    let size = anim_res.get_size();
    for anim_man in &mut anims {
        let flip_change = anim_man.delta_flip_x().is_some() || anim_man.delta_flip_y().is_some();
        let state_change = Some(&anim_man.this_frame) == anim_man.last_frame.as_ref();
        if !flip_change && !state_change {
            continue;
        }
        for (body_eid, handle_map) in [
            (anim_man.pixel_body, &anim_man.pixel_handle_map),
            (anim_man.brightness_body, &anim_man.brightness_handle_map),
        ] {
            if body_eid == Entity::PLACEHOLDER {
                continue;
            }
            let mut body = bodies
                .get_mut(body_eid)
                .expect("Body invariant broken for AnimMan");
            if flip_change {
                body.flip_x = anim_man.get_flip_x();
                body.flip_y = anim_man.get_flip_y();
            }
            if state_change {
                body.image = handle_map[&anim_man.get_state()].clone();
                let bottom_left = UVec2::new(anim_man.get_ix() * size.x, 0);
                let top_right = UVec2::new((anim_man.get_ix() + 1) * size.x - 1, size.y);
                body.rect = Some(Rect::from_corners(
                    bottom_left.as_vec2(),
                    top_right.as_vec2(),
                ));
                #[cfg(debug_assertions)]
                {
                    // When debugging we may change the size or length of an anim mid run
                    body.custom_size = Some((anim_res.get_size() * StateMachine::REP).as_vec2());
                }
            }
        }
    }
}

fn trigger_state_changes<StateMachine: AnimStateMachine>(
    mut commands: Commands,
    anims: Query<&AnimMan<StateMachine>, With<AnimObserveStateChanges>>,
) {
    for anim_man in &anims {
        if let Some(delta) = anim_man.delta_state() {
            commands.trigger(delta);
        }
    }
}

pub(crate) fn register_anim_logic<StateMachine: AnimStateMachine>(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_anim_time,
            progress_animations::<StateMachine>,
            bless_animations::<StateMachine>,
        )
            .chain()
            .in_set(AnimPreSet),
    );
    app.add_systems(
        Update,
        (
            drive_animations::<StateMachine>,
            trigger_state_changes::<StateMachine>,
        )
            .chain()
            .in_set(AnimPostSet),
    );
}
