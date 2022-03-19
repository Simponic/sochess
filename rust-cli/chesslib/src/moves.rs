pub struct Move {
    from: (usize, usize),
    to: (usize, usize),
}

impl Move {
    pub fn with_coords(from: (usize, usize), to: (usize, usize)) -> Result<Move, MovementError> {
        let (x1, y1) = from;
        let (x2, y2) = to;

        if x1 > 7 || y1 > 7  {
            Err(MovementError::FromOutOfBounds)
        } else if x2 > 7 || y2 > 7  {
            Err(MovementError::ToOutOfBounds)
        } else {
            Ok(Move {
                from,
                to,
            })
        }
    }
}

pub enum MovementError {
    FromOutOfBounds,
    ToOutOfBounds,
}
