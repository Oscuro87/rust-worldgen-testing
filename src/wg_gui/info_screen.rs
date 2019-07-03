use tcod::console::Console;

pub struct InfoScreen;

impl InfoScreen {
    pub fn create() -> Self {
        InfoScreen {}
    }

    pub fn draw(&self, console: &mut tcod::console::Offscreen) -> () {
        console.print(1, 1, "INFO");
        console.horizontal_line(0, 0, console.width(), tcod::BackgroundFlag::None);
        console.vertical_line(0, 0, console.height(), tcod::BackgroundFlag::None);
        console.print(0, 0, "#");
    }
}
