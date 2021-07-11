mod board;
mod game;
mod piece;
mod pos;
use game::Game;
use itertools::Itertools;
use piece::{Action, Color, Piece};
use pos::Pos;
use rand::Rng;
use std::{collections::HashMap, str};

fn random_move(all_moves: &HashMap<Pos, Vec<Vec<Action>>>) -> (Pos, &Vec<Action>) {
    let pieces: Vec<&Pos> = all_moves.keys().collect();
    let pos = pieces[rand::thread_rng().gen_range(0..pieces.len())];
    let moves = &all_moves[pos];
    (*pos, &moves[rand::thread_rng().gen_range(0..moves.len())])
}

fn piece2pgn(piece: Piece) -> &'static str {
    match piece {
        Piece::Pawn {
            orientation: _,
            status: _,
        } => "p",
        Piece::Knight => "N",
        Piece::Bishop => "B",
        Piece::Rook => "R",
        Piece::Queen => "Q",
        Piece::King => "K",
    }
}

fn pos2pgn(pos: Pos) -> String {
    let letters = ["a", "b", "c", "d", "e", "f", "g", "h"];
    format!("{}{}", letters[pos.0 as usize], 8 - pos.1)
}

fn move2pgn(pos: Pos, actions: &Vec<Action>) -> String {
    let mut res = String::new();
    for action in actions {
        if let Action::Go(go_pos) = action {
            res += format!("{}{}", pos2pgn(pos), pos2pgn(*go_pos)).as_str();
        } else if let Action::Promotion(piece) = action {
            res += format!("={}", piece2pgn(*piece)).as_str();
        }
    }
    res
}

fn main() {
    let mut game = Game::new();
    let mut pgn_moves: Vec<String> = Vec::new();
    let mut turn = 0;
    loop {
        let moves = game.board.moves(game.turn, true);
        if moves.len() == 0 {
            break;
        }

        let (pos, actions) = random_move(&moves);
        let pgn_move = move2pgn(pos, actions);
        pgn_moves.push(pgn_move);
        game.board = game.board.play(game.turn, pos, actions);
        game.turn = match game.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        turn += 1;
        if turn >= 150 {
            break;
        }
    }
    println!("{}", pgn_moves.iter().join(" "));
    println!("\nNo more valid moves");
}
