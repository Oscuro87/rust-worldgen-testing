use super::super::wg_core::config::Config;
use super::super::wg_managers::turn_manager::TurnManager;
use super::map::{Map, MapGenerator};
use crate::wg_gui::game_log::GameLog;
use crate::wg_gui::info_screen::InfoScreen;

pub struct World {
    pub map: Map,
    pub log: GameLog,
    pub info: InfoScreen,
    pub turn_mgr: TurnManager,
}

impl World {
    pub fn new() -> Self {
        World {
            map: MapGenerator::generate_map(Config::get().MAP_WIDTH, Config::get().MAP_HEIGHT),
            log: GameLog::create(),
            info: InfoScreen::create(),
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
        self.info.draw(info_console);
    }

    fn end_turn(&mut self) -> () {
        self.turn_mgr.next_turn();
        self.log
            .push_message(String::from(format!("Turn# {}", self.turn_mgr.get_turn())));
    }

    // pub fn dispatch_key_event

    pub fn tick(&mut self, root: &mut tcod::RootConsole) -> () {
        self.end_turn();
    }

    pub fn recreate_map(&mut self) -> () {
        self.map = MapGenerator::generate_map(Config::get().MAP_WIDTH, Config::get().MAP_HEIGHT);
        self.log.push_message(String::from("MAP REBUILT!!"));
    }
}
