pub struct TurnManager {
    turn_no: u32,
}

impl TurnManager {
    pub fn create() -> Self {
        TurnManager {
            turn_no: 0,
        }
    }

    pub fn next_turn(&mut self) -> () {
        self.turn_no += 1;
    }

    pub fn get_turn(&self) -> u32 {
        return self.turn_no;
    }
}