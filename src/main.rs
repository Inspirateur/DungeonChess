mod board;
mod game;
mod pgn;
use pgn::move2pgn;
mod piece;
mod pos;
use game::Game;
use itertools::Itertools;
mod ai;
use ai::{minmax, random_move};

fn main() {
    let mut game = Game::new();
    let mut pgn_moves: Vec<String> = Vec::new();
    let mut turn = 0;
    loop {
        let move_opt = minmax(&game.board, game.turn);
        if move_opt.is_none() {
            println!("\nNo more valid moves");
            break;
        }
        let (pos, actions) = move_opt.unwrap();
        let pgn_move = move2pgn(pos, &actions);
        pgn_moves.push(pgn_move);
        game.board = game.board.play(game.turn, pos, &actions);
        game.turn = game.turn.next();
        turn += 1;
        if turn >= 100 {
            println!("\nGame too long");
            break;
        }
    }
    println!("{}", pgn_moves.iter().join(" "));
}
