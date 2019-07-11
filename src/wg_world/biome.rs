use rand::prelude::*;
use super::tile::{ Tile, Soil };
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct Biome {
    pub name: String,
    pub display_name: String,
    pub tiles: Vec<Tile>,
    pub soils: Vec<Soil>,
}

impl Biome {
    pub fn pick_random_tile(&self) -> Tile {
        let mut rng = rand::thread_rng();
        let slice = self.tiles.as_slice();
        slice
            .choose_weighted(&mut rng, |item| item.meta.weight)
            .unwrap()
            .clone()
    }

    pub fn pick_random_soil(&self) -> Soil {
        let mut rng = rand::thread_rng();
        let slice = self.soils.as_slice();
        slice.choose_weighted(&mut rng, |item| item.meta.weight).unwrap().clone()
    }
}

pub struct BiomeLibrary {
    pub library: Vec<Biome>,
}

impl BiomeLibrary {
    pub fn load_from_json() -> Self {
        let json_contents: String =
            fs::read_to_string("data/biomes.json").expect("Cannot read the biomes file!");
        let all_biomes: Vec<Biome> = serde_json::from_str(&json_contents).unwrap();

        BiomeLibrary {
            library: all_biomes,
        }
    }

    pub fn get_biome_by_name(name: &str) -> Result<Biome, ()> {
        let loaded = BiomeLibrary::load_from_json();

        for b in loaded.library {
            if b.name == name {
                return Ok(b);
            }
        }

        Err(())
    }
}