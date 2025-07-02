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
pub struct StickFrame(pub(crate) HashMap<Stick, Vec2>);
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
}

#[derive(Clone, Debug, Reflect)]
pub struct PressFrame<K: std::hash::Hash + Eq>(pub(crate) HashMap<K, PressData>);
impl<K: std::hash::Hash + Eq> Default for PressFrame<K> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}
impl<K: std::hash::Hash + Eq> PressFrame<K> {
    pub fn pressed(&self, key: K) -> bool {
        self.0.get(&key).map_or(false, |data| data.pressed)
    }
    pub fn just_pressed(&self, key: K) -> bool {
        self.0.get(&key).map_or(false, |data| data.just_pressed)
    }
    pub fn just_released(&self, key: K) -> bool {
        self.0.get(&key).map_or(false, |data| data.just_released)
    }

    pub fn any_pressed(&self, keys: impl IntoIterator<Item = K>) -> bool {
        keys.into_iter().any(|key| self.pressed(key))
    }
    pub fn any_just_pressed(&self, keys: impl IntoIterator<Item = K>) -> bool {
        keys.into_iter().any(|key| self.just_pressed(key))
    }
    pub fn any_just_released(&self, keys: impl IntoIterator<Item = K>) -> bool {
        keys.into_iter().any(|key| self.just_released(key))
    }

    pub fn all_pressed(&self, keys: impl IntoIterator<Item = K>) -> bool {
        keys.into_iter().all(|key| self.pressed(key))
    }
    pub fn all_just_pressed(&self, keys: impl IntoIterator<Item = K>) -> bool {
        keys.into_iter().all(|key| self.just_pressed(key))
    }
    pub fn all_just_released(&self, keys: impl IntoIterator<Item = K>) -> bool {
        keys.into_iter().all(|key| self.just_released(key))
    }
}

#[derive(Clone, Debug, Reflect, Default)]
pub struct InputFrame {
    pub sticks: StickFrame,
    pub butts: PressFrame<Butt>,
    pub combos: PressFrame<ComboKey>,
}
impl InputFrame {}

pub const INPUT_HISTORY_LENGTH: usize = 12;
pub type ComboKey = u32;
type ComboTriggerInput = [InputFrame; INPUT_HISTORY_LENGTH];

#[derive(Resource, Default)]
pub struct Input {
    pub(crate) frame_history: ComboTriggerInput,
    pub(crate) combo_map:
        HashMap<ComboKey, Box<dyn Fn(&ComboTriggerInput) -> bool + 'static + Sync + Send>>,
    pub sticks: StickFrame,
    pub butts: PressFrame<Butt>,
    pub combos: PressFrame<ComboKey>,
}
impl Input {
    pub(crate) fn add_frame(&mut self, record: InputFrame) {
        for i in (1..INPUT_HISTORY_LENGTH).rev() {
            self.frame_history[i] = self.frame_history[i - 1].clone();
        }
        self.frame_history[0] = record;
    }

    pub(crate) fn correct_current_frame_combos(&mut self, combos: PressFrame<ComboKey>) {
        self.frame_history[0].combos = combos;
    }

    pub fn add_combo(
        &mut self,
        key: ComboKey,
        trigger: impl Fn(&ComboTriggerInput) -> bool + 'static + Sync + Send,
    ) {
        debug_assert!(!self.combo_map.contains_key(&key));
        self.combo_map.insert(key, Box::new(trigger));
    }
}
impl std::fmt::Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Input")
            .field("frame_history", &self.frame_history)
            .field("combo_map_keys", &self.combo_map.keys().collect::<Vec<_>>())
            .field("sticks", &self.sticks)
            .field("butts", &self.butts)
            .field("combos", &self.combos)
            .finish()
    }
}
