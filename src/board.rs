use crate::piece::{Action, Color, Piece};
use crate::pos::{Pos, LOS};
use std::collections::HashMap;

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

    pub fn moves(&self, color: Color) -> HashMap<Pos, Vec<Vec<Action>>> {
        // generate all moves for color
        let mut res = HashMap::new();
        let mut king_pos = None;
        for (i, square) in self.squares.iter().enumerate() {
            if let Some((piece_color, piece)) = square {
                if *piece_color == color {
                    let pos = Pos((i % self.width) as i32, (i / self.height) as i32);
                    res.insert(pos, piece.moves(self, pos, color));
                    if *piece == Piece::King {
                        king_pos = Some(pos);
                    }
                }
            }
        }
        // if this panics it means there's no king with this color on the board lol
        let king_pos = king_pos.unwrap();
        // scan all los from the king's pos to see which pieces are potentially blocking an attack on the king
        // ENNEMY PIECES COUNT TOO -> cause in en passant case you can take a piece without replacing it with your own on the same square
        // After that, generate board from moves involving these position
        //  for each of these boards, check the concerned los again and if there's an opponent that can take the king on it, cancel the move

        // additionnaly, check all the king moves and make sure the opponent has no Go or Take move on this position
        // (this time we can't use los tricks cause we have to watch out for knights)
        res
    }

    pub fn play(&self, color: Color, pos: Pos, actions: Vec<Action>) {
        // TODO: resolve the move into a new board, send the appropriate event to update pieces data
    }
}
