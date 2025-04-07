use crate::world::chunk::build_greedy_mesh;
use bevy::prelude::*;
use bevy::render::render_resource::Buffer;
use noise::{NoiseFn, Perlin};
use rand::Rng;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
const MAX_INSTANCES_PER_CHUNK: usize = 65536;

// 实例化数据结构
#[repr(C)]  // 强制C内存布局
#[derive(Component, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceData {
    pub(crate) position: [f32; 3],  // 使用数组代替Vec3
    pub(crate) block_type: u32,
}

//管理GPU缓冲区和实例数量
#[derive(Component)]
pub struct ChunkInstanceBuffer {
    pub(crate) buffer: Buffer,
    pub(crate) length: u32,
}


//全局资源，存储所有区块和共享材质
#[derive(Resource)]
pub struct World {
    pub chunks: Vec<Chunk>,
    pub material_map: Handle<StandardMaterial>,
}

//单个区块数据
#[derive(Clone)]
pub struct Chunk {
    pub instance_data: Vec<InstanceData>,  // 实例化数据
    pub mesh_handle: Handle<Mesh>,         // 关联的网格
}

// 设置世界
pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 生成基础立方体mesh（所有方块共享）
    //let cube_mesh = meshes.add(Mesh::from(Cuboid::new(1.0,1.0,1.0)));

    // 生成材质
    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 0.5, 0.5),
        perceptual_roughness: 0.9,
        ..default()
    });

    //生成世界种子
    let seed = rand::thread_rng().gen_range(1..u32::MAX);

    //使用柏林噪声生成地形
    let perlin = Perlin::new(seed);

    //构建区块
    let mut chunks = Vec::new();

    // 生成区块地形
    for chunk_x in 0..16 {
        for chunk_z in 0..16 {
            let mut instance_data = Vec::new();

            //每个区块填充
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let height = ((perlin.get([
                        (chunk_x * CHUNK_SIZE + x) as f64 / 64.0,
                        (chunk_z * CHUNK_SIZE + z) as f64 / 64.0
                    ]) + 1.0) * 32.0) as usize;

                    for y in 0..height.min(CHUNK_HEIGHT) {
                        instance_data.push(InstanceData {
                            position: <[f32; 3]>::from(Vec3::new(
                                (chunk_x * CHUNK_SIZE + x) as f32,
                                y as f32,
                                (chunk_z * CHUNK_SIZE + z) as f32
                            )),
                            block_type: if y == height-1 { 0 } else { 1 },
                        });
                    }
                }
            }

            let mesh = build_greedy_mesh(&instance_data);
            let mesh_handle = meshes.add(mesh);

            chunks.push(Chunk {
                instance_data,
                mesh_handle,
            });
        }
    }

    commands.insert_resource(World {
        chunks,
        material_map: material,
    });

    // 添加太阳光源
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::rgb(0.98, 0.95, 0.82),
                illuminance: 100_000.0,
                shadows_enabled: false,
                shadow_depth_bias: 0.2,
                shadow_normal_bias: 0.8,
                ..default()
            },
            transform: Transform::from_rotation(
                Quat::from_euler(EulerRot::XYZ, -45.0f32.to_radians(), 45.0f32.to_radians(), 0.0)
            ),
            ..default()
        },
        Name::new("Sunlight"),
    ));

    // 环境光配置
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0,
    });
}