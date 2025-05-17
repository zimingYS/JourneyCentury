use bevy::prelude::*;
use crate::player::init::Player;
use crate::world;
use crate::world::chunk::chunk_generator::generate_chunk;
use crate::world::init::{CHUNK_SIZE, LOAD_RADIUS, MAX_PER_FRAME};

pub fn update_loaded_chunks(
    mut world: ResMut<world::init::World>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.single() else { return };

    // 计算玩家所在区块坐标
    let (current_x, current_z) = (
        (player_transform.translation.x / CHUNK_SIZE as f32).floor() as i32,
        (player_transform.translation.z / CHUNK_SIZE as f32).floor() as i32
    );

    let mut required_chunks = Vec::new();

    // 生成需要加载的区块坐标
    for x in (current_x - LOAD_RADIUS)..=(current_x + LOAD_RADIUS) {
        for z in (current_z - LOAD_RADIUS)..=(current_z + LOAD_RADIUS) {
            required_chunks.push((x, z));
        }
    }

    // 添加新区块到生成队列
    for coord in &required_chunks {
        if !world.chunks.contains_key(coord) && !world.generation_queue.contains(coord) {
            world.generation_queue.push_back(*coord);
        }
    }

    // 卸载超出范围的区块
    let mut to_remove = Vec::new();
    for (coord, _) in world.chunks.iter() {
        if (coord.0 - current_x).abs() > LOAD_RADIUS ||
            (coord.1 - current_z).abs() > LOAD_RADIUS {
            to_remove.push(*coord);
        }
    }
    for coord in to_remove {
        world.chunks.remove(&coord);
    }
}

pub fn process_generation_queue(
    mut world: ResMut<world::init::World>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for _ in 0..MAX_PER_FRAME {
        if let Some((x, z)) = world.generation_queue.pop_front() {
            let chunk = generate_chunk(x, z, world.seed, &mut meshes);
            world.chunks.insert((x, z), chunk);
        } else {
            break;
        }
    }
}