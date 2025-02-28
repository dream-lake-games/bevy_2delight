use bevy::prelude::*;
use bevy::reflect::Reflect;
use bevy::render::view::RenderLayers;
use bevy::utils::HashMap;

use crate::composition::prelude::Layer;
use crate::prelude::Frac;

use super::traits::AnimStateMachine;

#[derive(Debug, Clone, Reflect, PartialEq)]
pub enum AnimNextState<NextType> {
    Stay,
    Some(NextType),
    Despawn,
    Remove,
}

/// Data for a specific frame of an animation. If this is unchanged between frames,
/// it means we don't actually need to do anything when driving animations.
#[derive(Clone, Debug, Reflect)]
pub(super) struct AnimFrameData<StateMachine: AnimStateMachine> {
    pub(super) state: StateMachine,
    pub(super) ix: u32,
    pub(super) flip_x: bool,
    pub(super) flip_y: bool,
}
impl<StateMachine: AnimStateMachine> Default for AnimFrameData<StateMachine> {
    fn default() -> Self {
        Self {
            state: default(),
            ix: 0,
            flip_x: false,
            flip_y: false,
        }
    }
}
impl<StateMachine: AnimStateMachine> AnimFrameData<StateMachine> {
    fn new(state: StateMachine) -> Self {
        Self { state, ..default() }
    }
}
impl<StateMachine: AnimStateMachine> PartialEq for AnimFrameData<StateMachine> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
            && self.ix == other.ix
            && self.flip_x == other.flip_x
            && self.flip_y == other.flip_y
    }
}

/// A generic struct for communicating back potential updates frame-to-frame
#[derive(Event, Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct AnimDelta<T: Clone + std::fmt::Debug + Eq + std::hash::Hash + PartialEq + Reflect> {
    pub this_frame: T,
    pub last_frame: Option<T>,
}

/// When attached to entities with an AnimMan, events will be triggered when the state changes.
#[derive(Component, Clone, Debug, Reflect)]
pub struct AnimObserveStateChanges;

/// The main animation controller
#[derive(Component, Clone, Debug)]
#[require(Transform, Visibility)]
pub struct AnimMan<StateMachine: AnimStateMachine> {
    /// Current data this frame of the animation
    pub(super) this_frame: AnimFrameData<StateMachine>,
    /// Data from last frame of the animation
    pub(super) last_frame: Option<AnimFrameData<StateMachine>>,
    /// How much time has been spent on this ix of the animation
    pub(super) time: Frac,
    /// The render layer of the animation
    pub(super) render_layers: RenderLayers,
    /// INTERNAL: More ergonomic way to get to the body
    pub(super) body: Entity,
    /// INTERNAL: Hold these strong handles to prevent flickering
    pub(super) handle_map: HashMap<StateMachine, Handle<Image>>,
}

impl<StateMachine: AnimStateMachine> Default for AnimMan<StateMachine> {
    fn default() -> Self {
        Self {
            this_frame: default(),
            last_frame: None,
            time: Frac::ZERO,
            render_layers: Layer::Static.render_layers(),
            body: Entity::PLACEHOLDER,
            handle_map: default(),
        }
    }
}

/// Initialization implementation
impl<StateMachine: AnimStateMachine> AnimMan<StateMachine> {
    pub fn new(state: StateMachine) -> Self {
        Self {
            this_frame: AnimFrameData::new(state),
            ..default()
        }
    }
    pub fn with_initial_ix(mut self, ix: u32) -> Self {
        self.this_frame.ix = ix;
        self
    }
    pub fn with_flip_x(mut self, val: bool) -> Self {
        self.this_frame.flip_x = val;
        self
    }
    pub fn with_flip_y(mut self, val: bool) -> Self {
        self.this_frame.flip_y = val;
        self
    }
    pub fn with_layer(mut self, layer: Layer) -> Self {
        self.render_layers = layer.render_layers();
        self
    }
}

/// Get implementation
impl<StateMachine: AnimStateMachine> AnimMan<StateMachine> {
    pub fn get_state(&self) -> StateMachine {
        self.this_frame.state
    }
    pub fn get_ix(&self) -> u32 {
        self.this_frame.ix
    }
    pub fn get_flip_x(&self) -> bool {
        self.this_frame.flip_x
    }
    pub fn get_flip_y(&self) -> bool {
        self.this_frame.flip_y
    }
    /// Returns information about any state changes happening this frame
    pub fn delta_state(&self) -> Option<AnimDelta<StateMachine>> {
        if Some(self.this_frame.state) != self.last_frame.as_ref().map(|f| f.state) {
            Some(AnimDelta {
                this_frame: self.this_frame.state,
                last_frame: self.last_frame.as_ref().map(|f| f.state),
            })
        } else {
            None
        }
    }
    /// Returns information about any ix changes happening this frame
    pub fn delta_ix(&self) -> Option<AnimDelta<(StateMachine, u32)>> {
        if Some((self.this_frame.state, self.this_frame.ix))
            != self.last_frame.as_ref().map(|f| (f.state, f.ix))
        {
            Some(AnimDelta {
                this_frame: (self.this_frame.state, self.this_frame.ix),
                last_frame: self.last_frame.as_ref().map(|f| (f.state, f.ix)),
            })
        } else {
            None
        }
    }
    /// Returns information about any flipx changes happening this frame
    pub fn delta_flip_x(&self) -> Option<AnimDelta<bool>> {
        if Some(self.this_frame.flip_x) != self.last_frame.as_ref().map(|f| f.flip_x) {
            Some(AnimDelta {
                this_frame: self.this_frame.flip_x,
                last_frame: self.last_frame.as_ref().map(|f| f.flip_x),
            })
        } else {
            None
        }
    }
    /// Returns information about any flipy changes happening this frame
    pub fn delta_flip_y(&self) -> Option<AnimDelta<bool>> {
        if Some(self.this_frame.flip_y) != self.last_frame.as_ref().map(|f| f.flip_y) {
            Some(AnimDelta {
                this_frame: self.this_frame.flip_y,
                last_frame: self.last_frame.as_ref().map(|f| f.flip_y),
            })
        } else {
            None
        }
    }
}

/// Set implementation
impl<StateMachine: AnimStateMachine> AnimMan<StateMachine> {
    /// If the given state is equal to the current state, nothing happens.
    /// Otherwise, the state is changed to the given state, and the animation is reset to the first frame.
    pub fn set_state(&mut self, state: StateMachine) {
        if self.this_frame.state != state {
            self.reset_state(state);
        }
    }
    /// The given state is set, and the animation is reset to the first frame.
    pub fn reset_state(&mut self, state: StateMachine) {
        self.this_frame.state = state;
        self.this_frame.ix = 0;
        self.time = Frac::ZERO;
    }
    /// Set the flipx value of the animation
    pub fn set_flip_x(&mut self, flip_x: bool) {
        self.this_frame.flip_x = flip_x;
    }
    /// Set the flipy value of the animation
    pub fn set_flip_y(&mut self, flip_y: bool) {
        self.this_frame.flip_y = flip_y;
    }
}
