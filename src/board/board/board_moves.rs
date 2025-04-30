//! Helper module for the [board](super) module, exposing various bit-twiddling operations
//! for different piece moves and such.
//!
//! This module mostly operates on [Bitboards](crate::Bitboard), but we generally consider
//! the exact sequence of operations needed to test different moves outside of the scope of a
//! `Bitboard`, and the [Board](super::Board) impl is long enough as is.
//!
//! Most of the operations implemented here take a [Square](super::Square) as input and output
//! a [Bitboard](crate::Bitboard) representing the valid moves that piece can make.
//!
//! All operations here are implemented as associated functions on a [Board](super::Board), unless
//! they require knowledge of the current board state (i.e for occupancy checks), in which case
//! they will take a `&Board` instead.

use crate::{bitboard::Bitboard, board::Square};

use super::Board;

impl Board {
    fn knight_moves(from: Square) -> Bitboard
    {
        todo!();
    }
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn knight_moves_from_center()
    {
        let knight_move_mask = Board::knight_moves(Square::new(3, 3));
        let expected_bitboard: Bitboard = 
            Bitboard::from(Square::new(1, 2)) |
            Bitboard::from(Square::new(1, 4)) |
            Bitboard::from(Square::new(2, 1)) |
            Bitboard::from(Square::new(2, 5)) |
            Bitboard::from(Square::new(4, 1)) |
            Bitboard::from(Square::new(4, 5)) |
            Bitboard::from(Square::new(5, 2)) |
            Bitboard::from(Square::new(5, 4));
        assert_eq!(knight_move_mask, expected_bitboard);
    }

    #[test]
    fn knight_moves_from_corner_1()
    {
        let knight_move_mask = Board::knight_moves(Square::new(0, 0));
        let expected_bitboard: Bitboard = 
            Bitboard::from(Square::new(1, 2)) |
            Square::new(2, 1).into();
        assert_eq!(knight_move_mask, expected_bitboard);
    }

    #[test]
    fn knight_moves_from_corner_2()
    {
        let knight_move_mask = Board::knight_moves(Square::new(7, 7));
        let expected_bitboard =
            Bitboard::from(Square::new(6, 5)) |
            Square::new(5, 6).into();
        assert_eq!(knight_move_mask, expected_bitboard);
    }

    #[test]
    fn knight_moves_from_side()
    {
        let knight_move_mask = Board::knight_moves(Square::new(3, 7));
        let expected_bitboard =
            Bitboard::from(Square::new(1, 6)) |
            Square::new(2, 5).into() |
            Square::new(4, 5).into() |
            Square::new(5, 6).into();
        assert_eq!(knight_move_mask, expected_bitboard);
    }
}
