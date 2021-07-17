use crate::board::Board;
use crate::piece::{Color, PawnStatus, Piece};
use crate::pos::Pos;

pub fn standard_board() -> Board {
    let mut board = Board::new(8, 8);
    let pieces = [
        Piece::Rook,
        Piece::Knight,
        Piece::Bishop,
        Piece::Queen,
        Piece::King,
        Piece::Bishop,
        Piece::Knight,
        Piece::Rook,
    ];
    for (i, piece) in pieces
        .iter()
        .chain(
            [Piece::Pawn {
                orientation: Pos(0, 1),
                status: PawnStatus::CanLeap,
            }; 8]
                .iter(),
        )
        .enumerate()
    {
        board.squares[i] = Some((Color::Black, *piece));
    }
    let len_squares = board.squares.len();
    for (i, piece) in pieces
        .iter()
        .rev()
        .chain(
            [Piece::Pawn {
                orientation: Pos(0, -1),
                status: PawnStatus::CanLeap,
            }; 8]
                .iter(),
        )
        .enumerate()
    {
        board.squares[len_squares - i - 1] = Some((Color::White, *piece));
    }
    board
}

pub fn invert_color(board: Board) -> Board {
    let mut inverted = Board::new(board.width, board.height);
    inverted.squares = board
        .squares
        .iter()
        .map(|square| {
            if let Some((color, piece)) = square {
                Some((color.next(), *piece))
            } else {
                None
            }
        })
        .collect();
    inverted
}
