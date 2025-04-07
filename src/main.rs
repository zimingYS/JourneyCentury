use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};
use JourneyCentury::{player, world};
use JourneyCentury::world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "JourneyCentury".into(),
                    cursor_options: {
                        let mut cursor = CursorOptions::default();
                        cursor.visible = false;
                        cursor.grab_mode = CursorGrabMode::Locked;
                        cursor
                    },
                    ..default()
                }),
                ..default()
            }),
            bevy::diagnostic::LogDiagnosticsPlugin::default(),
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, (
            player::spawn_player,
            // world::systems::setup::setup_world,
        ))
        .add_systems(Update, (
            player::input::keyboard_movement,
            player::camera::mouse_look,
            // world::systems::rendering::render_chunks,
        ))
        .add_plugins(WorldPlugin)
        .run();
}