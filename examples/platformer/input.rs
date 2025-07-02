use bevy::prelude::*;
use bevy_2delight::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[repr(u32)]
#[derive(EnumIter)]
pub enum Combos {
    Jump,
}

fn jump_combo(frames: &[InputFrame; INPUT_HISTORY_LENGTH]) -> bool {
    let head = &frames[0..6];
    head.iter()
        .any(|frame| frame.butts.just_pressed(Butt::West))
}

fn add_combos(mut input: ResMut<Input>) {
    for combo in Combos::iter() {
        let func = match combo {
            Combos::Jump => jump_combo,
        };
        input.add_combo(combo as ComboKey, func);
    }
}

pub(super) fn register_input(app: &mut App) {
    app.add_systems(Startup, add_combos);
}
