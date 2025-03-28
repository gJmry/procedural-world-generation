pub struct Tile {
    pub height: f32,
    pub width: f32,
    pub biome: BiomeType,
}

pub enum BiomeType {
    Ocean,
    Beach,
    Plains,
    Forest,
    Mountain
}