use bevy::pbr::{MeshMaterial3d, PbrBundle};
use bevy::prelude::{default, Commands, DetectChanges, Entity, Mesh3d, Query, Res, ResMut, Transform, With};
use bevy::render::render_resource::{BufferInitDescriptor, BufferUsages};
use bevy::render::renderer::RenderDevice;
use crate::world::{ChunkInstanceBuffer, World};

pub fn render_chunks(
    mut commands: Commands,
    world: Res<World>,
    mut render_device: ResMut<RenderDevice>,
    mut query: Query<Entity, With<ChunkInstanceBuffer>>,
) {
    //检查world是否更新
    if !world.is_changed() {
        return;
    }

    // 清除所有旧的区块实体
    for entity in &mut query {
        commands.entity(entity).despawn();
    }

    // 生成新的区块实体
    for chunk in &world.chunks {
        // 创建 GPU 缓冲区
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("chunk instance buffer"),
            contents: bytemuck::cast_slice(&chunk.instance_data),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        commands.spawn((
            PbrBundle {
                mesh: Mesh3d::from(chunk.mesh_handle.clone()),
                material: MeshMaterial3d::from(world.material.clone()),
                transform: Transform::IDENTITY,
                ..default()
            },
            ChunkInstanceBuffer {
                buffer,
                length: chunk.instance_data.len() as u32,
            },
        ));
    }
}