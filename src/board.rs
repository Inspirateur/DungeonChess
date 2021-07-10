use crate::piece::{Color, Piece};
use crate::pos::Pos;

type Square = Option<(Color, Piece)>;

pub struct Board {
    width: usize,
    height: usize,
    squares: Vec<Square>,
}

impl Board {
    fn in_bound(&self, pos: Pos) -> bool {
        0 <= pos.0 && pos.0 < self.width as i32 && 0 <= pos.1 && pos.1 < self.height as i32
    }

    pub fn get(&self, pos: Pos) -> Option<&Square> {
        if !self.in_bound(pos) {
            return None;
        }
        Some(&self.squares[(pos.0 + pos.1 * self.width as i32) as usize])
    }
}
