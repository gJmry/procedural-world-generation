pub struct Tile {
    height: f32,
    width: f32,
    biome: BiomeType,
}


pub enum BiomeType {
    Ocean,
    Beach,
    Plains,
    Forest,
    Mountain
}