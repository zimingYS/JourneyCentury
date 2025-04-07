use crate::world::terrain::{ChunkInstanceBuffer, InstanceData, World};
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_resource::{BufferInitDescriptor, BufferUsages};
use bevy::render::renderer::RenderDevice;
use bevy::utils::HashSet;

// 每帧最多生成 100 个方块（可调整）
const MAX_BLOCKS_PER_FRAME: usize = 100;

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
            MaterialMeshBundle {
                mesh: Mesh3d::from(chunk.mesh_handle.clone()),
                material: MeshMaterial3d::from(world.material_map.clone()),
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

#[derive(PartialEq, Eq, Hash)]
struct BlockKey {
    x: i32,
    y: i32,
    z: i32,
}
pub fn  build_greedy_mesh(instance_data: &[InstanceData]) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList,RenderAssetUsages::default());
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    let blocks: HashSet<BlockKey> = instance_data.iter()
        .map(|i| BlockKey {
            x: i.position[0] as i32,
            y: i.position[1] as i32,
            z: i.position[2] as i32,
        })
        .collect();

    // 遍历所有实例生成网格
    for instance in instance_data {
        let pos = (
            instance.position[0] as i32,
            instance.position[1] as i32,
            instance.position[2] as i32,
        );

        // 定义六个面方向
        let directions = [
            (1, 0, 0, Vec3::X),    // 右
            (-1, 0, 0, Vec3::NEG_X), // 左
            (0, 1, 0, Vec3::Y),    // 上
            (0, -1, 0, Vec3::NEG_Y), // 下
            (0, 0, 1, Vec3::Z),    // 前（Z+方向）
            (0, 0, -1, Vec3::NEG_Z), // 后（Z-方向）
        ];

        //检查六个相邻方向
        for (dx, dy, dz, normal) in directions {
            let neighbor_pos = (pos.0 + dx, pos.1 + dy, pos.2 + dz);
            //如果相邻位置没有方块，则生成该面
            if !blocks.contains(&BlockKey {
                x: neighbor_pos.0,
                y: neighbor_pos.1,
                z: neighbor_pos.2,
            }) {
                // 生成可见的面
                let vertices = generate_face_vertices(pos, normal);
                let base_index = positions.len() as u32;

                // 顶点位置
                positions.extend(vertices.iter().map(|v| *v));

                // 法线 (所有顶点相同)
                normals.extend(vec![normal; 4]);

                // UV 坐标 (根据方块类型调整)
                let uv = get_uv_coords(instance.block_type);
                uvs.extend(vec![
                    [uv.0, uv.1],     // 左下
                    [uv.2, uv.1],     // 右下
                    [uv.2, uv.3],     // 右上
                    [uv.0, uv.3],     // 左上
                ]);

                // 索引
                indices.extend(&[
                    base_index, base_index + 1, base_index + 2,
                    base_index, base_index + 2, base_index + 3,
                ]);
            }
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn generate_face_vertices(pos: (i32, i32, i32), normal: Vec3) -> [Vec3; 4] {
    let (x, y, z) = (pos.0 as f32, pos.1 as f32, pos.2 as f32);
    let offset = 0.501; // 轻微重叠避免接缝

    match normal {
        Vec3::X => [ // 右侧面（X+方向）
            Vec3::new(x + offset, y - offset, z + offset),
            Vec3::new(x + offset, y - offset, z - offset),
            Vec3::new(x + offset, y + offset, z - offset),
            Vec3::new(x + offset, y + offset, z + offset),
        ],
        Vec3::NEG_X => [ // 左侧面（X-方向）
            Vec3::new(x - offset, y - offset, z - offset),
            Vec3::new(x - offset, y - offset, z + offset),
            Vec3::new(x - offset, y + offset, z + offset),
            Vec3::new(x - offset, y + offset, z - offset),
        ],
        Vec3::Y => [ // 上面
            Vec3::new(x - offset, y + offset, z + offset),
            Vec3::new(x + offset, y + offset, z + offset),
            Vec3::new(x + offset, y + offset, z - offset),
            Vec3::new(x - offset, y + offset, z - offset),
        ],
        Vec3::NEG_Y => [ // 下面
            Vec3::new(x - offset, y - offset, z - offset),
            Vec3::new(x + offset, y - offset, z - offset),
            Vec3::new(x + offset, y - offset, z + offset),
            Vec3::new(x - offset, y - offset, z + offset),
        ],
        Vec3::Z => [ // 前面（Z+方向）
            Vec3::new(x - offset, y - offset, z + offset),
            Vec3::new(x + offset, y - offset, z + offset),
            Vec3::new(x + offset, y + offset, z + offset),
            Vec3::new(x - offset, y + offset, z + offset),
        ],
        Vec3::NEG_Z => [ // 后面（Z-方向）
            Vec3::new(x + offset, y - offset, z - offset),
            Vec3::new(x - offset, y - offset, z - offset),
            Vec3::new(x - offset, y + offset, z - offset),
            Vec3::new(x + offset, y + offset, z - offset),
        ],
        _ => panic!("Invalid normal"),
    }
}

// 获取不同方块类型的UV坐标 (假设使用16x16纹理图集)
fn get_uv_coords(block_type: u32) -> (f32, f32, f32, f32) {
    let tile_size = 1.0 / 16.0;
    let (row, col) = match block_type {
        0 => (0, 0),   // 空气（不可见）
        1 => (2, 3),   // 草方块
        2 => (2, 2),   // 泥土
        3 => (1, 1),   // 石头
        _ => (0, 0),
    };

    let u_min = col as f32 * tile_size;
    let u_max = u_min + tile_size;
    let v_min = 1.0 - (row as f32 + 1.0) * tile_size;
    let v_max = v_min + tile_size;

    (u_min, v_min, u_max, v_max)
}