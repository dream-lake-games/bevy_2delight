use bevy::prelude::*;
use serde_json::Value;

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct TagInfo {
    pub w: u32,
    pub h: u32,
    pub length: u32,
}
impl TagInfo {
    pub fn from_path(
        path: &std::path::PathBuf,
    ) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        // Read and parse the JSON file
        let contents = std::fs::read_to_string(path)?;
        let json: Value = serde_json::from_str(&contents)?;

        // Get the frames object
        let frames = json
            .get("frames")
            .and_then(|f| f.as_object())
            .ok_or("Missing or invalid 'frames' object")?;

        // Count total frames
        let frame_count = frames.len() as u32;

        // Get any arbitrary frame (we'll take the first one)
        let (_, first_frame) = frames.iter().next().ok_or("No frames found")?;

        // Extract source size from the first frame
        let source_size = first_frame
            .get("sourceSize")
            .and_then(|s| s.as_object())
            .ok_or("Missing or invalid 'sourceSize' object")?;

        let width = source_size
            .get("w")
            .and_then(|w| w.as_u64())
            .ok_or("Invalid width")? as u32;

        let height = source_size
            .get("h")
            .and_then(|h| h.as_u64())
            .ok_or("Invalid height")? as u32;

        Ok(TagInfo {
            w: width,
            h: height,
            length: frame_count,
        })
    }
}

#[derive(Resource, Reflect, Debug)]
pub(super) struct AnimRes<StateMachine: AnimStateMachine> {
    size: UVec2,
    lengths: HashMap<StateMachine, u32>,
    has_brightness: bool,
    has_reflexivity: bool,
}
impl<StateMachine: AnimStateMachine> AnimRes<StateMachine> {
    pub fn load() -> Self {
        let pixel_tag_infos = StateMachine::iter()
            .map(|state| {
                let path = state.get_pixel_jsonpath();
                (state, TagInfo::from_path(&path).unwrap())
            })
            .collect::<Vec<_>>();
        let size = UVec2::new(pixel_tag_infos[0].1.w, pixel_tag_infos[0].1.h);
        let brightness_path =
            std::path::Path::new("assets").join(StateMachine::default().get_brightness_filepath());
        let reflexivity_path =
            std::path::Path::new("assets").join(StateMachine::default().get_reflexivity_filepath());
        Self {
            size,
            lengths: pixel_tag_infos
                .into_iter()
                .map(|(state, t)| (state, t.length))
                .collect(),
            has_brightness: brightness_path.exists(),
            has_reflexivity: reflexivity_path.exists(),
        }
    }

    pub fn get_size(&self) -> UVec2 {
        self.size
    }
    pub fn get_length(&self, state: StateMachine) -> u32 {
        *self.lengths.get(&state).unwrap_or(&1)
    }
    pub fn has_brightness(&self) -> bool {
        self.has_brightness
    }
    pub fn has_reflexivity(&self) -> bool {
        self.has_reflexivity
    }
}

#[cfg(debug_assertions)]
fn on_reload_anims<StateMachine: AnimStateMachine>(
    _trigger: Trigger<super::anim_plugin::ReloadAnims>,
    mut anim_res: ResMut<AnimRes<StateMachine>>,
) {
    *anim_res = AnimRes::load();
}

pub(super) fn register_anim_res<StateMachine: AnimStateMachine>(app: &mut App) {
    app.insert_resource(AnimRes::<StateMachine>::load());
    #[cfg(debug_assertions)]
    {
        app.add_observer(on_reload_anims::<StateMachine>);
    }
}
