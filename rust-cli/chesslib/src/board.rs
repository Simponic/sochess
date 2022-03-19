use std::ops::Index;

use crate::ColoredPiece;

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
}

impl Index<(usize, usize)> for Board {
    type Output = BoardSquare;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.get(index.0, index.1).expect("Array index out of bounds")
    }
}
