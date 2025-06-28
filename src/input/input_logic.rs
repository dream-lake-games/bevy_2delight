use super::input_data::*;
use bevy::prelude::*;

pub(super) fn register_input_logic(app: &mut App) {
    app.insert_resource(StickInput::default());
    app.insert_resource(ButtInput::default());
    app.insert_resource(InputHistory::default());
    app.insert_resource(InputCombos::default());
}
