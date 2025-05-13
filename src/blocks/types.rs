use bevy::reflect::Reflect;

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum BlockType {
    Air,
    Grass,
    Dirt,
    Stone,
}
