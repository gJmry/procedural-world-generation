use rand::Rng;
use crate::models::tile::{Tile, BiomeType};

pub fn generate_world(width: u16, height: u16) {
    let mut world = Vec::new();
    let mut rng = rand::rng();
    print!("{:?}", rng.random::<u8>());

    for _ in 0..height {
        let mut row = Vec::new();
        for _ in 0..width {
            let height: f32 = rng.random::<f32>();
            let width: f32 = rng.random::<f32>();
            let biome: BiomeType = determine_biome(height);
            let tile = Tile {
                height,
                width,
                biome,
            };
        }
        row
    }
}

fn determine_biome(height: f32) -> BiomeType {
    if (height < 0.2) {
        BiomeType::Ocean
    } else if (height < 0.4) {
        BiomeType::Beach
    } else if (height < 0.6) {
        BiomeType::Plains
    } else if (height < 0.8) {
        BiomeType::Forest
    } else {
        return BiomeType::Mountain
    }
}