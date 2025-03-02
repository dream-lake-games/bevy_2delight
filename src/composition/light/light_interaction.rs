use bevy::prelude::*;

use super::light_man::LightAnim;

pub(super) fn register_light_interaction<Anim: LightAnim>(_app: &mut App) {}
