use tcod::console::Console;


pub struct GameLog {
    messages: Vec<String>,
}

impl GameLog {
    pub fn create() -> Self {
        GameLog { messages: vec![] }
    }

    pub fn draw(&self, log_console: &mut tcod::console::Offscreen) -> () {
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

    pub fn push_message(&mut self, message: String) -> () {
        assert!(message.len() > 0);
        self.messages.push(message);
    }
}