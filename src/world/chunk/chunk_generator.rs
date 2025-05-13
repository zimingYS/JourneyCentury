use bevy::prelude::*;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use crate::blocks::types::BlockType;
use crate::world::chunk::init::{Chunk, InstanceData};
use crate::world::chunk::mesh_builder::build_greedy_mesh;
use crate::world::init::{CHUNK_HEIGHT, CHUNK_SIZE};

pub fn generate_chunk(
    x: i32,
    z: i32,
    seed: u32,
    meshes: &mut Assets<Mesh>,
) -> Chunk {
    let mut blocks = [[[BlockType::Air; CHUNK_HEIGHT]; CHUNK_SIZE]; CHUNK_SIZE];
    let mut instance_data = Vec::new();

    let noise = Perlin::new(seed);
    let fbm = Fbm::<Perlin>::new(seed + 1)
        .set_octaves(3)
        .set_persistence(0.5);

    for local_x in 0..CHUNK_SIZE {
        for local_z in 0..CHUNK_SIZE {
            let world_x = x * CHUNK_SIZE as i32 + local_x as i32;
            let world_z = z * CHUNK_SIZE as i32 + local_z as i32;

            let base_noise = noise.get([world_x as f64 / 150.0, world_z as f64 / 150.0]);
            let fractal = fbm.get([world_x as f64 / 80.0, world_z as f64 / 80.0]);

            let height = ((base_noise * 25.0 + fractal * 15.0) + 80.0) as usize;

            for y in 0..height.min(CHUNK_HEIGHT - 1) {
                blocks[local_x][local_z][y] = match y {
                    h if h == height - 1 => BlockType::Grass,
                    h if h > height.saturating_sub(6) => BlockType::Dirt,
                    _ => BlockType::Stone,
                };

                if blocks[local_x][local_z][y] != BlockType::Air {
                    instance_data.push(InstanceData {
                        position: [world_x as f32, y as f32, world_z as f32],
                        block_type: blocks[local_x][local_z][y] as u32,
                    });
                }
            }
        }
    }

    let mesh = build_greedy_mesh(&instance_data);
    Chunk {
        blocks,
        instance_data,
        mesh_handle: meshes.add(mesh),
    }
}