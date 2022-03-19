mod board;
mod color;
mod context;
mod moves;
mod piece;

use std::ops::Deref;

pub use board::*;
pub use color::*;
pub use context::*;
pub use moves::*;
pub use piece::*;

pub fn alnum_to_coords<S: Deref<Target = str>>(alnum: S) -> Result<(usize, usize), String> {
    let mut chs = alnum.as_bytes().iter();
    let letter = chs.next().ok_or_else(|| "Need a letter.")?.to_ascii_lowercase();
    let number = *chs.next().ok_or_else(|| "Need a number.")?;
    if chs.next() != None {
        return Err("Too many characters".into());
    }

    if !(97..=103).contains(&letter) {
        return Err(format!("'{letter}' is not between 'a' and 'g'."));
    }

    let y = letter - 97;

    if !(49..=56).contains(&number) {
        return Err(format!("'{number}' is not between '1' and '9'."));
    }

    let x = number - 49;

    Ok((x as usize, y as usize))
}
