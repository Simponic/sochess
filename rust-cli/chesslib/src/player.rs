#[derive(Copy, Clone)]
pub enum Color {
    White,
    Black,
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}
