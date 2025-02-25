use bevy::ecs::schedule::SystemSet;

mod camera;
mod layer;
// mod light;
mod parallax;
mod plugin;

/// The set that handles driving underlying light animations. Happens during `PostUpdate`, before `AnimSet`.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LightAnimSet;

/// The set that handles light interaction. Happens during `Update`, after `PhysicsSet`.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LightInteractionSet;

/// The set that internally handles updating layer cameras. This happens in `Update`.
/// NOTE: This is the system that places all the cameras. You must make sure the pos is correct before this system.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LayersCameraSet;

pub mod prelude {
    pub use super::camera::{CameraShake, DynamicCamera};
    pub use super::layer::Layer;
    pub use super::parallax::{ParallaxX, ParallaxY};
    pub(crate) use super::plugin::CompositionPlugin;
    pub use super::plugin::CompositionSettings;
    pub use super::{LayersCameraSet, LightAnimSet, LightInteractionSet};
}
