use super::biome::{Biome, BiomeLibrary};
use super::layers::{Layer, SoilLayer, TileLayer};
use super::tile::{Soil, Tile};
use crate::wg_utils::point_2d::Point2D;
use tcod::console::Console;

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub soils: SoilLayer,
    pub tiles: TileLayer,
    pub biome: Biome,
}

impl Map {
    pub fn create(
        width: i32,
        height: i32,
        soils: SoilLayer,
        tiles: TileLayer,
        biome: Biome,
    ) -> Self {
        Map {
            width,
            height,
            soils,
            tiles,
            biome,
        }
    }

    pub fn draw(&self, map_console: &mut tcod::console::Offscreen) -> () {
        for y in 0..self.height {
            for x in 0..self.width {
                let current_soil_ref: &Soil = self.soils.get_cell_ref_from_point(x, y);
                let current_tile_ref: &Tile = self.tiles.get_cell_ref_from_point(x, y);
                map_console.set_char_foreground(
                    x,
                    y,
                    tcod::Color::new(
                        current_tile_ref.color[0],
                        current_tile_ref.color[1],
                        current_tile_ref.color[2],
                    ),
                );
                map_console.set_char_background(
                    x,
                    y,
                    tcod::Color::new(
                        current_soil_ref.color[0],
                        current_soil_ref.color[1],
                        current_soil_ref.color[2],
                    ),
                    tcod::BackgroundFlag::Set,
                );

                map_console.print(x, y, &current_tile_ref.glyph);
            }
        }
    }
}

pub struct MapGenerator;

impl MapGenerator {
    pub fn generate_map(width: i32, height: i32) -> Map {
        let mut tiles: TileLayer = TileLayer::create_empty(width, height);
        let mut soils: SoilLayer = SoilLayer::create_empty(width, height);
        let biome = BiomeLibrary::get_biome_by_name("plains").unwrap();

        for index in 0..(width * height) {
            // let point: Point2D = Point2D::calc_point_from_index(index, width);
            let picked_soil = biome.pick_random_soil();
            let picked_tile = biome.pick_random_tile();
            soils.push_cell(picked_soil);
            tiles.push_cell(picked_tile);
        }

        Map::create(width, height, soils, tiles, biome)
    }
}
