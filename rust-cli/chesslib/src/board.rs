use std::ops::{Index, IndexMut, Deref};

use crate::{ColoredPiece, Move, MovementError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardSquare {
    Empty,
    Full(ColoredPiece),
}

impl TryFrom<char> for BoardSquare {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        ColoredPiece::try_from(value).map(BoardSquare::Full)
    }
}

impl From<ColoredPiece> for BoardSquare {
    fn from(value: ColoredPiece) -> Self {
        BoardSquare::Full(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    squares: [BoardSquare; 64],
}

impl Board {
    pub fn standard() -> Board {
        Board::from_board_string(
r#"
rnbqkbnr
pppppppp




PPPPPPPP
RNBQKBNR
"#
        ).unwrap()
    }

    pub fn from_board_string<S: Deref<Target = str>>(s: S) -> Result<Board, String> {
        let mut squares = [BoardSquare::Empty; 64];
        let mut count = 0;
        for ch in s.chars() {
            if ch == ' ' { count += 1; }
            if !ch.is_ascii_whitespace() {
                squares[count] = ch.try_into()?;
                count += 1;
            }
        }
        if count == 64 {
            Ok(Board { squares })
        } else {
            Err(format!("Expected 64 characters; found {count}."))
        }
    }

    pub fn blank() -> Board {
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

#[cfg(test)]
mod test {
    use crate::{Board, BoardSquare};
    use crate::ColoredPiece;
    use crate::Piece::*;
    #[test]
    fn trivial_board() {
        let stringed = Board::from_board_string(r#"
P q  R p
P q  R p
P q  R p
P q  R p
P q  R p
P q  R p
P q  R p
P q  R p
"#).unwrap();
        let wp = ColoredPiece::white(Pawn).into();
        let wr = ColoredPiece::white(Rook).into();
        let bp = ColoredPiece::black(Pawn).into();
        let bq = ColoredPiece::black(Queen).into();
        let mut arr = [BoardSquare::Empty; 64];

        for i in 0..=7 {
            arr[i*8] = wp;
            arr[i*8+2] = bq;
            arr[i*8+5] = wr;
            arr[i*8+7] = bp;
        }

        assert_eq!(stringed, Board { squares: arr });
    }
}
