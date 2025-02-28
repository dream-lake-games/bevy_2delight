use bevy::prelude::*;

#[derive(Clone)]
pub struct CompositionSettings {
    pub title: String,
    pub screen_size: UVec2,
}
impl Default for CompositionSettings {
    fn default() -> Self {
        Self {
            title: "CHANGE ME TITLE".into(),
            screen_size: UVec2::new(240, 240),
        }
    }
}

pub(crate) struct CompositionPlugin {
    settings: CompositionSettings,
}
impl CompositionPlugin {
    pub fn new(settings: CompositionSettings) -> Self {
        Self { settings }
    }
}
impl Plugin for CompositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: bevy::asset::AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        title: self.settings.title.clone(),
                        // resolution: bevy::window::WindowResolution::new(
                        //     10.0,
                        //     10.0, // self.settings.screen_size.x as f32,
                        //          // self.settings.screen_size.y as f32,
                        // ),
                        mode: bevy::window::WindowMode::BorderlessFullscreen(
                            MonitorSelection::Primary,
                        ),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );

        super::camera::register_camera(app);
        super::layer::register_layer(app, self.settings.screen_size);
        super::parallax::register_parallax(app);
    }
}
