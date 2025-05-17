use bevy::prelude::*;
use crate::blocks::types::BlockType;
use crate::world::init::{CHUNK_HEIGHT, CHUNK_SIZE, MAX_INSTANCES_PER_CHUNK};

// 实例化数据
#[repr(C)]
#[derive(Component, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct InstanceData {
    pub position: [f32; 3],
    pub block_type: u32,
}

// GPU缓冲组件
#[derive(Component)]
pub struct ChunkInstanceBuffer {
    pub buffer: bevy::render::render_resource::Buffer,
    pub length: u32,
}

// 区块数据结构
#[derive(Clone)]
pub struct Chunk {
    pub blocks: [[[BlockType; CHUNK_HEIGHT]; CHUNK_SIZE]; CHUNK_SIZE],
    pub instance_data: Vec<InstanceData>,
    pub mesh_handle: Handle<Mesh>,
    pub(crate) is_dirty: bool,
}
#[derive(Component)]
pub struct ChunkCoord(pub i32, pub i32); // 跟踪区块坐标

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: [[[BlockType::Air; CHUNK_HEIGHT]; CHUNK_SIZE]; CHUNK_SIZE],
            instance_data: Vec::with_capacity(MAX_INSTANCES_PER_CHUNK),
            mesh_handle: Handle::default(),
            is_dirty: false,
        }
    }

    pub fn mark_dirty(&mut self){
        self.is_dirty = true;
    }
}