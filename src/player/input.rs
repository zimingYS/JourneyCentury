use crate::blocks::types::BlockType;
use crate::player::camera::raycast;
use crate::player::init::Player;
use crate::world;
use crate::world::init::CHUNK_SIZE;
use bevy::prelude::*;

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
// 最大破坏距离
const MAX_DESTROY_DISTANCE: f32 = 5.0;
pub fn mouse_input(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut world: ResMut<world::init::World>,
){
    // 破坏方块
    if mouse.just_pressed(MouseButton::Left) {
        let (_camera, camera_transform) = camera_query.single().unwrap();
        let ray_pos = camera_transform.translation();
        let ray_dir = camera_transform.forward();
        let ray = Ray3d::new(ray_pos,ray_dir);
        if let Some(block_pos) = raycast(ray, &world, MAX_DESTROY_DISTANCE) {
            world.set_block(block_pos.0,block_pos.1,block_pos.2,BlockType::Air);
            let chunk_x = block_pos.0.div_euclid(CHUNK_SIZE as i32);
            let chunk_z = block_pos.2.div_euclid(CHUNK_SIZE as i32);
            world.mark_chunk_dirty(chunk_x,chunk_z);
        }
    }
}