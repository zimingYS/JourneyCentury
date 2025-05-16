use bevy::prelude::{Assets, Mesh, ResMut};
use crate::world::chunk;
use crate::world::chunk::chunk_generator::generate_chunk;
use crate::world::init::MAX_PER_FRAME;

// 更新脏区块
pub fn update_dirty_chunks(
    mut world: ResMut<chunk::init::World>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut dirty_coords = Vec::new();

    // 收集所有脏区块的坐标
    for (coord, chunk) in world.chunks.iter() {
        if chunk.is_dirty {
            dirty_coords.push(*coord);
        }
    }

    let seed = world.seed;
    for coord in dirty_coords.into_iter().take(MAX_PER_FRAME) {
        if let Some(chunk) = world.chunks.get_mut(&coord) {
            // 重新生成区块（复用 generate_chunk 逻辑）
            let new_chunk = generate_chunk(coord.0, coord.1, seed, &mut meshes);
            chunk.instance_data = new_chunk.instance_data;
            chunk.mesh_handle = new_chunk.mesh_handle;
            chunk.is_dirty = false; // 清除脏标记
        }
    }
}