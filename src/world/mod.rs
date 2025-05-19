pub mod chunk;
pub mod init;
mod light;

use bevy::prelude::*;
use crate::world::chunk::chunk_loader::{process_generation_queue, update_loaded_chunks};
use crate::world::chunk::chunk_render::render_chunks;
use crate::world::light::{adjust_suns, setup_suns};

// 世界插件
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                init::setup_world,
                setup_suns
            ))
            .add_systems(Update, (
                update_loaded_chunks,
                process_generation_queue,
                adjust_suns,
                render_chunks,
            ));
    }
}