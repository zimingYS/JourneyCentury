use crate::player::camera::PlayerCamera;
use bevy::pbr::{Atmosphere, AtmosphereSettings};
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Transform::from_xyz(0.0, 50.0, 0.0), // 玩家初始位置保持不变
        Name::new("Player"),
    )).with_children(|parent| { // 添加子实体（摄像机）
        parent.spawn((
            Camera3d::default(),
            // 启用HDR
            Camera {
                hdr:true,
                ..Default::default()
            },
            // 启用大气散射的件
            Atmosphere::EARTH,
            AtmosphereSettings {
                ..Default::default()
            },
            Transform::from_xyz(0.0, 51.8, 0.0).looking_at(Vec3::NEG_Z, Vec3::Y),
            PlayerCamera,
        ));
    });
}