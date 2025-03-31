use std::fmt;
use serde::Serialize;

#[derive(Serialize)]
pub struct Tile {
    pub height: f64,
    pub moisture: f64,
    pub biome: BiomeType,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Height: {}, Moisture: {}, Biome: {}",
               self.height, self.moisture, self.biome)
    }
}

#[derive(Serialize)]
pub enum BiomeType {
    Ocean,
    Beach,
    Plains,
    Forest,
    Mountain,
    Desert,
}
impl fmt::Display for BiomeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            BiomeType::Ocean => "Ocean",
            BiomeType::Beach => "Beach",
            BiomeType::Plains => "Plains",
            BiomeType::Forest => "Forest",
            BiomeType::Mountain => "Mountain",
            BiomeType::Desert => "Desert",
        };
        write!(f, "{}", name)
    }
}