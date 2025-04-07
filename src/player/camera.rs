use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use crate::player::Player;

#[derive(Component)]
pub struct PlayerCamera;


pub fn mouse_look(
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    mut mouse_events: EventReader<MouseMotion>,
) {
    let mut rotation = Vec2::ZERO;
    for event in mouse_events.read() {
        rotation += event.delta;
    }

    let sensitivity = 0.001;
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // 水平旋转作用于玩家
            player_transform.rotate_y(-rotation.x * sensitivity);

            // 垂直旋转作用于摄像机（限制角度）
            let vertical_rot = -rotation.y * sensitivity;
            camera_transform.rotate_local_x(vertical_rot.clamp(-0.5, 0.5));
        }
    }
}