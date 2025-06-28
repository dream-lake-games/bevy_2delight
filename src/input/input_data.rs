use bevy::{platform::collections::HashMap, prelude::*};
use strum_macros::EnumIter;

#[derive(Clone, Debug, Reflect, Copy, EnumIter, PartialEq, Eq, Hash)]
pub enum Stick {
    Left,
    Right,
}

#[derive(Clone, Debug, Reflect, Copy, EnumIter, PartialEq, Eq, Hash)]
pub enum Butt {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug, Reflect, Default)]
pub struct StickFrame(HashMap<Stick, Vec2>);
impl StickFrame {
    pub fn dir(&self, stick: Stick) -> Vec2 {
        self.0.get(&stick).cloned().unwrap_or_default()
    }
}

#[derive(Clone, Debug, Reflect, Default)]
pub(crate) struct PressData {
    pub(crate) pressed: bool,
    pub(crate) just_pressed: bool,
    pub(crate) just_released: bool,
    pub(crate) held: bool,
}

#[derive(Clone, Debug, Reflect)]
pub struct PressFrame<K: std::hash::Hash + Eq>(HashMap<K, PressData>);
impl<K: std::hash::Hash + Eq> Default for PressFrame<K> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}
impl<K: std::hash::Hash + Eq> PressFrame<K> {
    pub fn pressed(&self, butt: K) -> bool {
        self.0.get(&butt).map_or(false, |data| data.pressed)
    }
    pub fn just_pressed(&self, butt: K) -> bool {
        self.0.get(&butt).map_or(false, |data| data.just_pressed)
    }
    pub fn just_released(&self, butt: K) -> bool {
        self.0.get(&butt).map_or(false, |data| data.just_released)
    }
    pub fn held(&self, butt: K) -> bool {
        self.0.get(&butt).map_or(false, |data| data.held)
    }

    pub fn any_pressed(&self, butts: impl IntoIterator<Item = K>) -> bool {
        butts.into_iter().any(|butt| self.pressed(butt))
    }
    pub fn any_just_pressed(&self, butts: impl IntoIterator<Item = K>) -> bool {
        butts.into_iter().any(|butt| self.just_pressed(butt))
    }
    pub fn any_just_released(&self, butts: impl IntoIterator<Item = K>) -> bool {
        butts.into_iter().any(|butt| self.just_released(butt))
    }
    pub fn any_held(&self, butts: impl IntoIterator<Item = K>) -> bool {
        butts.into_iter().any(|butt| self.held(butt))
    }

    pub fn all_pressed(&self, butts: impl IntoIterator<Item = K>) -> bool {
        butts.into_iter().all(|butt| self.pressed(butt))
    }
    pub fn all_just_pressed(&self, butts: impl IntoIterator<Item = K>) -> bool {
        butts.into_iter().all(|butt| self.just_pressed(butt))
    }
    pub fn all_just_released(&self, butts: impl IntoIterator<Item = K>) -> bool {
        butts.into_iter().all(|butt| self.just_released(butt))
    }
    pub fn all_held(&self, butts: impl IntoIterator<Item = K>) -> bool {
        butts.into_iter().all(|butt| self.held(butt))
    }
}

#[derive(Clone, Debug, Reflect, Default)]
pub struct FrameRecord {
    pub sticks: StickFrame,
    pub butts: PressFrame<Butt>,
    pub combos: PressFrame<ComboKey>,
}
impl FrameRecord {}

pub const INPUT_HISTORY_LENGTH: usize = 12;
pub type ComboKey = u32;

#[derive(Resource, Default)]
pub struct Input {
    frame_history: [FrameRecord; INPUT_HISTORY_LENGTH],
    combo_map: HashMap<
        ComboKey,
        Box<dyn Fn(&[FrameRecord; INPUT_HISTORY_LENGTH]) -> bool + 'static + Sync + Send>,
    >,
    pub sticks: StickFrame,
    pub butts: PressFrame<Butt>,
    pub combos: PressFrame<ComboKey>,
}
impl Input {
    pub(crate) fn add_frame(&mut self, record: FrameRecord) {
        for i in (1..INPUT_HISTORY_LENGTH).rev() {
            self.frame_history[i] = self.frame_history[i - 1].clone();
        }
        self.frame_history[0] = record;
    }

    pub fn add_combo(
        &mut self,
        key: ComboKey,
        trigger: impl Fn(&[FrameRecord; INPUT_HISTORY_LENGTH]) -> bool + 'static + Sync + Send,
    ) {
        debug_assert!(!self.combo_map.contains_key(&key));
        self.combo_map.insert(key, Box::new(trigger));
    }
}
