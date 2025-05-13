use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use crate::world::chunk::init::InstanceData;

#[derive(PartialEq, Eq, Hash)]
struct BlockKey {
    x: i32,
    y: i32,
    z: i32,
}

pub fn build_greedy_mesh(instance_data: &[InstanceData]) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    let mut colors = Vec::new();

    let blocks: std::collections::HashSet<BlockKey> = instance_data.iter()
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

        let block_color = get_block_color(instance.block_type);

        // 检查六个相邻方向
        for (dx, dy, dz, normal) in directions {
            let neighbor_pos = (pos.0 + dx, pos.1 + dy, pos.2 + dz);
            if !blocks.contains(&BlockKey {
                x: neighbor_pos.0,
                y: neighbor_pos.1,
                z: neighbor_pos.2,
            }) {
                // 生成可见的面
                let vertices = generate_face_vertices(pos, normal);
                let base_index = positions.len() as u32;

                positions.extend(vertices.iter().map(|v| *v));
                normals.extend(vec![normal; 4]);

                let uv = get_uv_coords(instance.block_type);
                uvs.extend(vec![
                    [uv.0, uv.1],     // 左下
                    [uv.2, uv.1],     // 右下
                    [uv.2, uv.3],     // 右上
                    [uv.0, uv.3],     // 左上
                ]);

                indices.extend(&[
                    base_index, base_index + 1, base_index + 2,
                    base_index, base_index + 2, base_index + 3,
                ]);

                colors.extend(vec![block_color; 4]);
            }
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
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

fn get_block_color(block_type: u32) -> [f32; 4] {
    match block_type {
        1 => [0.0, 1.0, 0.0, 1.0], // 草方块（绿色）
        2 => [0.6, 0.3, 0.1, 1.0], // 泥土（棕色）
        3 => [0.5, 0.5, 0.5, 1.0], // 石头（灰色）
        _ => [1.0, 1.0, 1.0, 1.0], // 默认白色
    }
}