use super::biome::{ Biome, BiomeLibrary };
use super::tile::Tile;
use crate::wg_utils::point_2d::Point2D;
use tcod::console::Console;

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
    pub biome: Biome,
}

impl Map {
    pub fn create(width: i32, height: i32, tiles: Vec<Tile>, biome: Biome) -> Self {
        Map {
            width,
            height,
            tiles,
            biome,
        }
    }

    pub fn get_tile_at(&self, x: i32, y: i32) -> &Tile {
        assert!(x >= 0 && y >= 0 && x < self.width && y < self.height);
        &self.tiles[(x + self.width * y) as usize]
    }

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
    fn generate_cluster_at(tile_mold: Tile, epicenter: Point2D) {
        unimplemented!()
    }

    fn does_clusterize(tile: Tile) -> bool {
        
    }

    pub fn generate_map(width: i32, height: i32) -> Map {
        let mut tiles: Vec<Tile> = vec![];
        let biome = BiomeLibrary::get_biome_by_name("plains").unwrap();

        for _ in 0..(width * height) {
            let picked_tile = biome.pick_random_tile();
            tiles.push(picked_tile);
        }

        Map::create(width, height, tiles, biome)
    }
}
