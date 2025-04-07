use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};

mod player;
mod world;
mod rendering;

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
            world::terrain::setup_world,
        ))
        .add_systems(Update, (
            player::input::keyboard_movement,
            player::camera::mouse_look,
            world::chunk::render_chunks,
        ))
        .run();
}