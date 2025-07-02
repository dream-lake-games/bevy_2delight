use super::input_data::*;
use bevy::{input::InputSystem, platform::collections::HashMap, prelude::*};
use strum::IntoEnumIterator;

fn update_input(keyboard: Res<ButtonInput<KeyCode>>, mut input: ResMut<Input>) {
    let last_frame = input.frame_history[0].clone();

    let mut left_stick_dir = Vec2::ZERO;
    let key_dirs = [
        (KeyCode::KeyW, Vec2::Y),
        (KeyCode::KeyS, -Vec2::Y),
        (KeyCode::KeyA, -Vec2::X),
        (KeyCode::KeyD, Vec2::X),
    ];
    for (key, dir) in key_dirs.iter() {
        if keyboard.pressed(*key) {
            left_stick_dir += *dir;
        }
    }
    left_stick_dir = left_stick_dir.normalize_or_zero();
    let sticks = StickFrame(HashMap::from_iter([
        (Stick::Left, left_stick_dir),
        (Stick::Right, default()),
    ]));

    let mut butts = PressFrame::<Butt>::default();
    let keymap = HashMap::<Butt, KeyCode>::from_iter([
        (Butt::North, KeyCode::KeyI),
        (Butt::East, KeyCode::KeyL),
        (Butt::South, KeyCode::KeyK),
        (Butt::West, KeyCode::KeyJ),
    ]);
    for butt in Butt::iter() {
        let pressed_this_frame = keyboard.pressed(keymap[&butt]);
        butts.0.insert(
            butt,
            PressData {
                pressed: pressed_this_frame,
                just_pressed: pressed_this_frame && !last_frame.butts.pressed(butt),
                just_released: !pressed_this_frame && last_frame.butts.pressed(butt),
            },
        );
    }

    let new_frame = InputFrame {
        sticks,
        butts,
        combos: default(),
    };
    input.add_frame(new_frame);

    let mut correct_combos = PressFrame::<ComboKey>::default();
    for (combo, trigger) in input.combo_map.iter() {
        let pressed_this_frame = trigger(&input.frame_history);
        correct_combos.0.insert(
            *combo,
            PressData {
                pressed: pressed_this_frame,
                just_pressed: pressed_this_frame && !last_frame.combos.pressed(*combo),
                just_released: !pressed_this_frame && last_frame.combos.pressed(*combo),
            },
        );
    }
    input.correct_current_frame_combos(correct_combos);

    let new_frame = input.frame_history[0].clone();
    input.sticks = new_frame.sticks;
    input.butts = new_frame.butts;
    input.combos = new_frame.combos;
}

pub(super) fn register_input_logic(app: &mut App) {
    app.insert_resource(Input::default());

    app.add_systems(Update, update_input.in_set(InputSystem));
}
