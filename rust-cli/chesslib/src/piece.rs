use crate::Color;

#[derive(Clone, Copy)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub struct ColoredPiece {
    color: Color,
    piece: Piece,
}
