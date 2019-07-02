use serde::Deserialize;
use std::fs;

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Config {
    pub MAP_WIDTH: i32,
    pub MAP_HEIGHT: i32,
    pub MAP_OFFSET_X: i32,
    pub MAP_OFFSET_Y: i32,
    pub SCREEN_WIDTH: i32,
    pub SCREEN_HEIGHT: i32,
}

impl Config {
    pub fn get() -> Self {
        let json_contents: String = fs::read_to_string("data/config.json").expect("Cannot read the config file!");
        let config: Config = serde_json::from_str(&json_contents).unwrap();

        Config {
            MAP_WIDTH: config.MAP_WIDTH,
            MAP_HEIGHT: config.MAP_HEIGHT,
            MAP_OFFSET_X: config.MAP_OFFSET_X,
            MAP_OFFSET_Y: config.MAP_OFFSET_Y,
            SCREEN_WIDTH: config.SCREEN_WIDTH,
            SCREEN_HEIGHT: config.SCREEN_HEIGHT,
        }
    }
}