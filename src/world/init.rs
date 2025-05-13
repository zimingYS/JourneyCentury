use crate::world::chunk;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{default, Commands, ResMut};
use rand::Rng;
use std::collections::{HashMap, VecDeque};

// 世界配置常量
pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const MAX_INSTANCES_PER_CHUNK: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT;

// 加载范围（视距）
pub const LOAD_RADIUS: i32 = 8;

// 每帧最多生成 100 个方块（可调整）
pub const MAX_BLOCKS_PER_FRAME: usize = 100;

// 每帧渲染最大区块数
pub const MAX_PER_FRAME: usize = 2;

pub fn setup_world(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        perceptual_roughness: 0.9,
        ..default()
    });
    let seed = rand::thread_rng().gen_range(1..u32::MAX);

    commands.insert_resource(chunk::init::World {
        chunks: HashMap::new(),
        material,
        seed,
        generation_queue: VecDeque::new(),
    });
}