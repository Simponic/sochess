mod piece;
mod board;
mod context;
mod player;
mod moves;

pub use piece::*;
pub use board::*;
pub use context::*;
pub use player::*;
pub use moves::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
