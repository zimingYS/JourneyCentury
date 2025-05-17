use bevy::prelude::{Assets, Mesh, ResMut};
use crate::world;
use crate::world::chunk::chunk_generator::regenerate_mesh;
use crate::world::init::MAX_PER_FRAME;

// 更新脏区块
pub fn update_dirty_chunks(
    mut world: ResMut<world::init::World>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut dirty_coords = Vec::new();

    // 收集所有脏区块的坐标
    for (coord, chunk) in world.chunks.iter() {
        if chunk.is_dirty {
            println!("Found dirty chunk: ({}, {})", coord.0, coord.1); // 关键日志：确认脏区块被收集
            dirty_coords.push(*coord);
        }
    }

    for coord in dirty_coords.into_iter().take(MAX_PER_FRAME) {
        if let Some(chunk) = world.chunks.get_mut(&coord) {
            // 仅重新生成网格（使用现有 blocks 数组）
            let (new_instance_data, new_mesh_handle) =
                regenerate_mesh(&chunk.blocks, coord.0, coord.1, &mut meshes);
            chunk.instance_data = new_instance_data;
            chunk.mesh_handle = new_mesh_handle;
            chunk.is_dirty = false; // 清除脏标记
        }
    }
}