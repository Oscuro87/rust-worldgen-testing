
use super::map::Map;
use crate::wg_utils::point_2d::Point2D;


pub struct Player {
    name: String,
    position: Point2D,
    glyph: String,
}

impl Player {
    pub fn create(name: String, starting_pos: Point2D, glyph: &str) -> Self {
        Player {
            name,
            position: starting_pos,
            glyph: String::from(glyph),
        }
    }

    // pub fn handle_key(&mut self, key_evt: &tcod::eve)
    
    pub fn do_move(&mut self, map: &Map, dx: i32, dy: i32) -> bool {
        false
    }
}