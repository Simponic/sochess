use std::{ops::{Index, IndexMut}, mem};

use crate::{ColoredPiece, Move, MovementError};

#[derive(Clone, Copy)]
pub enum BoardSquare {
    Empty,
    Full(ColoredPiece),
}

pub struct Board {
    squares: [BoardSquare; 64],
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [BoardSquare::Empty; 64],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&BoardSquare> {
        self.squares.get(x * 8 + y)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut BoardSquare> {
        self.squares.get_mut(x * 8 + y)
    }

    pub fn execute(&mut self, m: Move) -> Result<(), MovementError> {
        let from = self[m.before()];
        let to = self[m.after()];

        if let BoardSquare::Full(_) = from {
            if let BoardSquare::Full(_) = to {
                self[m.before()] = to;
                self[m.after()] = from;
                Ok(())
            } else {
                Err(MovementError::Occupied)
            }
        } else {
            Err(MovementError::PieceDoesNotExist)
        }
    }
}

impl Index<(usize, usize)> for Board {
    type Output = BoardSquare;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.get(index.0, index.1).expect("Array index out of bounds")
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1).expect("Array index out of bounds")
    }
}
