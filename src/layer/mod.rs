use bevy::prelude::*;

pub mod prelude {
    pub use super::camera::camera_shake::CameraShake;
    pub use super::camera::DynamicCamera;
    pub use super::layer::{
        layer_defns::{
            BgLayer, FgLayer, LightLayer, MainAmbienceLayer, MainDetailLayer, MainStaticLayer,
            MenuLayer, OverlayLayer, TransitionLayer,
        },
        Layer,
    };
    pub use super::light::light_man::{LightDefnPlugin, LightMan, LightStateMachine};
    pub use super::parallax::{ParallaxX, ParallaxY};
    pub use super::plugin::*;
    pub use super::{LayersCameraSet, LightAnimSet, LightInteractionSet};
}

mod camera;
mod consts;
mod layer;
mod layer_utils;
mod light;
mod parallax;
mod plugin;

/// This is a render layer that explicitly doesn't render
pub const DUMMY_LAYER_USIZE: usize = 0;

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
