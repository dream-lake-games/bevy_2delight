use bevy::{prelude::*, reflect::Reflectable, utils::HashMap};

use super::{man::AnimNextState, time::AnimTimeClass};

pub trait AnimStateMachine:
    Sized
    + Send
    + Sync
    + 'static
    + std::hash::Hash
    + PartialEq
    + Eq
    + Copy
    + Default
    + FromReflect
    + Reflectable
    + std::fmt::Debug
{
    // type Layer: Layer; TODO

    const SIZE: UVec2;
    const ZIX: f32;
    const TIME_CLASS: Option<AnimTimeClass>;
    const REP: UVec2;

    fn make_handle_map(ass: &Res<AssetServer>) -> HashMap<Self, Handle<Image>>;

    fn get_filepath(&self) -> &'static str;

    fn get_length(&self) -> u32;

    fn get_fps(&self) -> u32;

    fn get_offset(&self) -> IVec2;

    fn get_next(&self) -> AnimNextState<Self>;
}

pub trait AnimTimeProvider: Resource + std::fmt::Debug + Default {
    fn get_delta_us(&self, class: AnimTimeClass) -> u32;
}
