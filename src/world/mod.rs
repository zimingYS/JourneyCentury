pub mod chunk;
pub mod terrain;
pub mod systems;

use bevy::prelude::*;

// 世界配置常量
pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const MAX_INSTANCES_PER_CHUNK: usize = 65536;

// 导出主要类型
pub use chunk::ChunkInstanceBuffer;
pub use terrain::World;

// 世界插件
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup,
                systems::setup::setup_world,
            )
            .add_systems(Startup,systems::sunlight::setup_lighting)
            .add_systems(Update, (
                systems::loading::update_loaded_chunks,
                systems::loading::process_generation_queue,
                systems::rendering::render_chunks,
                systems::sunlight::adjust_lighting,
            ));
    }
}