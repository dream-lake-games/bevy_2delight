use bevy::{asset::embedded_asset, prelude::*, sprite::Material2dPlugin};

use super::lit_mat::LitMat;

#[derive(Clone)]
pub struct CompositionSettings {
    pub title: String,
    pub screen_size: UVec2,
}
impl Default for CompositionSettings {
    fn default() -> Self {
        Self {
            title: "CHANGE ME TITLE".into(),
            screen_size: UVec2::new(300, 200),
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
                        resolution: bevy::window::WindowResolution::new(
                            self.settings.screen_size.x as f32,
                            self.settings.screen_size.y as f32,
                        ),
                        mode: bevy::window::WindowMode::BorderlessFullscreen(
                            MonitorSelection::Primary,
                        ),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );
        embedded_asset!(app, "lit_mat.wgsl");
        app.add_plugins(Material2dPlugin::<LitMat>::default());

        super::camera::register_camera(app);
        super::layer::register_layer(app, self.settings.screen_size);
        super::parallax::register_parallax(app);
        super::light::lighting::register_lighting(app);
        super::light::light_proc::register_light_proc(app);
    }
}
