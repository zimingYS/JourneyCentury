pub mod world;
pub mod player;
pub mod rendering;
pub mod ui;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        TransformBundle::from(Transform::from_xyz(0.0, 32.0, 0.0)),
    ));
}