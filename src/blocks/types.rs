use bevy::reflect::Reflect;

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum BlockType {
    Air,
    Grass,
    Dirt,
    Stone,
}

impl BlockType {
    pub fn is_transparent(&self) -> bool {
        matches!(self, BlockType::Air)
    }

    pub fn texture_coords(&self) -> (f32, f32, f32, f32) {
        let tile_size = 1.0 / 16.0;
        match self {
            BlockType::Grass => (2.0, 3.0, tile_size, tile_size),
            BlockType::Dirt => (2.0, 2.0, tile_size, tile_size),
            BlockType::Stone => (1.0, 1.0, tile_size, tile_size),
            _ => (0.0, 0.0, 0.0, 0.0),
        }
    }
}