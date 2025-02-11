use bevy::prelude::*;

use super::man::{AnimMan, AnimNextState, AnimObserveStateChanges};
use super::plugin::AnimDefaults;
use super::traits::{AnimStateMachine, AnimTimeProvider};
use super::AnimSet;

#[derive(Component)]
struct AnimBody;

#[derive(Bundle)]
struct AnimBodyBundle {
    name: Name,
    marker: AnimBody,
    transform: Transform,
    sprite: Sprite,
}
impl AnimBodyBundle {
    pub fn new(
        image: Handle<Image>,
        size: UVec2,
        offset: IVec2,
        flip_x: bool,
        flip_y: bool,
    ) -> Self {
        Self {
            name: Name::new("AnimBody"),
            marker: AnimBody,
            transform: Transform::from_translation(offset.as_vec2().extend(0.0)),
            sprite: Sprite {
                custom_size: Some(size.as_vec2()),
                rect: Some(Rect::from_corners(Vec2::ZERO, size.as_vec2())),
                image,
                flip_x,
                flip_y,
                ..default()
            },
        }
    }
}

/// This system progresses actively running animations. This happens during PreUpdate.
/// It ONLY updates state in AnimMan and DOES NOT update any body sprites.
fn progress_animations<StateMachine: AnimStateMachine, AnimTime: AnimTimeProvider>(
    mut commands: Commands,
    mut anims: Query<(Entity, &mut AnimMan<StateMachine>)>,
    defaults: Res<AnimDefaults>,
    anim_time: Res<AnimTime>,
) {
    let time_class = StateMachine::TIME_CLASS.unwrap_or(defaults.default_time_class);
    let time_delta_us = anim_time.get_delta_us(time_class);

    for (anim_eid, mut anim_man) in &mut anims {
        if anim_man.body == Entity::PLACEHOLDER {
            continue;
        }

        anim_man.last_frame = Some(anim_man.this_frame.clone());

        let get_upf = |current_state: &StateMachine| -> u32 {
            let fps = current_state.get_fps();
            1_000_000 / fps
        };

        // Transition through ixs and states
        anim_man.time_us += time_delta_us;
        while anim_man.time_us > get_upf(&anim_man.this_frame.state) {
            anim_man.this_frame.ix += 1;
            anim_man.time_us -= get_upf(&anim_man.this_frame.state);
            if anim_man.this_frame.ix >= anim_man.this_frame.state.get_length() {
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
                        commands.entity(anim_man.body).despawn_recursive();
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
) {
    for (eid, mut anim_man) in &mut anims {
        anim_man.handle_map = StateMachine::make_handle_map(&ass);

        let body_eid = commands
            .spawn(AnimBodyBundle::new(
                anim_man.handle_map[&anim_man.this_frame.state].clone(),
                StateMachine::SIZE,
                anim_man.get_state().get_offset(),
                anim_man.get_flip_x(),
                anim_man.get_flip_y(),
            ))
            .set_parent(eid)
            .id();

        anim_man.body = body_eid;
    }
}

/// Actually updates the sprites, during PostUpdate
fn drive_animations<StateMachine: AnimStateMachine>(
    mut anims: Query<&AnimMan<StateMachine>>,
    mut bodies: Query<&mut Sprite, With<AnimBody>>,
) {
    for anim_man in &mut anims {
        if anim_man.body == Entity::PLACEHOLDER {
            continue;
        }
        if Some(&anim_man.this_frame) == anim_man.last_frame.as_ref() {
            continue;
        }
        let mut body = bodies
            .get_mut(anim_man.body)
            .expect("Body invariant broken for AnimMan");
        body.image = anim_man.handle_map[&anim_man.get_state()].clone();
        let bottom_left = UVec2::new(anim_man.get_ix() * StateMachine::SIZE.x, 0);
        let top_right = UVec2::new(
            (anim_man.get_ix() + 1) * StateMachine::SIZE.x,
            StateMachine::SIZE.y,
        );
        body.rect = Some(Rect::from_corners(
            bottom_left.as_vec2(),
            top_right.as_vec2(),
        ));
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

pub(crate) fn register_logic<StateMachine: AnimStateMachine, AnimTime: AnimTimeProvider>(
    app: &mut App,
) {
    app.add_systems(
        PreUpdate,
        (
            progress_animations::<StateMachine, AnimTime>,
            bless_animations::<StateMachine>,
        )
            .chain()
            .in_set(AnimSet),
    );
    app.add_systems(
        PostUpdate,
        (
            drive_animations::<StateMachine>,
            trigger_state_changes::<StateMachine>,
        )
            .in_set(AnimSet),
    );
}
