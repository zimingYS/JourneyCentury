use crate::blocks::types::BlockType;
use crate::player::init::Player;
use crate::world;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

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
    if let Ok(mut player_transform) = player_query.single_mut() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            // 水平旋转作用于玩家
            player_transform.rotate_y(-rotation.x * sensitivity);

            // 垂直旋转作用于摄像机（限制角度）
            let vertical_rot = -rotation.y * sensitivity;
            camera_transform.rotate_local_x(vertical_rot.clamp(-0.5, 0.5));
        }
    }
}

// 体素尺寸（假设为1.0）
const VOXEL_SIZE: f32 = 1.0;

// 射线检测体素世界，返回击中的方块坐标
pub(crate) fn raycast(
    ray: Ray3d,
    world: &ResMut<world::init::World>,
    max_distance: f32,
) -> Option<(i32, i32, i32)> {
    let origin = ray.origin;
    let direction = ray.direction.normalize();
    let mut t = 0.0;
    let mut current = origin.as_ivec3(); // 当前体素坐标（整数）
    let step = direction.signum().as_ivec3(); // 步进方向（±1）
    let mut t_max = Vec3::new(
        if direction.x > 0.0 {
            (current.x as f32 + 1.0 - origin.x) / (direction.x + f32::EPSILON)
        } else {
            (current.x as f32 - origin.x) / (direction.x + f32::EPSILON)
        },
        if direction.y > 0.0 {
            (current.y as f32 + 1.0 - origin.y) / (direction.y + f32::EPSILON)
        } else {
            (current.y as f32 - origin.y) / (direction.y + f32::EPSILON)
        },
        if direction.z > 0.0 {
            (current.z as f32 + 1.0 - origin.z) / (direction.z + f32::EPSILON)
        } else {
            (current.z as f32 - origin.z) / (direction.z + f32::EPSILON)
        },
    );
    let t_delta = Vec3::new(
        VOXEL_SIZE / (direction.x.abs() + f32::EPSILON),
        VOXEL_SIZE / (direction.y.abs() + f32::EPSILON),
        VOXEL_SIZE / (direction.z.abs() + f32::EPSILON),
    );

    while t < max_distance {
        // 检查当前体素是否为非空气方块
        if let Some(block) = world.get_block(current.x, current.y, current.z) {
            if *block != BlockType::Air {
                return Some((current.x, current.y, current.z));
            }
        }

        // 选择下一个碰撞的轴
        let axis = if t_max.x < t_max.y {
            if t_max.x < t_max.z { 0 } else { 2 }
        } else {
            if t_max.y < t_max.z { 1 } else { 2 }
        };

        // 移动到下一个体素
        current[axis] += step[axis];
        t = t_max[axis];
        t_max[axis] += t_delta[axis];
    }
    None
}
