use bevy::prelude::*;

use crate::input::input_logic::register_input_logic;

pub(crate) struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        register_input_logic(app);
    }
}
