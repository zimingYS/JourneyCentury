pub(crate) mod terrain;
pub(crate) mod chunk;

use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;


#[derive(Resource)]
pub struct World {
    pub chunks: Vec<Chunk>,
    pub material_map: Handle<StandardMaterial>, // 统一World结构体定义
}

#[derive(Clone)]
pub struct Chunk {
    pub blocks: [[[BlockType; CHUNK_HEIGHT]; CHUNK_SIZE]; CHUNK_SIZE],
}

#[derive(Clone, Copy, PartialEq)]
pub enum BlockType {
    Air,
    Grass,
    Dirt,
    Stone,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            blocks: [[[BlockType::Air; CHUNK_HEIGHT]; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }
}

pub fn generate_terrain(mut commands: Commands,mut materials: ResMut<Assets<StandardMaterial>>,) {
    let perlin = Perlin::new(1234);
    let mut chunk = Chunk::default();

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let height = ((perlin.get([x as f64 / 16.0, z as f64 / 16.0]) + 1.0) * 64.0) as usize;

            for y in 0..height {
                chunk.blocks[x][z][y] = if y == height - 1 {
                    BlockType::Grass
                } else if y > height - 4 {
                    BlockType::Dirt
                } else {
                    BlockType::Stone
                };
            }
        }
    }

    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.4, 0.2, 0.1),
        perceptual_roughness: 0.9,
        ..default()
    });

    commands.insert_resource(World { chunks: vec![chunk], material_map: material });
}