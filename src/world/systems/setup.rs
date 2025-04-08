use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use crate::blocks::types::BlockType;
use crate::world::{terrain, CHUNK_HEIGHT, CHUNK_SIZE};
use crate::world::chunk::{build_greedy_mesh, Chunk, InstanceData};

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
            // 初始化 blocks 数组
            let mut blocks = [[[BlockType::Air; CHUNK_HEIGHT]; CHUNK_SIZE]; CHUNK_SIZE];
            let mut instance_data = Vec::new();

            //每个区块填充
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {

                    // 计算世界坐标（区块局部坐标 → 全局坐标）
                    let world_x = chunk_x * CHUNK_SIZE + x;
                    let world_z = chunk_z * CHUNK_SIZE + z;


                    //使用噪声生成高度
                    let height = ((perlin.get([
                        world_x as f64 / 50.0,
                        world_z as f64 / 50.0
                    ])* 40.0) + 60.0) as usize;

                    //填充 blocks 数据
                    for y in 0..height.min(CHUNK_HEIGHT- 1) {
                        blocks[x][z][y] = match y {
                            h if h == height - 1 => BlockType::Grass,
                            h if h > height.saturating_sub(6) => BlockType::Dirt,
                            _ => BlockType::Stone,
                        };

                        // 生成实例数据
                        if blocks[x][z][y] != BlockType::Air {
                            instance_data.push(InstanceData {
                                position: [
                                    world_x as f32,
                                    y as f32,
                                    world_z as f32
                                ],
                                block_type: blocks[x][z][y] as u32,
                            });
                        }
                    }
                }
            }

            //生成网格
            let mesh = build_greedy_mesh(&instance_data);
            let mesh_handle = meshes.add(mesh);

            chunks.push(Chunk {
                blocks,
                instance_data,
                mesh_handle,
            });
        }
    }

    commands.insert_resource(terrain::World {
        chunks,
        material,
        noise: perlin,
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