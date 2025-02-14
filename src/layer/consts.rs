use bevy::color::Color;

/// Anything further back than this gets culled by the camera(s)
pub const ZIX_MIN: f32 = -600.0;
/// Anything further forward than this gets culled by the camera(s)
pub const ZIX_MAX: f32 = 600.0;
/// Don't fully understand why but this works for clear color, can't be ClearColorConfig::None
pub const COLOR_NONE: Color = Color::linear_rgba(0.0, 0.0, 0.0, 0.0);
