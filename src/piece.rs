use crate::board::Board;
use crate::pos::Pos;
use itertools::iproduct;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

enum Action {
    Go(Pos),
    Take(Pos),
    Promotion(Piece),
}

#[derive(Clone, Copy)]
enum PawnStatus {
    CanLeap,
    JustLeaped,
    CannotLeap,
}
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

fn bishop_moves(board: &Board, pos: Pos, color: Color) -> Vec<Vec<Action>> {
    let mut res = Vec::new();
    for (x, y) in iproduct!([-1, 1], [-1, 1]) {
        let diag_dir = Pos(x, y);
        let len = 1;
        loop {
            let diag_pos = pos + diag_dir * len;
            let diag = board.get(diag_pos);
            if let Some(square) = diag {
                if let Some((other_color, _)) = square {
                    // it's a square with a piece
                    if color != *other_color {
                        // it's a square with an opponent
                        res.push(vec![Action::Go(diag_pos)]);
                    }
                    break;
                } else {
                    // it's a free square
                    res.push(vec![Action::Go(diag_pos)]);
                }
            } else {
                // it's out of the board
                break;
            }
        }
    }
    res
}

fn rook_moves(board: &Board, pos: Pos, color: Color) -> Vec<Vec<Action>> {
    let mut res = Vec::new();
    for dir in [-1, 1] {
        for line_dir in [Pos(0, dir), Pos(dir, 0)] {
            let len = 1;
            loop {
                let line_pos = pos + line_dir * len;
                let line = board.get(line_pos);
                if let Some(square) = line {
                    if let Some((other_color, _)) = square {
                        // it's a square with a piece
                        if color != *other_color {
                            // it's a square with an opponent
                            res.push(vec![Action::Go(line_pos)]);
                        }
                        break;
                    } else {
                        // it's a free square
                        res.push(vec![Action::Go(line_pos)]);
                    }
                } else {
                    // it's out of the board
                    break;
                }
            }
        }
    }
    res
}

fn queen_moves(board: &Board, pos: Pos, color: Color) -> Vec<Vec<Action>> {
    let mut res = bishop_moves(board, pos, color);
    res.extend(rook_moves(board, pos, color));
    res
}

fn king_moves(board: &Board, pos: Pos, color: Color) -> Vec<Vec<Action>> {
    // NOTE: we don't do castling because in the game you place your pieces at the start of the match
    // so it's both useless and inapplicable in our case (also a HUGE pain to implement)
    iproduct!(-1..=1, -1..=1)
        .filter(|(x, y)| *x != 0 || *y != 0)
        .map(|(x, y)| Pos(x, y) + pos)
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
    fn moves(self, board: &Board, pos: Pos, color: Color) -> Vec<Vec<Action>> {
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
            _ => Vec::new(),
        }
    }
}
