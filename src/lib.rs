#[cfg(feature = "dev")]
mod dev_tools;
mod game;
mod screen;
mod ui;

use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    prelude::*,
    window::{WindowMode, WindowResolution},
};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::linear_rgba(0.6, 0.6, 0.6, 1.0)));
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

        app.configure_sets(FixedUpdate, AppSet::FixedUpdate);

        app.add_systems(Startup, spawn_camera);

        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Tickery Tockery: Panic at the Clockery".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        resolution: WindowResolution::new(1280.0, 720.0),
                        mode: WindowMode::Windowed,
                        resizable: false,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.0),
                    },
                    ..default()
                }),
        );

        // Add other plugins.
        app.add_plugins((game::plugin, screen::plugin, ui::plugin));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum AppSet {
    TickTimers,
    RecordInput,
    Update,
    FixedUpdate,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle::default(),
        IsDefaultUiCamera,
    ));
}
