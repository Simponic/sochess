use crate::{Board, Color, Move};

pub struct Context {
    board: Board,
    turn: Color,
}

impl Context {
    pub fn new() -> Context {
        return Context {
            board: Board::new(),
            turn: Color::default(),
        }
    }

    pub fn execute(&mut self, m: Move) -> Result<Color, String> {
        self.board.execute(m)?;
        Ok(self.turn)
    }
    
    pub fn execute_and_cycle(&mut self, m: Move) -> Result<Color, String> {
        self.execute(m)?;
        self.turn = !self.turn;
        Ok(self.turn)
    }
    
    pub fn board(&self) -> &Board {
        &self.board
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
