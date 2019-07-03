#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate serde_json;
use rand::prelude::*;
use std::fs;
use tcod::Console;

mod wg_core;
mod wg_gui;
mod wg_managers;
mod wg_tests;
mod wg_utils;
mod wg_world;

use wg_core::config::Config;
use wg_world::world::World;

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
        // Map screen
        map_console.print(2, 2, "Generating map...");
        // Log screen
        log_console.horizontal_line(0, 0, log_console.width(), tcod::BackgroundFlag::None);
        log_console.print(0, 1, "LOGS");
        // World
        world.draw(
            &mut map_console,
            &mut log_console,
            &mut info_console,
            &mut entities_console,
        );

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
            &entities_console,
            (0, 0),
            (0, 0),
            &mut map_console,
            (0, 0),
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

        // Key events
        let key = root.wait_for_keypress(true);

        match key.code {
            tcod::input::KeyCode::Escape => {
                running = false;
            }
            tcod::input::KeyCode::Enter | tcod::input::KeyCode::NumPadEnter => {
                world.tick(&mut root);
            }
            _ => {}
        }

        match key.printable {
            'r' => {
                world.recreate_map();
            }
            _ => {}
        }
    }
}
