use crate::world::chunk;
use bevy::asset::{Assets, Handle};
use bevy::color::Color;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{default, Commands, ResMut, Resource};
use rand::Rng;
use std::collections::{HashMap, VecDeque};
use crate::blocks::types::BlockType;
use crate::world::chunk::init::Chunk;

// 世界配置常量
pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const MAX_INSTANCES_PER_CHUNK: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT;

// 加载范围（视距）
pub const LOAD_RADIUS: i32 = 8;

// 每帧最多生成 100 个方块（可调整）
pub const MAX_BLOCKS_PER_FRAME: usize = 100;

// 每帧渲染最大区块数和脏区块处理数
pub const MAX_PER_FRAME: usize = 2;

// 世界资源
#[derive(Resource)]
pub struct World {
    pub chunks: HashMap<(i32, i32), Chunk>,
    pub material: Handle<StandardMaterial>,
    pub seed: u32,
    pub generation_queue: VecDeque<(i32, i32)>,
}

impl World {
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<&BlockType>{
        let chunk_x = x.div_euclid(CHUNK_SIZE as i32);
        let chunk_z = z.div_euclid(CHUNK_SIZE as i32);
        let local_x = x.rem_euclid(CHUNK_SIZE as i32) as usize;
        let local_z = z.rem_euclid(CHUNK_SIZE as i32) as usize;
        let local_y = y as usize;
        println!(
            "Query block: world=({}, {}, {}), chunk=({}, {}), local=({}, {}, {})",
            x, y, z, chunk_x, chunk_z, local_x, local_y, local_z
        );
        self.chunks.get(&(chunk_x, chunk_z)).map(
            |chunk| &chunk.blocks[local_x][local_z][local_y]
        )
    }
    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: BlockType){
        let chunk_x = x.div_euclid(CHUNK_SIZE as i32);
        let chunk_z = z.div_euclid(CHUNK_SIZE as i32);
        let local_x = x.rem_euclid(CHUNK_SIZE as i32) as usize;
        let local_z = z.rem_euclid(CHUNK_SIZE as i32) as usize;
        let local_y = y as usize;

        self.chunks.get_mut(&(chunk_x, chunk_z)).map(
            |chunk| {
                let old_block = chunk.blocks[local_x][local_z][local_y];
                chunk.blocks[local_x][local_z][local_y] = block;
                println!(
                    "Set block: world=({}, {}, {}), chunk=({}, {}), local=({}, {}, {}), old={:?}, new={:?}",
                    x, y, z, chunk_x, chunk_z, local_x, local_y, local_z, old_block, block
                ); // 关键日志：确认方块被修改
            }
        );
    }
    pub fn mark_chunk_dirty(&mut self, chunk_x: i32, chunk_z: i32){
        self.chunks.get_mut(&(chunk_x, chunk_z)).map(
            |chunk| {
                println!("Marked chunk ({}, {}) as dirty", chunk_x, chunk_z); // 关键日志：确认脏标记
                chunk.mark_dirty()
            }
        );
    }
}


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

    commands.insert_resource(World {
        chunks: HashMap::new(),
        material,
        seed,
        generation_queue: VecDeque::new(),
    });
}