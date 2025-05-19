use crate::world::chunk::init::{ChunkCoord, ChunkInstanceBuffer};
use bevy::prelude::*;
use bevy::render::render_resource::{BufferInitDescriptor, BufferUsages};
use bevy::render::renderer::RenderDevice;
use crate::world;
use crate::world::chunk::chunk_generator::regenerate_mesh;

pub fn render_chunks(
    mut commands: Commands,
    mut world: ResMut<world::init::World>,
    render_device: ResMut<RenderDevice>,
    chunk_query: Query<(Entity, &ChunkCoord)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // 卸载不再存在的区块实体
    let loaded_coords: std::collections::HashSet<_> = world.chunks.keys().collect();
    for (entity, coord) in chunk_query.iter() {
        if !loaded_coords.contains(&(coord.0, coord.1)) {
            commands.entity(entity).despawn();
        }
    }

    // 生成新区块实体
    let material = world.material.clone();
    for ((x, z), chunk) in world.chunks.iter_mut() {
        if chunk_query.iter().any(|(_, c)| c.0 == *x && c.1 == *z) && !chunk.is_dirty{
            continue;
        }

        if chunk.is_dirty {

            for (entity, coord) in chunk_query.iter() {
                if coord.0 == *x && coord.1 == *z {
                    commands.entity(entity).despawn();
                    break;
                }
            }

            // 重新生成网格
            let (new_instance_data, new_mesh_handle) = regenerate_mesh(&chunk.blocks, *x, *z, &mut meshes);
            chunk.instance_data = new_instance_data;
            chunk.mesh_handle = new_mesh_handle;
            chunk.is_dirty = false; // 清除脏标记
        }

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("chunk instance buffer"),
            contents: bytemuck::cast_slice(&chunk.instance_data),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        commands.spawn((
            Mesh3d::from(chunk.mesh_handle.clone()),
            MeshMaterial3d::from(material.clone()),
            ChunkCoord(*x, *z),
            ChunkInstanceBuffer {
                buffer,
                length: chunk.instance_data.len() as u32,
            },
        ));
    }
}
