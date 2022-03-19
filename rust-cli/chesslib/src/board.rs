use std::{ops::{Deref, Index, IndexMut}, fmt::{Display, Formatter, Write}};

use crate::{ColoredPiece, Move};

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

impl Display for BoardSquare {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        if let BoardSquare::Full(cp) = self {
            f.write_char(' ')?;
            f.write_char((*cp).into())?;
            f.write_char(' ')?;
        } else {
            f.write_str("   ")?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    squares: [BoardSquare; 64],
}

impl Board {
    pub fn new() -> Board {
        Board::from_board_string(
            r#"
rnbqkbnr
pppppppp
        
        
        
        
PPPPPPPP
RNBQKBNR
"#,
        )
        .unwrap()
    }

    pub fn from_board_string<S: Deref<Target = str>>(s: S) -> Result<Board, String> {
        let mut squares = [BoardSquare::Empty; 64];
        let mut count = 0;
        for ch in s.chars() {
            if ch == ' ' {
                count += 1;
            }
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

    pub fn execute(&mut self, m: Move) -> Result<(), String> {
        let before = m.before();
        let after = m.after();
        let from = self[before];
        let to = self[after];

        if let BoardSquare::Full(_) = from {
            if let BoardSquare::Full(_) = to {
                self[before] = to;
                self[after] = from;
                Ok(())
            } else {
                let (x, y) = after;
                Err(format!("After location ({x}, {y}) is occupied."))
            }
        } else {
            let (x, y) = before;
            Err(format!("Before location ({x}, {y}) is vacant."))
        }
    }

    pub fn squares(&self) -> &[BoardSquare] {
        &self.squares
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<(usize, usize)> for Board {
    type Output = BoardSquare;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self
            .get(index.0, index.1)
            .expect("Array index out of bounds")
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1)
            .expect("Array index out of bounds")
    }
}

#[cfg(test)]
mod test {
    use crate::ColoredPiece;
    use crate::Piece::*;
    use crate::{Board, BoardSquare};
    #[test]
    fn trivial_board() {
        let stringed = Board::from_board_string(
            r#"
P q  R p
P q  R p
P q  R p
P q  R p
P q  R p
P q  R p
P q  R p
P q  R p
"#,
        )
        .unwrap();
        let wp = ColoredPiece::white(Pawn).into();
        let wr = ColoredPiece::white(Rook).into();
        let bp = ColoredPiece::black(Pawn).into();
        let bq = ColoredPiece::black(Queen).into();
        let mut arr = [BoardSquare::Empty; 64];

        for i in 0..=7 {
            arr[i * 8] = wp;
            arr[i * 8 + 2] = bq;
            arr[i * 8 + 5] = wr;
            arr[i * 8 + 7] = bp;
        }

        assert_eq!(stringed, Board { squares: arr });
    }
}
