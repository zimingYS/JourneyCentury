use crate::world::chunk;
use crate::world::chunk::init::{ChunkCoord, ChunkInstanceBuffer};
use bevy::prelude::*;
use bevy::render::render_resource::{BufferInitDescriptor, BufferUsages};
use bevy::render::renderer::RenderDevice;

pub fn render_chunks(
    mut commands: Commands,
    world: Res<chunk::init::World>,
    render_device: ResMut<RenderDevice>,
    chunk_query: Query<(Entity, &ChunkCoord)>,
) {
    // 卸载不再存在的区块实体
    let loaded_coords: std::collections::HashSet<_> = world.chunks.keys().collect();
    for (entity, coord) in chunk_query.iter() {
        if !loaded_coords.contains(&(coord.0, coord.1)) {
            commands.entity(entity).despawn();
        }
    }

    // 生成新区块实体
    for ((x, z), chunk) in world.chunks.iter() {
        if chunk_query.iter().any(|(_, c)| c.0 == *x && c.1 == *z) {
            continue;
        }

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("chunk instance buffer"),
            contents: bytemuck::cast_slice(&chunk.instance_data),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        commands.spawn((
            Mesh3d::from(chunk.mesh_handle.clone()),
            MeshMaterial3d::from(world.material.clone()),
            ChunkCoord(*x, *z),
            ChunkInstanceBuffer {
                buffer,
                length: chunk.instance_data.len() as u32,
            },
        ));
    }
}
