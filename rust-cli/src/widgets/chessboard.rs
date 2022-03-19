use chesslib::{Color as ChessColor, Context as ChesslibContext, Move};
use tui::{
    style::{Color, Style},
    widgets::StatefulWidget,
};

pub struct Context {
    inner: ChesslibContext,
    hovered: (u16, u16),
    selected: Option<(u16, u16)>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            inner: ChesslibContext::new(),
            hovered: (0, 0),
            selected: None,
        }
    }

    pub fn hover_left(&mut self) {
        self.hovered.1 += 7;
        self.hovered.1 %= 8;
    }

    pub fn hover_right(&mut self) {
        self.hovered.1 += 1;
        self.hovered.1 %= 8;
    }

    pub fn hover_up(&mut self) {
        self.hovered.0 += 7;
        self.hovered.0 %= 8;
    }

    pub fn hover_down(&mut self) {
        self.hovered.0 += 1;
        self.hovered.0 %= 8;
    }

    pub fn inner_mut(&mut self) -> &mut ChesslibContext {
        &mut self.inner
    }

    pub fn click(&mut self) -> Option<Move> {
        if let Some((x1, y1)) = self.selected {
            let (x2, y2) = self.hovered;
            if self.inner.board()[(x2 as usize, y2 as usize)].is_empty() {
                self.selected = None;
                // gross casting garbage
                Some(
                    Move::with_coords((x1 as usize, y1 as usize), (x2 as usize, y2 as usize))
                        .unwrap(),
                )
            } else {
                None
            }
        } else {
            let (x, y) = self.hovered;
            if self.inner.board()[(x as usize, y as usize)].is_full() {
                self.selected = Some(self.hovered);
            }
            None
        }
    }
}

pub struct ChessboardWidget;

// don't wanna get lost with magic numbers
const CELL_WIDTH: u16 = 3;
const CELL_HEIGHT: u16 = 1;
const NUM_CELLS: u16 = 8;

impl StatefulWidget for ChessboardWidget {
    type State = Context;

    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let reverse = state.inner.turn() == ChessColor::Black;
        let startx = area.width / 2 - CELL_WIDTH * NUM_CELLS / 2;
        let starty = area.height / 2 - CELL_HEIGHT * NUM_CELLS / 2;

        let mut row = 0;
        let mut column = 0;

        let mut sqcolor = ChessColor::White;

        for sq in state.inner.board().squares().iter() {
            let (bg, fg) = {
                if (row, column) == state.hovered {
                    (Color::Yellow, Color::Black)
                } else if state.selected.is_some() && (row, column) == state.selected.unwrap() {
                    (Color::Red, Color::Black)
                } else if sqcolor == ChessColor::White {
                    (Color::White, Color::Black)
                } else {
                    (Color::Black, Color::White)
                }
            };
            let style = Style::default().fg(fg).bg(bg);

            if reverse {
                buf.set_string(
                    startx + CELL_WIDTH * (NUM_CELLS - column - 1),
                    starty + CELL_HEIGHT * (NUM_CELLS - row - 1),
                    format!("{sq}"),
                    style,
                );
            } else {
                buf.set_string(
                    startx + CELL_WIDTH * column,
                    starty + CELL_HEIGHT * row,
                    format!("{sq}"),
                    style,
                );
            }

            column += 1;

            if column > 7 {
                println!();
                column = 0;
                row += 1;
            } else {
                sqcolor = !sqcolor;
            }
        }
    }
}
