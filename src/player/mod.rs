pub mod input;
pub mod camera;

use bevy::prelude::*;
use crate::player::camera::PlayerCamera;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        SpatialBundle::from_transform(Transform::from_xyz(0.0, 64.0, 0.0)),
        Name::new("Player"),
    )).with_children(|parent| { // 添加子实体（摄像机）
        parent.spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 1.8, 0.0) // 玩家头顶位置
                    .looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            PlayerCamera,
        ));
    });
}