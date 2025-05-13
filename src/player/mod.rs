pub mod input;
pub mod camera;

use bevy::prelude::*;
use crate::player::camera::PlayerCamera;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Transform::from_xyz(-2.5, 4.5, 9.0), // 玩家初始位置保持不变
        Name::new("Player"),
    )).with_children(|parent| { // 添加子实体（摄像机）
        parent.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 100.0, 0.0).looking_at(Vec3::NEG_Z, Vec3::Y),
            PlayerCamera,
        ));
    });
}