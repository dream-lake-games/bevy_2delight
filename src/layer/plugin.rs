use bevy::prelude::*;

use crate::layer::{
    camera::{setup_smush_camera, LayersCameraPlugin},
    layer::setup_all_layers,
    light::LayersLightPlugin,
    parallax::LayersParallaxPlugin,
};

pub struct LayerSettings {
    pub screen_size: UVec2,
    pub overlay_growth: u32,
    pub window: Window,
    pub asset_plugin: AssetPlugin,
}
impl Clone for LayerSettings {
    fn clone(&self) -> Self {
        Self {
            screen_size: self.screen_size,
            overlay_growth: self.overlay_growth,
            window: self.window.clone(),
            asset_plugin: AssetPlugin {
                file_path: self.asset_plugin.file_path.clone(),
                processed_file_path: self.asset_plugin.processed_file_path.clone(),
                meta_check: self.asset_plugin.meta_check.clone(),
                mode: match self.asset_plugin.mode {
                    AssetMode::Processed => AssetMode::Processed,
                    AssetMode::Unprocessed => AssetMode::Unprocessed,
                },
                watch_for_changes_override: self.asset_plugin.watch_for_changes_override,
            },
        }
    }
}
impl Default for LayerSettings {
    fn default() -> Self {
        let overlay_growth = 4;
        Self {
            screen_size: UVec2::new(240, 240),
            overlay_growth,
            window: Window {
                resizable: true,
                title: "CHANGE THE TITLE".to_string(),
                resolution: bevy::window::WindowResolution::new(
                    240.0 * overlay_growth as f32,
                    240.0 * overlay_growth as f32,
                ),
                // mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                mode: bevy::window::WindowMode::Windowed,
                ..default()
            },
            asset_plugin: AssetPlugin {
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            },
        }
    }
}

#[derive(Resource, Clone)]
pub(crate) struct LayerRes {
    /// How big is the world screen
    pub screen_size: UVec2,
    /// How many multiples of the screen size are things like menu, overlay, transition...
    pub overlay_growth: u32,
    /// Root component
    _root_eid: Entity,
}
impl LayerRes {
    pub(crate) fn root_eid(&self) -> Entity {
        self._root_eid
    }
}

pub(crate) fn init_root_eid(mut commands: Commands, mut layers_res: ResMut<LayerRes>) {
    let root_eid = commands
        .spawn((
            Name::new("LayersRoot"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();
    layers_res._root_eid = root_eid;
}

pub(crate) struct LayerPlugin {
    pub(crate) settings: LayerSettings,
}
impl LayerPlugin {
    pub(crate) fn new(settings: LayerSettings) -> Self {
        Self { settings }
    }
}
impl Plugin for LayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: self.settings.asset_plugin.file_path.clone(),
                    processed_file_path: self.settings.asset_plugin.processed_file_path.clone(),
                    meta_check: self.settings.asset_plugin.meta_check.clone(),
                    mode: match self.settings.asset_plugin.mode {
                        AssetMode::Processed => AssetMode::Processed,
                        AssetMode::Unprocessed => AssetMode::Unprocessed,
                    },
                    watch_for_changes_override: self
                        .settings
                        .asset_plugin
                        .watch_for_changes_override,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(self.settings.window.clone()),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );
        app.add_plugins(LayersLightPlugin);
        app.add_plugins(LayersCameraPlugin);
        app.add_plugins(LayersParallaxPlugin);

        app.insert_resource(LayerRes {
            screen_size: self.settings.screen_size,
            overlay_growth: self.settings.overlay_growth,
            _root_eid: Entity::PLACEHOLDER,
        });

        app.add_systems(
            Startup,
            (init_root_eid, setup_all_layers, setup_smush_camera).chain(),
        );
    }
}
