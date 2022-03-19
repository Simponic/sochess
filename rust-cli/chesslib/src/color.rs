use std::ops::Not;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        if self == Color::White {
            Color::Black
        } else {
            Color::White
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}
