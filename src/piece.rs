use crate::board::Board;
use crate::pos::{Pos, DIAGS, LINES, LOS};
use itertools::iproduct;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

pub enum Action {
    Go(Pos),
    Take(Pos),
    Promotion(Piece),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PawnStatus {
    CanLeap,
    JustLeaped,
    CannotLeap,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Pawn {
        orientation: Pos,
        status: PawnStatus,
    },
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

fn pawn_moves(
    board: &Board,
    pos: Pos,
    color: Color,
    orientation: Pos,
    status: PawnStatus,
) -> Vec<Vec<Action>> {
    let mut res = Vec::new();
    // Non-Taking moves
    let forward_pos = orientation + pos;
    let leap_pos = orientation * 2 + pos;
    let leap = board.get(leap_pos);
    // if there is a free cell forward
    if let Some(None) = board.get(forward_pos) {
        res.push(vec![Action::Go(forward_pos)]);
        // if we reached the backrank
        if leap.is_none() {
            res.push(vec![Action::Promotion(Piece::Knight)]);
            res.push(vec![Action::Promotion(Piece::Queen)]);
        }
        // if we can leap
        if matches!(status, PawnStatus::CanLeap) {
            // and the square is available
            if let Some(None) = leap {
                res.push(vec![Action::Go(leap_pos)]);
            }
        }
    }
    // Taking moves
    for diag_pos in orientation.neighbors() {
        let diag = board.get(diag_pos);
        // if it's a square
        if let Some(square) = diag {
            // if there's a piece on a taking square
            if let Some((other_color, _)) = square {
                // if it's an opponent
                if color != *other_color {
                    res.push(vec![Action::Go(diag_pos)]);
                }
            } else {
                // the square is empty
                let en_passant_pos = diag_pos + orientation * -1;
                // if there's a piece in en passant pos
                if let Some(Some((other_color, piece))) = board.get(en_passant_pos) {
                    // if it's an opponent
                    if color != *other_color {
                        // if it's a pawn
                        if let Piece::Pawn {
                            orientation: _,
                            status,
                        } = piece
                        {
                            // if it just leaped forward
                            if matches!(status, PawnStatus::JustLeaped) {
                                res.push(vec![Action::Go(diag_pos), Action::Take(en_passant_pos)])
                            }
                        }
                    }
                }
            }
        }
    }
    res
}

fn knight_moves(board: &Board, pos: Pos, color: Color) -> Vec<Vec<Action>> {
    iproduct!([-2, 2], [-1, 1])
        .flat_map(|(long, short)| [Pos(long, short) + pos, Pos(short, long) + pos])
        .filter(|take_pos| {
            if let Some(square) = board.get(*take_pos) {
                if let Some((other_color, _)) = square {
                    if color == *other_color {
                        return false;
                    }
                }
                return true;
            }
            return false;
        })
        .map(|take_pos| vec![Action::Go(take_pos)])
        .collect()
}

fn los_moves(board: &Board, pos: Pos, color: Color, dirs: &[Pos]) -> Vec<Vec<Action>> {
    let mut res = Vec::new();
    for dir in dirs {
        let mut curr_pos = pos;
        loop {
            curr_pos = curr_pos + *dir;
            let line = board.get(curr_pos);
            if let Some(square) = line {
                if let Some((other_color, _)) = square {
                    // it's a square with a piece
                    if color != *other_color {
                        // it's a square with an opponent
                        res.push(vec![Action::Go(curr_pos)]);
                    }
                    break;
                } else {
                    // it's a free square
                    res.push(vec![Action::Go(curr_pos)]);
                }
            } else {
                // it's out of the board
                break;
            }
        }
    }
    res
}

fn bishop_moves(board: &Board, pos: Pos, color: Color) -> Vec<Vec<Action>> {
    los_moves(board, pos, color, &DIAGS)
}

fn rook_moves(board: &Board, pos: Pos, color: Color) -> Vec<Vec<Action>> {
    los_moves(board, pos, color, &LINES)
}

fn queen_moves(board: &Board, pos: Pos, color: Color) -> Vec<Vec<Action>> {
    los_moves(board, pos, color, &LOS)
}

fn king_moves(board: &Board, pos: Pos, color: Color) -> Vec<Vec<Action>> {
    // NOTE: we don't do castling because in the game you place your pieces at the start of the match
    // so it's both useless and inapplicable in our case (also a HUGE pain to implement)
    LOS.iter()
        .map(|los_dir| *los_dir + pos)
        .filter(|take_pos| {
            if let Some(square) = board.get(*take_pos) {
                if let Some((other_color, _)) = square {
                    if color == *other_color {
                        return false;
                    }
                }
                return true;
            }
            return false;
        })
        .map(|take_pos| vec![Action::Go(take_pos)])
        .collect()
}

impl Piece {
    pub fn moves(self, board: &Board, pos: Pos, color: Color) -> Vec<Vec<Action>> {
        match self {
            Piece::Pawn {
                orientation,
                status,
            } => pawn_moves(board, pos, color, orientation, status),
            Piece::Knight => knight_moves(board, pos, color),
            Piece::Bishop => bishop_moves(board, pos, color),
            Piece::Rook => rook_moves(board, pos, color),
            Piece::Queen => queen_moves(board, pos, color),
            Piece::King => king_moves(board, pos, color),
        }
    }
}
