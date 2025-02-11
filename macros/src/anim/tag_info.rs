use serde_json::Value;

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
