#[derive(Copy, Clone)]
pub struct Move {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Move {
    pub fn with_coords(before: (usize, usize), after: (usize, usize)) -> Result<Move, String> {
        let (x1, y1) = before;
        let (x2, y2) = after;

        if x1 > 7 || y1 > 7 {
            Err(format!(
                "before position ({x1}, {y1}) out of range of board."
            ))
        } else if x2 > 7 || y2 > 7 {
            Err(format!(
                "after position ({x1}, {y1}) out of range of board."
            ))
        } else {
            Ok(Move { x1, y1, x2, y2 })
        }
    }

    pub fn before(&self) -> (usize, usize) {
        (self.x1, self.y1)
    }

    pub fn after(&self) -> (usize, usize) {
        (self.x2, self.y2)
    }
}

impl Into<(usize, usize, usize, usize)> for Move {
    fn into(self) -> (usize, usize, usize, usize) {
        (self.x1, self.y1, self.x2, self.y2)
    }
}
