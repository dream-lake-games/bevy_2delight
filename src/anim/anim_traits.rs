use std::path::PathBuf;

use bevy::{camera::visibility::RenderLayers, prelude::*, reflect::Reflectable};

use super::{anim_man::AnimNextState, anim_time::AnimTimeClass};

use crate::prelude::*;

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
    + strum::IntoEnumIterator
{
    const RENDER_LAYERS: Option<RenderLayers>;
    const ZIX: f32;
    const TIME_CLASS: Option<AnimTimeClass>;
    const REP: UVec2;

    fn get_special_filepath(&self, prefix: Option<&str>) -> String;
    fn get_pixel_filepath(&self) -> String {
        self.get_special_filepath(None)
    }
    fn get_brightness_filepath(&self) -> String {
        self.get_special_filepath(Some("_brightness"))
    }
    fn get_reflexivity_filepath(&self) -> String {
        self.get_special_filepath(Some("_reflexivity"))
    }
    fn get_pixel_jsonpath(&self) -> PathBuf {
        let mut path = std::path::Path::new("assets").join(self.get_pixel_filepath());
        path.set_extension("json");
        path
    }

    fn make_special_handle_map(
        ass: &Res<AssetServer>,
        prefix: Option<&str>,
    ) -> HashMap<Self, Handle<Image>> {
        let mut result = HashMap::default();
        for state in Self::iter() {
            result.insert(state, ass.load(state.get_special_filepath(prefix)));
        }
        result
    }
    fn make_pixel_handle_map(ass: &Res<AssetServer>) -> HashMap<Self, Handle<Image>> {
        Self::make_special_handle_map(ass, None)
    }
    fn make_brightness_handle_map(ass: &Res<AssetServer>) -> HashMap<Self, Handle<Image>> {
        Self::make_special_handle_map(ass, Some("_brightness"))
    }
    fn make_reflexivity_handle_map(ass: &Res<AssetServer>) -> HashMap<Self, Handle<Image>> {
        Self::make_special_handle_map(ass, Some("_reflexivity"))
    }

    fn get_fps(&self) -> u32;

    fn get_offset(&self) -> IVec2;

    fn get_next(&self) -> AnimNextState<Self>;
}
