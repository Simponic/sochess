use chesslib::{Board, Color};

extern crate chesslib;

fn main() {
    let mut b = Board::new();

    let mut column = 0;
    let mut col = Color::White;
    for sq in b.squares().iter() {
        if col == Color::Black {
            print!("\x1B[0;37;40m")
        } else {
            print!("\x1B[0;47;30m")
        }

        print!("{}\x1B[0m", sq);
        column += 1;
        if column > 7 {
            println!();
            column = 0;
        } else {
            col = !col;
        }
    }
}
