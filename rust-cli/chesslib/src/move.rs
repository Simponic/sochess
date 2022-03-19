pub struct Move {
    from: (usize, usize),
    to: (usize, usize),
}

impl Move {
    pub fn with_coords(from: (usize, usize), to: (usize, usize)) -> Result<Move> {
        let (x1, y1) = from;
        let (x2, y2) = to;

        if x1 > 7
    }
}
