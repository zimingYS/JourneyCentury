use crate::blocks::types::BlockType;
use crate::player::Player;
use crate::world::chunk::{build_greedy_mesh, Chunk, InstanceData};
use crate::world::{terrain, CHUNK_HEIGHT, CHUNK_SIZE};
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

pub fn update_loaded_chunks(
    mut world: ResMut<terrain::World>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };

    // 计算玩家所在区块坐标
    let (current_x, current_z) = (
        (player_transform.translation.x / CHUNK_SIZE as f32).floor() as i32,
        (player_transform.translation.z / CHUNK_SIZE as f32).floor() as i32
    );

    // 加载范围（视距）
    const LOAD_RADIUS: i32 = 8;
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
    mut world: ResMut<terrain::World>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    const MAX_PER_FRAME: usize = 2;
    for _ in 0..MAX_PER_FRAME {
        if let Some((x, z)) = world.generation_queue.pop_front() {
            let chunk = generate_chunk(x, z, &world.noise, &mut meshes);
            world.chunks.insert((x, z), chunk);
        } else {
            break;
        }
    }
}

fn generate_chunk(
    x: i32,
    z: i32,
    noise: &Perlin,
    meshes: &mut Assets<Mesh>,
) -> Chunk {
    let mut blocks = [[[BlockType::Air; CHUNK_HEIGHT]; CHUNK_SIZE]; CHUNK_SIZE];
    let mut instance_data = Vec::new();

    for local_x in 0..CHUNK_SIZE {
        for local_z in 0..CHUNK_SIZE {
            let world_x = x * CHUNK_SIZE as i32 + local_x as i32;
            let world_z = z * CHUNK_SIZE as i32 + local_z as i32;

            let height = ((noise.get([world_x as f64 / 50.0, world_z as f64 / 50.0]) * 40.0) + 60.0) as usize;

            for y in 0..height.min(CHUNK_HEIGHT - 1) {
                blocks[local_x][local_z][y] = match y {
                    h if h == height - 1 => BlockType::Grass,
                    h if h > height.saturating_sub(6) => BlockType::Dirt,
                    _ => BlockType::Stone,
                };

                if blocks[local_x][local_z][y] != BlockType::Air {
                    instance_data.push(InstanceData {
                        position: [world_x as f32, y as f32, world_z as f32],
                        block_type: blocks[local_x][local_z][y] as u32,
                    });
                }
            }
        }
    }

    let mesh = build_greedy_mesh(&instance_data);
    Chunk {
        blocks,
        instance_data,
        mesh_handle: meshes.add(mesh),
    }
}