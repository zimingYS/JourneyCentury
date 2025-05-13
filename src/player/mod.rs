pub mod input;
pub mod camera;
pub mod init;
use bevy::prelude::*;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            init::spawn_player,
        ))
        .add_systems(Update, (
            input::keyboard_movement,
            camera::mouse_look,
        ));
    }
}