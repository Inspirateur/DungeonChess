use crate::board::Board;
use crate::piece::{Action, Color, Piece};
use crate::pos::Pos;
use rand::Rng;

fn piece_value(piece: Piece) -> f32 {
    match piece {
        Piece::Pawn {
            orientation: _,
            status: _,
        } => 1.,
        Piece::Knight => 3.,
        Piece::Bishop => 3.5,
        Piece::Rook => 5.,
        Piece::Queen => 9.,
        Piece::King => 1000.,
    }
}

fn pos_value(x: i32, len: usize) -> f32 {
    let v = 2. * x as f32 / len as f32 - 1.;
    f32::exp(-f32::powi(v, 2))
}

fn naive_score(board: &Board, player: Color) -> f32 {
    board
        .squares
        .iter()
        .enumerate()
        .map(|(i, square)| {
            if let Some((color, piece)) = square {
                let pos = board.pos(i);
                let pos_score = pos_value(pos.0, board.width) * pos_value(pos.1, board.height);
                (piece_value(*piece) + pos_score) * if *color == player { 1. } else { -1. }
            } else {
                0.
            }
        })
        .fold(0., |a, b| a + b)
}

fn _negamax(board: &Board, color: Color, depth: u32) -> f32 {
    if depth == 0 {
        return -naive_score(board, color);
    } else {
        let all_moves = board.moves(color, false);
        let mut best_score = f32::NEG_INFINITY;
        for (pos, moves) in all_moves {
            for actions in moves {
                let score = _negamax(&board.play(color, pos, &actions), color.next(), depth - 1);
                if score > best_score {
                    best_score = score;
                }
            }
        }
        return -best_score;
    }
}

pub fn minmax(board: &Board, color: Color) -> Option<(Pos, Vec<Action>)> {
    let depth = 3;
    let all_moves = board.moves(color, true);
    let mut best_score = f32::NEG_INFINITY;
    let mut best_move = None;
    for (pos, moves) in all_moves {
        for actions in moves {
            let score = _negamax(&board.play(color, pos, &actions), color.next(), depth - 1);
            if score > best_score {
                best_move = Some((pos, actions));
                best_score = score;
            }
        }
    }
    best_move
}

pub fn random_move(board: &Board, color: Color) -> Option<(Pos, Vec<Action>)> {
    let all_moves = board.moves(color, true);
    if all_moves.len() == 0 {
        return None;
    }
    let pieces: Vec<&Pos> = all_moves.keys().collect();
    let pos = pieces[rand::thread_rng().gen_range(0..pieces.len())];
    let moves = &all_moves[pos];
    Some((
        *pos,
        moves[rand::thread_rng().gen_range(0..moves.len())].clone(),
    ))
}
