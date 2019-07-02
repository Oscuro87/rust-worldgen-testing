#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate serde_json;
use rand::prelude::*;
use std::fs;
use tcod::Console;

mod wg_core;
use wg_core::Config::Config;

#[derive(Debug, Clone, Deserialize)]
struct Tile {
    name: String,
    display_name: String,
    glyph: String,
    fg_color: Vec<u8>,
    bg_color: Vec<u8>,
    weight: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct Biome {
    name: String,
    display_name: String,
    tiles: Vec<Tile>,
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

struct BiomeLibrary {
    library: Vec<Biome>,
}

impl BiomeLibrary {
    fn load_from_json() -> Self {
        let json_contents: String =
            fs::read_to_string("data/biomes.json").expect("Cannot read the biomes file!");
        let all_biomes: Vec<Biome> = serde_json::from_str(&json_contents).unwrap();

        BiomeLibrary {
            library: all_biomes,
        }
    }

    fn get_biome_by_name(name: &str) -> Result<Biome, ()> {
        let loaded = BiomeLibrary::load_from_json();

        for b in loaded.library {
            if b.name == name {
                return Ok(b);
            }
        }

        Err(())
    }
}

struct Map {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
}

impl Map {
    fn create(width: i32, height: i32, tiles: Vec<Tile>) -> Self {
        Map {
            width,
            height,
            tiles,
        }
    }

    fn get_tile_at(&self, x: i32, y: i32) -> &Tile {
        assert!(x >= 0 && y >= 0 && x < self.width && y < self.height);
        &self.tiles[(x + self.width * y) as usize]
    }

    // fn get_map_tile_at(map: &Map, x: i32, y: i32) -> &Tile {
    //     assert!(x >= 0 && y >= 0 && x < map.width && y < map.height);
    //     &map.tiles[(x + map.width * y) as usize]
    // }

    fn draw(&self, map_console: &mut tcod::console::Offscreen) -> () {
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

struct MapGenerator;

impl MapGenerator {
    fn generate_simple_plain_map(width: i32, height: i32) -> Map {
        let mut tiles: Vec<Tile> = vec![];
        let biome = BiomeLibrary::get_biome_by_name("plains").unwrap();

        for x in 0..(width * height) {
            let picked_tile = biome.pick_random_tile();
            tiles.push(picked_tile);
        }

        Map::create(width, height, tiles)
    }
}

struct World {
    map: Map,
    log: GameLog,
    turn_no: u32,
}

impl World {
    fn new() -> Self {
        World {
            map: MapGenerator::generate_simple_plain_map(
                Config::get().MAP_WIDTH,
                Config::get().MAP_HEIGHT,
            ),
            log: GameLog::create(),
            turn_no: 0,
        }
    }

    fn draw(
        &self,
        map_console: &mut tcod::console::Offscreen,
        log_console: &mut tcod::console::Offscreen,
        info_console: &mut tcod::console::Offscreen,
        entities_console: &mut tcod::console::Offscreen,
    ) -> () {
        self.map.draw(map_console);
        self.log.draw(log_console);
    }

    fn end_turn(&mut self) -> () {
        self.turn_no += 1;
        self.log
            .push_message(String::from(format!("Turn# {}", self.turn_no)));
    }


    fn tick(&mut self, root: &mut tcod::RootConsole) -> bool {
        // Key events
        let key = root.wait_for_keypress(true);

        if key.code == tcod::input::KeyCode::Escape {
            return false;
        }

        if key.printable == 'r' {
            self.map = MapGenerator::generate_simple_plain_map(
                Config::get().MAP_WIDTH,
                Config::get().MAP_HEIGHT,
            );
            self.log.push_message(String::from("MAP REBUILT!!"));
        }

        if key.code == tcod::input::KeyCode::Enter || key.code == tcod::input::KeyCode::NumPadEnter
        {
            self.end_turn();
        }

        true
    }
}

struct GameLog {
    messages: Vec<String>,
}

impl GameLog {
    fn create() -> Self {
        GameLog { messages: vec![] }
    }

    fn draw(&self, log_console: &mut tcod::console::Offscreen) -> () {
        if self.messages.len() == 0 {
            return;
        }

        let start_at_x = 3;
        let mut max_messages = log_console.height() - start_at_x;

        if max_messages > self.messages.len() as i32 {
            max_messages = self.messages.len() as i32;
        }

        let start_at: usize = self.messages.len();
        let end_at: i32 = start_at as i32 - max_messages;

        println!("Start at: {} | End at: {}", start_at, end_at);

        for i in 0..max_messages as usize {
            let index: usize = (self.messages.len() - 1) - i;
            log_console.print(
                0,
                log_console.height() - i as i32 - 1,
                &self.messages[index],
            );
        }
    }

    fn push_message(&mut self, message: String) -> () {
        assert!(message.len() > 0);
        self.messages.push(message);
    }
}

fn main() {
    let mut root = tcod::RootInitializer::new()
        .font("data/arial10x10.png", tcod::FontLayout::Tcod)
        .font_type(tcod::FontType::Greyscale)
        .size(Config::get().SCREEN_WIDTH, Config::get().SCREEN_HEIGHT)
        .title("World Gen")
        .fullscreen(false)
        .renderer(tcod::Renderer::SDL)
        .init();

    let mut running: bool = true;
    let mut world = World::new();
    let mut map_console = tcod::OffscreenConsole::new(world.map.width, world.map.height);
    let mut entities_console = tcod::OffscreenConsole::new(world.map.width, world.map.height);
    let mut info_console = tcod::OffscreenConsole::new(30, 20);
    let mut log_console = tcod::OffscreenConsole::new(Config::get().SCREEN_WIDTH - 30, 20);

    while running {
        // Clear
        root.set_default_foreground(tcod::colors::WHITE);
        root.set_default_background(tcod::colors::BLACK);

        map_console.clear();
        log_console.clear();
        info_console.clear();
        root.clear();

        // Draw
        // Info screen
        info_console.print(1, 1, "INFO");
        info_console.horizontal_line(0, 0, info_console.width(), tcod::BackgroundFlag::None);
        info_console.vertical_line(0, 0, info_console.height(), tcod::BackgroundFlag::None);
        info_console.print(0, 0, "#");
        // Log screen
        log_console.horizontal_line(0, 0, log_console.width(), tcod::BackgroundFlag::None);
        log_console.print(0, 1, "LOGS");
        // World
        world.draw(&mut map_console, &mut log_console, &mut info_console, &mut entities_console);

        // Blit stuff
        tcod::console::blit(
            &map_console,
            (0, 0),
            (0, 0),
            &mut root,
            (Config::get().MAP_OFFSET_X, Config::get().MAP_OFFSET_Y),
            1.0,
            1.0,
        );
        tcod::console::blit(
            &log_console,
            (0, 0),
            (0, 0),
            &mut root,
            (0, Config::get().SCREEN_HEIGHT - info_console.height()),
            1.0,
            1.0,
        );
        tcod::console::blit(
            &info_console,
            (0, 0),
            (0, 0),
            &mut root,
            (
                Config::get().SCREEN_WIDTH - info_console.width(),
                Config::get().SCREEN_HEIGHT - info_console.height(),
            ),
            1.0,
            1.0,
        );

        root.flush();

        running = world.tick(&mut root);
    }
}
