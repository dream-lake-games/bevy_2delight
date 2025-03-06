use bevy::prelude::*;

mod camera;
mod layer;
mod light;
mod lit_mat;
mod parallax;
mod plugin;

/// The set that handles driving underlying light animations.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct LightAnimSet;

/// The set that handles light interaction.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct LightInteractionSet;

/// The set that handles translating the user defined base lights + brightness thresholds to the mats.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct LightingSet;

/// The set that internally handles updating layer cameras. This happens in `Update`.
/// NOTE: This is the system that places all the cameras. You must make sure the pos is correct before this system.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LayersCameraSet;

pub mod prelude {
    pub use super::camera::{CameraShake, DynamicCamera};
    pub use super::layer::Layer;
    pub use super::light::light_interaction::OccludeLight;
    pub use super::light::light_man::{LightAnim, LightDefnPlugin, LightMan};
    pub use super::light::light_proc::CircleLight;
    pub use super::light::lighting::Lighting;
    pub use super::parallax::{ParallaxX, ParallaxY};
    pub(crate) use super::plugin::CompositionPlugin;
    pub use super::plugin::CompositionSettings;
    pub(crate) use super::{LayersCameraSet, LightAnimSet, LightInteractionSet};
    pub use crate::defn_light;
}
