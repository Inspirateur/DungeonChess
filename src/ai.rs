use crate::board::Board;
use crate::pgn::move2pgn;
use crate::piece::{Action, Color, Piece};
use crate::pos::Pos;
use itertools::Itertools;
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

fn axis_value(x: i32, len: usize) -> f32 {
    // score a single axis of a position, gives more value to center
    0.5 - f32::abs(x as f32 / (len - 1) as f32 - 0.5)
}

fn pos_value(board: &Board, pos: Pos) -> f32 {
    // score a position, gives more value to center
    axis_value(pos.0, board.width) * axis_value(pos.1, board.height)
}

fn move_value(board: &Board, pos: Pos, actions: &Vec<Action>) -> f32 {
    // compute the material value of a move, *assuming that if the moves win any material the piece is lost*
    let (color, piece) = board.get(pos).unwrap().unwrap();
    let mut value = 0.;
    for action in actions {
        match *action {
            Action::Go(go_pos) => {
                if let Some(Some((o_color, o_piece))) = board.get(go_pos) {
                    value += piece_value(*o_piece) * if *o_color == color { -1. } else { 1. };
                }
            }
            Action::Take(take_pos) => {
                if let Some(Some((o_color, o_piece))) = board.get(take_pos) {
                    value += piece_value(*o_piece) * if *o_color == color { -1. } else { 1. };
                }
            }
            Action::Promotion(n_piece) => {
                value += piece_value(n_piece);
            }
        }
    }
    if value > 0. {
        value -= piece_value(piece);
    }
    value
}

fn mat_pos_score(board: &Board, player: Color) -> f32 {
    board
        .squares
        .iter()
        .enumerate()
        .map(|(i, square)| {
            if let Some((color, piece)) = square {
                (piece_value(*piece) + pos_value(board, board.pos(i)))
                    * if *color == player { 1. } else { -1. }
            } else {
                0.
            }
        })
        .fold(0., |a, b| a + b)
}

fn _negamax(board: &Board, depth: u32, mut alpha: f32, beta: f32, color: Color) -> f32 {
    if depth == 0 {
        return mat_pos_score(board, color);
    } else {
        // get all possible moves, including potentially illegal ones (they won't be played but still help evaluating position)
        let mut all_moves = board.moves(color, false);
        // for the last depth we don't consider moves with value < 0, they are unsafe.
        if depth == 1 {
            all_moves = all_moves
                .into_iter()
                .filter(|(pos, actions)| move_value(board, *pos, actions) >= 0.)
                .collect();
        }
        // sort the moves with move_value heuristic
        all_moves.sort_by(|(pos1, actions1), (pos2, actions2)| {
            move_value(board, *pos2, actions2)
                .partial_cmp(&move_value(board, *pos1, actions1))
                .unwrap()
        });
        let mut best_score = f32::NEG_INFINITY;
        for (pos, actions) in all_moves {
            best_score = f32::max(
                best_score,
                -_negamax(
                    &board.play(color, pos, &actions),
                    depth - 1,
                    -beta,
                    -alpha,
                    color.next(),
                ),
            );
            alpha = f32::max(alpha, best_score);
            if alpha >= beta {
                break;
            }
        }
        return best_score;
    }
}

pub fn minmax(board: &Board, color: Color, depth: u32) -> Option<(Pos, Vec<Action>)> {
    let all_moves = board.moves(color, true);
    let mut best_score = f32::NEG_INFINITY;
    let mut best_move = None;
    for (pos, actions) in all_moves {
        let score = -_negamax(
            &board.play(color, pos, &actions),
            depth - 1,
            f32::NEG_INFINITY,
            f32::INFINITY,
            color.next(),
        );
        if score > best_score {
            best_move = Some((pos, actions));
            best_score = score;
        }
    }
    best_move
}

pub fn random_move(board: &Board, color: Color) -> Option<(Pos, Vec<Action>)> {
    let all_moves = board.moves(color, true);
    if all_moves.len() == 0 {
        return None;
    }
    Some(all_moves[rand::thread_rng().gen_range(0..all_moves.len())].clone())
}

pub fn auto_play(mut board: Board, starting_player: Color, depth: u32) -> String {
    let mut pgn_moves: Vec<String> = Vec::new();
    let mut player = starting_player;
    let mut turn = 0;
    loop {
        let move_opt = minmax(&board, player, depth);
        if move_opt.is_none() {
            println!("\nNo more valid moves");
            break;
        }
        let (pos, actions) = move_opt.unwrap();
        let pgn_move = move2pgn(pos, &actions);
        pgn_moves.push(pgn_move);
        board = board.play(player, pos, &actions);
        player = player.next();
        turn += 1;
        if turn >= 100 {
            println!("\nGame too long");
            break;
        }
    }
    pgn_moves.iter().join(" ")
}

mod tests {
    use crate::{
        ai::auto_play,
        ai::minmax,
        board::Board,
        game::invert_color,
        game::standard_board,
        pgn::move2pgn,
        piece::{Action, Color},
        pos::Pos,
    };

    #[test]
    fn depth_3_fork() {
        let mut board = standard_board();
        board = board.play(Color::White, Pos(6, 7), &vec![Action::Go(Pos(5, 5))]);
        board = board.play(Color::Black, Pos(4, 1), &vec![Action::Go(Pos(4, 3))]);
        board = board.play(Color::White, Pos(4, 6), &vec![Action::Go(Pos(4, 5))]);
        board = board.play(Color::Black, Pos(3, 1), &vec![Action::Go(Pos(3, 3))]);
        board = board.play(Color::White, Pos(5, 7), &vec![Action::Go(Pos(3, 5))]);
        board = board.play(Color::Black, Pos(6, 0), &vec![Action::Go(Pos(5, 2))]);
        board = board.play(Color::White, Pos(7, 6), &vec![Action::Go(Pos(7, 5))]);
        println!("{}\n", board);
        let (pos, actions) = minmax(&board, Color::Black, 3).unwrap();
        board = board.play(Color::Black, pos, &actions);
        println!("{}", board);
        assert!(pos == Pos(4, 3) && actions == vec![Action::Go(Pos(4, 4))]);
    }

    #[test]
    fn color_invariant() {
        let board = standard_board();
        let pgn_moves1 = auto_play(board, Color::White, 3);
        let board = invert_color(standard_board());
        let pgn_moves2 = auto_play(board, Color::Black, 3);
        assert!(pgn_moves1 == pgn_moves2);
    }
}
