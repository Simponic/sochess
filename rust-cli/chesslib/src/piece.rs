use crate::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColoredPiece {
    color: Color,
    piece: Piece,
}

impl ColoredPiece {
    pub fn black(piece: Piece) -> ColoredPiece {
        ColoredPiece {
            color: Color::Black,
            piece,
        }
    }

    pub fn white(piece: Piece) -> ColoredPiece {
        ColoredPiece {
            color: Color::White,
            piece,
        }
    }
}

impl TryFrom<char> for ColoredPiece {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(ColoredPiece {
            color: {
                if value.is_ascii_uppercase() {
                    Color::White
                } else if value.is_ascii_lowercase() {
                    Color::Black
                } else {
                    return Err(format!("'{value}' is not an alphabetic character"));
                }
            },
            piece: match value.to_ascii_lowercase() {
                'p' => Piece::Pawn,
                'n' => Piece::Knight,
                'b' => Piece::Bishop,
                'r' => Piece::Rook,
                'q' => Piece::Queen,
                'k' => Piece::King,
                _ => return Err(format!("'value' is not a recognized letter")),
            },
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{ColoredPiece, Piece::*};

    #[test]
    fn white_pieces() {
        assert_eq!(
            ColoredPiece::try_from('P').unwrap(),
            ColoredPiece::white(Pawn)
        );
        assert_eq!(
            ColoredPiece::try_from('N').unwrap(),
            ColoredPiece::white(Knight)
        );
        assert_eq!(
            ColoredPiece::try_from('B').unwrap(),
            ColoredPiece::white(Bishop)
        );
        assert_eq!(
            ColoredPiece::try_from('R').unwrap(),
            ColoredPiece::white(Rook)
        );
        assert_eq!(
            ColoredPiece::try_from('Q').unwrap(),
            ColoredPiece::white(Queen)
        );
        assert_eq!(
            ColoredPiece::try_from('K').unwrap(),
            ColoredPiece::white(King)
        );
    }

    #[test]
    fn black_pieces() {
        assert_eq!(
            ColoredPiece::try_from('p').unwrap(),
            ColoredPiece::black(Pawn)
        );
        assert_eq!(
            ColoredPiece::try_from('n').unwrap(),
            ColoredPiece::black(Knight)
        );
        assert_eq!(
            ColoredPiece::try_from('b').unwrap(),
            ColoredPiece::black(Bishop)
        );
        assert_eq!(
            ColoredPiece::try_from('r').unwrap(),
            ColoredPiece::black(Rook)
        );
        assert_eq!(
            ColoredPiece::try_from('q').unwrap(),
            ColoredPiece::black(Queen)
        );
        assert_eq!(
            ColoredPiece::try_from('k').unwrap(),
            ColoredPiece::black(King)
        );
    }

    #[test]
    fn not_letters() {
        assert!(ColoredPiece::try_from('!').is_err());
        assert!(ColoredPiece::try_from('4').is_err());
        assert!(ColoredPiece::try_from('🦆').is_err());
        assert!(ColoredPiece::try_from('鴨').is_err());
    }

    #[test]
    fn bad_letters() {
        assert!(ColoredPiece::try_from('s').is_err());
        assert!(ColoredPiece::try_from('w').is_err());
        assert!(ColoredPiece::try_from('a').is_err());
        assert!(ColoredPiece::try_from('g').is_err());
    }
}
