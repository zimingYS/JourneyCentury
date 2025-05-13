use bevy::app::{App, Plugin, PluginGroup};
use bevy::DefaultPlugins;
use bevy::prelude::{default, Window, WindowPlugin};
use bevy::window::{CursorGrabMode, CursorOptions};

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
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
        ));
    }
}