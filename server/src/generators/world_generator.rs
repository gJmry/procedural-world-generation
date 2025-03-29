use rand::Rng;
use crate::models::tile::{Tile, BiomeType};
use noise::{NoiseFn, Perlin};

pub fn generate_world(width: u16, height: u16) -> Vec<Vec<Tile>>{
    let mut world = Vec::new();
    let perlin = Perlin::new(42);
    let scale = 0.1;


    for x in 0..height {
        let mut row: Vec<Tile> = Vec::new();
        for y in 0..width {
            let x_coord = scale * (x as f64);
            let y_coord = scale * (y as f64);
            let height = (perlin.get([x_coord, y_coord, 0.0]) + 1.0) / 2.0;
            let moisture = (perlin.get([x_coord, y_coord, 1.0]) + 1.0) / 2.0;
            let biome: BiomeType = determine_biome(height, moisture);
            row.push(Tile {
                height,
                moisture,
                biome,
            });
        }
        world.push(row);
    }
    world
}

fn determine_biome(height: f64, moisture: f64) -> BiomeType {
    if height < 0.2 {
        BiomeType::Ocean
    } else if height < 0.3 {
        BiomeType::Beach
    } else {
        if moisture < 0.3 {
            BiomeType::Desert
        } else if moisture < 0.6 {
            BiomeType::Plains
        } else if height < 0.7 {
            BiomeType::Forest
        } else {
            BiomeType::Mountain
        }
    }
}