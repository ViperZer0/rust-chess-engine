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
    /// Generates a bitboard where all valid squares that a knight on a given square can move to are set to 1.
    /// The square that the knight is currently on is not included in this set.
    ///
    /// # Arguments
    ///
    /// * `from` - The square that the knight is on.
    ///
    /// # Examples
    ///
    /// ```
    /// let knight_moves: Bitboard = Board::knight_moves(Square::new(0,0));
    /// let knight_move_squares: Vec<Square> = knight_moves.squares().collect();
    /// // This should be 2 since a knight in the corner only ever has two moves.
    /// assert_eq!(knight_move_squares.len(), 2);
    /// assert_eq!(knight_move_squares, vec![Square::new(1, 2), Square::new(2, 1)]);
    /// ```
    fn knight_moves(from: Square) -> Bitboard
    {
        // For each possible knight move, we want to mask out the areas of the board that the
        // knight can't move from.
        // I.e if the knight is on the edge of the board, we can't move further to the left.
        let knight_on_rank_1_or_higher = Bitboard::from(from) & Bitboard::rank_mask_iter(1..8);
        let knight_on_rank_2_or_higher = Bitboard::from(from) & Bitboard::rank_mask_iter(2..8);
        let knight_on_rank_6_or_lower  = Bitboard::from(from) & Bitboard::rank_mask_iter(0..7);
        let knight_on_rank_5_or_lower  = Bitboard::from(from) & Bitboard::rank_mask_iter(0..6);
        let knight_on_file_b_to_h      = Bitboard::from(from) & Bitboard::file_mask_iter(1..8);
        let knight_on_file_c_to_h      = Bitboard::from(from) & Bitboard::file_mask_iter(2..8);
        let knight_on_file_a_to_g      = Bitboard::from(from) & Bitboard::file_mask_iter(0..7);
        let knight_on_file_a_to_f      = Bitboard::from(from) & Bitboard::file_mask_iter(0..6);

        // Each bitshift is the number of squares between the source and target square
        // With a bitmask to block out "bad" areas that can't be moved from.
        let north_north_east = (knight_on_rank_5_or_lower & knight_on_file_a_to_g) << 17;
        let north_east_east = (knight_on_rank_6_or_lower & knight_on_file_a_to_f) << 10;
        let south_east_east = (knight_on_rank_1_or_higher & knight_on_file_a_to_f) >> 6;
        let south_south_east = (knight_on_rank_2_or_higher & knight_on_file_a_to_g) >> 15;
        let south_south_west = (knight_on_rank_2_or_higher & knight_on_file_b_to_h) >> 17;
        let south_west_west = (knight_on_rank_1_or_higher & knight_on_file_c_to_h) >> 10;
        let north_west_west = (knight_on_rank_6_or_lower & knight_on_file_c_to_h) << 6;
        let north_north_west = (knight_on_rank_5_or_lower & knight_on_file_b_to_h) << 15;

        Bitboard::default() 
            | north_north_east
            | north_east_east
            | south_east_east
            | south_south_east
            | south_south_west
            | south_west_west
            | north_west_west
            | north_north_west
    }

    /// Generates a bitmask of all valid squares that a king can move to.
    ///
    /// This is typically the 8 surrounding squares, unless it's on a corner or edge.
    ///
    /// # Arguments
    ///
    /// * `from` - The square the king is currently on.
    ///
    /// # Examples
    ///
    /// ```
    /// let king_moves = Board::king_moves(Square::new(0, 0));
    /// let king_move_squares: Vec<Square> = king_moves.squares().collect();
    /// assert_eq!(king_move_squares.len(), 3);
    /// assert_eq!(king_move_squares, vec![Square::new(1,0), Square::new(0,1), Square::new(1,1)]);
    /// ```
    fn king_moves(from: Square) -> Bitboard
    {
        todo!()
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

    #[test]
    fn king_moves_from_center()
    {
        let king_move_mask = Board::king_moves(Square::new(1, 1));
        let expected_bitboard = 
            Bitboard::from(Square::new(0, 0)) |
            Square::new(0, 1).into() |
            Square::new(0, 2).into() |
            Square::new(1, 2).into() |
            Square::new(2, 2).into() |
            Square::new(2, 1).into() |
            Square::new(2, 0).into() |
            Square::new(1, 0).into();
        assert_eq!(king_move_mask, expected_bitboard);
    }

    #[test]
    fn king_moves_from_corner()
    {
        let king_move_mask = Board::king_moves(Square::new(0, 0));
        let expected_bitboard =
            Bitboard::from(Square::new(0, 1)) |
            Square::new(1, 1).into() |
            Square::new(1, 0).into();
        assert_eq!(king_move_mask, expected_bitboard);
    }
}
