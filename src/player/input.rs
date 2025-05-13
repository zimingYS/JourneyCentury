use bevy::prelude::*;
use crate::player::init::Player;

pub fn keyboard_movement(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = query.single_mut() else {
        warn!("Player entity not found!");
        return;
    };

    let speed = 5.0;
    let mut direction = Vec3::ZERO;

    // 前后左右移动（基于玩家当前朝向）
    if keyboard.pressed(KeyCode::KeyW) { direction.z += 1.0 }
    if keyboard.pressed(KeyCode::KeyS) { direction.z -= 1.0 }
    if keyboard.pressed(KeyCode::KeyA) { direction.x -= 1.0 }
    if keyboard.pressed(KeyCode::KeyD) { direction.x += 1.0 }

    // 垂直升降控制
    if keyboard.pressed(KeyCode::Space) { direction.y += 1.0 }
    if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) {
        direction.y -= 1.0;
    }

    if direction != Vec3::ZERO {
        // 获取基于玩家旋转的向量
        let forward = transform.forward();
        let right = transform.right();

        // 组合移动向量并归一化
        let move_vector = (forward * direction.z + right * direction.x + Vec3::Y * direction.y)
            .normalize_or_zero();

        transform.translation += move_vector * speed * time.delta_secs();
    }
}
