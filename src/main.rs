mod ai;
mod board;
mod game;
mod pgn;
mod piece;
mod pos;
use ai::auto_play;
use game::standard_board;
use piece::Color;

fn main() {
    let pgn_moves = auto_play(standard_board(), Color::White, 5);
    println!("{}", pgn_moves);
}
