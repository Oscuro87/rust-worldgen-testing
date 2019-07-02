
use rand::prelude::*;
use std::fs;
use tcod::console::Console;

#[derive(Debug, Clone, Deserialize)]
pub struct Tile {
    pub name: String,
    pub display_name: String,
    pub glyph: String,
    pub fg_color: Vec<u8>,
    pub bg_color: Vec<u8>,
    pub weight: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Biome {
    pub name: String,
    pub display_name: String,
    pub tiles: Vec<Tile>,
}

impl Biome {
    fn pick_random_tile(&self) -> Tile {
        let mut rng = rand::thread_rng();
        let slice = self.tiles.as_slice();
        slice
            .choose_weighted(&mut rng, |item| item.weight)
            .unwrap()
            .clone()
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

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn create(width: i32, height: i32, tiles: Vec<Tile>) -> Self {
        Map {
            width,
            height,
            tiles,
        }
    }

    pub fn get_tile_at(&self, x: i32, y: i32) -> &Tile {
        assert!(x >= 0 && y >= 0 && x < self.width && y < self.height);
        &self.tiles[(x + self.width * y) as usize]
    }

    // fn get_map_tile_at(map: &Map, x: i32, y: i32) -> &Tile {
    //     assert!(x >= 0 && y >= 0 && x < map.width && y < map.height);
    //     &map.tiles[(x + map.width * y) as usize]
    // }

    pub fn draw(&self, map_console: &mut tcod::console::Offscreen) -> () {
        for y in 0..self.height {
            for x in 0..self.width {
                let current_tile_ref: &Tile = self.get_tile_at(x, y);
                map_console.set_default_foreground(tcod::Color::new(
                    current_tile_ref.fg_color[0],
                    current_tile_ref.fg_color[1],
                    current_tile_ref.fg_color[2],
                ));
                map_console.set_default_background(tcod::Color::new(
                    current_tile_ref.bg_color[0],
                    current_tile_ref.bg_color[1],
                    current_tile_ref.bg_color[2],
                ));
                map_console.print(x, y, &current_tile_ref.glyph);
            }
        }
    }
}

pub struct MapGenerator;

impl MapGenerator {
    pub fn generate_map(width: i32, height: i32) -> Map {
        let mut tiles: Vec<Tile> = vec![];
        let biome = BiomeLibrary::get_biome_by_name("plains").unwrap();

        for x in 0..(width * height) {
            let picked_tile = biome.pick_random_tile();
            tiles.push(picked_tile);
        }

        Map::create(width, height, tiles)
    }
}