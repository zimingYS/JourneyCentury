use bevy::pbr::{MeshMaterial3d, PbrBundle};
use bevy::prelude::{default, Commands, DetectChanges, Entity, Mesh3d, Query, Res, ResMut, Transform, With};
use bevy::render::render_resource::{BufferInitDescriptor, BufferUsages};
use bevy::render::renderer::RenderDevice;
use bevy::utils::HashSet;
use crate::world::{terrain, ChunkInstanceBuffer, World};
use crate::world::terrain::ChunkCoord;

pub fn render_chunks(
    mut commands: Commands,
    world: Res<World>,
    mut render_device: ResMut<RenderDevice>,
    chunk_query: Query<(Entity, &ChunkCoord)>,
) {
    // 卸载不再存在的区块实体
    let loaded_coords: HashSet<_> = world.chunks.keys().collect();
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
            PbrBundle {
                mesh: Mesh3d::from(chunk.mesh_handle.clone()),
                material: MeshMaterial3d::from(world.material.clone()),
                ..default()
            },
            ChunkCoord(*x, *z),
            ChunkInstanceBuffer {
                buffer,
                length: chunk.instance_data.len() as u32,
            },
        ));
    }
}


// pub fn render_chunks(
//     mut commands: Commands,
//     world: Res<World>,
//     mut render_device: ResMut<RenderDevice>,
//     mut query: Query<Entity, With<ChunkInstanceBuffer>>,
// ) {
//     //检查world是否更新
//     if !world.is_changed() {
//         return;
//     }
//
//     // 清除所有旧的区块实体
//     for entity in &mut query {
//         commands.entity(entity).despawn();
//     }
//
//     // 生成新的区块实体
//     for chunk in &world.chunks {
//         // 创建 GPU 缓冲区
//         let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
//             label: Some("chunk instance buffer"),
//             contents: bytemuck::cast_slice(&chunk.instance_data),
//             usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
//         });
//
//         commands.spawn((
//             PbrBundle {
//                 mesh: Mesh3d::from(chunk.mesh_handle.clone()),
//                 material: MeshMaterial3d::from(world.material.clone()),
//                 transform: Transform::IDENTITY,
//                 ..default()
//             },
//             ChunkInstanceBuffer {
//                 buffer,
//                 length: chunk.instance_data.len() as u32,
//             },
//         ));
//     }
// }