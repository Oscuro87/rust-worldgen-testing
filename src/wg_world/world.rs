use super::super::wg_core::config::Config;
use super::super::wg_gui::game_log::GameLog;
use super::super::wg_managers::turn_manager::TurnManager;
use super::map::{Map, MapGenerator};

pub struct World {
    pub map: Map,
    pub log: GameLog,
    pub turn_mgr: TurnManager,
}

impl World {
    pub fn new() -> Self {
        World {
            map: MapGenerator::generate_map(Config::get().MAP_WIDTH, Config::get().MAP_HEIGHT),
            log: GameLog::create(),
            turn_mgr: TurnManager::create(),
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
        self.turn_mgr.next_turn();
        self.log
            .push_message(String::from(format!("Turn# {}", self.turn_mgr.get_turn())));
    }

    pub fn tick(&mut self, root: &mut tcod::RootConsole) -> () {
        self.end_turn();
    }

    pub fn recreate_map(&mut self) -> () {
        self.map = MapGenerator::generate_map(Config::get().MAP_WIDTH, Config::get().MAP_HEIGHT);
        self.log.push_message(String::from("MAP REBUILT!!"));
    }
}
