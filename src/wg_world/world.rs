use super::map::{ Map, MapGenerator };
use super::super::wg_gui::gameLog::GameLog;
use super::super::wg_core::config::Config;


pub struct World {
    pub map: Map,
    pub log: GameLog,
    pub turn_no: u32,
}

impl World {
    pub fn new() -> Self {
        World {
            map: MapGenerator::generate_map(Config::get().MAP_WIDTH, Config::get().MAP_HEIGHT),
            log: GameLog::create(),
            turn_no: 0,
        }
    }

    pub fn draw(
        &self,
        map_console: &mut tcod::console::Offscreen,
        log_console: &mut tcod::console::Offscreen,
        info_console: &mut tcod::console::Offscreen,
        entities_console: &mut tcod::console::Offscreen,
    ) -> () {
        self.map.draw(map_console);
        self.log.draw(log_console);
    }

    pub fn end_turn(&mut self) -> () {
        self.turn_no += 1;
        self.log
            .push_message(String::from(format!("Turn# {}", self.turn_no)));
    }


    pub fn tick(&mut self, root: &mut tcod::RootConsole) -> bool {
        // Key events
        let key = root.wait_for_keypress(true);

        if key.code == tcod::input::KeyCode::Escape {
            return false;
        }

        if key.printable == 'r' {
            self.map =
                MapGenerator::generate_map(Config::get().MAP_WIDTH, Config::get().MAP_HEIGHT);
            self.log.push_message(String::from("MAP REBUILT!!"));
        }

        if key.code == tcod::input::KeyCode::Enter || key.code == tcod::input::KeyCode::NumPadEnter
        {
            self.end_turn();
        }

        true
    }
}