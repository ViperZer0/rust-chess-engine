//! Contains functions for queries on the current state of a board.
//!
//! This module allows one to query the board for information on what pieces are where and such.
//! These functions operate on [Board](super::Board)s but for the sake of space are isolated into
//! their own sub-module.

use crate::{bitboard::{Bitboard, OutOfBoundsError}, board::{PieceType, PlayerColor, Square}};

use super::Board;

pub struct BoardQuery<'a, T>
{
    // A reference to the board being queried.
    board: &'a Board,
    // The result that this query will return.
    result: T
}

impl<'a, T> BoardQuery<'a, T>
{
    pub fn result(self) -> T
    {
        self.result
    }
}

impl<'a> BoardQuery<'a, Bitboard>
{
    /// Filters a [Bitboard] to only pieces of the specified color.
    ///
    /// # Arguments
    ///
    /// * `color` - The player color to filter by.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{Square, PlayerColor, Board};
    /// // Make a new "standard" board.
    /// let board = Board::new_default_starting_board();
    /// // This should return true. There IS a white piece at 0, 0 (the white rook)
    /// assert!(board.query().color(PlayerColor::White).piece_at(Square::new(0,0)).unwrap().result());
    /// // This should return FALSE. There is not a black piece at 0, 0.
    /// assert!(!board.query().color(PlayerColor::Black).piece_at(Square::new(0,0)).unwrap().result());
    /// ```
    pub fn color(self, color: PlayerColor) -> Self
    {
        let piece_color_mask = match color
        {
            PlayerColor::White => self.board.white_pieces,
            PlayerColor::Black => self.board.black_pieces,
        };
        Self
        {
            result: self.result & piece_color_mask,
            ..self
        }
    }

    /// Filters a [Bitboard] to only pieces of the specified type.
    /// The pieces can be of any color. If you only want pieces of a certain color too, chain this
    /// operation with [BoardQuery::color()].
    ///
    ///
    /// # Arguments
    ///
    /// * `piece_type` - The type of the piece to filter by.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{Square, PieceType, Board};
    /// let board = Board::new_default_starting_board();
    /// // This should return true since there is a rook at 0, 0.
    /// assert!(board.query().piece_type(PieceType::Rook).piece_at(Square::new(0,0)).unwrap().result());
    /// // There should also be a rook at 7, 7.
    /// assert!(board.query().piece_type(PieceType::Rook).piece_at(Square::new(7,7)).unwrap().result());
    /// // There should not be a knight at 0, 0.
    /// assert!(!board.query().piece_type(PieceType::Knight).piece_at(Square::new(0,0)).unwrap().result());
    /// ```
    pub fn piece_type(self, piece_type: PieceType) -> Self
    {
        let piece_type_mask = match piece_type
        {
            PieceType::King => self.board.king_pieces,
            PieceType::Queen => self.board.queen_pieces,
            PieceType::Rook => self.board.rook_pieces,
            PieceType::Bishop => self.board.bishop_pieces,
            PieceType::Knight => self.board.knight_pieces,
            PieceType::Pawn => self.board.pawn_pieces,
        };
        Self
        {
            result: self.result & piece_type_mask,
            ..self
        }
    }

    /// Takes a [BoardQuery] and converts it from a [Bitboard] to a [bool],
    /// with that `bool` set to true if there was a piece at that location (i.e a bit set to 1 at
    /// the specified coordinates), and false if there was not a piece at that location (i.e a bit
    /// set to 0).
    ///
    /// This function returns a [Result], with the [Err] variant being an [OutOfBoundsError] if 
    /// a [Square] with invalid coordinates was passed in (i.e rank or file being >= 8).
    ///
    /// If you know that [Square] is within the bounds specified, you can use
    /// [Self::piece_at_unchecked] instead.
    ///
    /// # Arguments
    ///
    /// * `at` - The square to check for the existence of a piece at.
    ///
    /// # Errors
    ///
    /// An [OutOfBoundsError] is returned if a [Square] was passed in with invalid coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{Square, PieceType, PlayerColor, Board};
    /// let board = Board::new_default_starting_board();
    /// // Yes, there should be a piece (any piece) at 0,0.
    /// assert!(board.query().piece_at(Square::new(0,0)).unwrap().result());
    /// // Yes, there should be a WHITE piece at 0,0.
    /// assert!(board.query().color(PlayerColor::White).piece_at(Square::new(0,0)).unwrap().result());
    /// assert!(board.query().color(PlayerColor::White).piece_type(PieceType::Rook).piece_at(Square::new(0,0)).unwrap().result());
    ///
    /// ```
    pub fn piece_at(self, at: Square) -> Result<BoardQuery<'a, bool>, OutOfBoundsError>
    {
        let bit_index = Bitboard::coords_to_index(at)?;
        Ok(BoardQuery{
            result: self.result.is_bit_set(bit_index),
            board: self.board,
        })
    }

    /// Same as [Self::piece_at] but unchecked. Use this version if you know that your passed in
    /// [Square] is within bounds and don't want to check for errors.
    ///
    /// # Panics
    ///
    /// This function panics if [Square] has a rank or file greater than 7.
    pub fn piece_at_unchecked(self, at: Square) -> BoardQuery<'a, bool>
    {
        self.piece_at(at).expect("Provided square coordinates were out of bounds!")
    }
}

impl Board
{
    /// Creates a new [BoardQuery], which can be used to filter for desired board information.
    ///
    /// By default a [BoardQuery] will basically have no filter applied, containing a [Bitboard]
    /// with all pieces have their bit set to 1.
    /// Further filtering can be done with [BoardQuery] methods.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{Square, Board};
    /// let board = Board::new_default_starting_board();
    /// // Yes, there should be a piece at 0, 0 (a rook)
    /// assert!(board.query().piece_at(Square::new(0, 0)).unwrap().result());
    /// ```
    pub fn query(&self) -> BoardQuery<'_, Bitboard>
    {
        let mask_of_all_pieces = self.white_pieces | self.black_pieces;
        BoardQuery
        {
            board: &self,
            result: mask_of_all_pieces,
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_bitboard_query_on_starting_board()
    {
        let board = Board::new_default_starting_board();
        let bitboard_result = board.query().result();
        // 0xFFFF_0000_0000_FFFF is the bit representation of the initial board layout where
        // the first two and last two ranks are completely filled.
        assert_eq!(bitboard_result, Bitboard::new(0xFFFF_0000_0000_FFFF));
    }

    #[test]
    fn test_bitboard_query_on_empty_board()
    {
        let board = Board::new_blank_board();
        let bitboard_result = board.query().result();
        assert_eq!(bitboard_result, Bitboard::new(0));
    }

    #[test]
    fn test_bitboard_query_on_white_pieces_only()
    {
        let board = Board::new_default_starting_board();
        let bitboard_result = board.query().color(PlayerColor::White).result();
        assert_eq!(bitboard_result, Bitboard::new(0x0000_0000_0000_FFFF));
    }

    #[test]
    fn test_bitboard_query_on_black_pieces_only()
    {
        let board = Board::new_default_starting_board();
        let bitboard_result = board.query().color(PlayerColor::Black).result();
        assert_eq!(bitboard_result, Bitboard::new(0xFFFF_0000_0000_0000));
    }
    #[test]
    fn test_bitboard_query_on_piece_types()
    {
        let board = Board::new_default_starting_board();
        let rook_bitboard = board.query().piece_type(PieceType::Rook).result();
        assert_eq!(rook_bitboard, Bitboard::new(0x8100_0000_0000_0081));
        let knight_bitboard = board.query().piece_type(PieceType::Knight).result();
        assert_eq!(knight_bitboard, Bitboard::new(0x4200_0000_0000_0042));
        let bishop_bitboard = board.query().piece_type(PieceType::Bishop).result();
        assert_eq!(bishop_bitboard, Bitboard::new(0x2400_0000_0000_0024));
        let king_bitboard = board.query().piece_type(PieceType::King).result();
        assert_eq!(king_bitboard, Bitboard::new(0x1000_0000_0000_0010));
        let queen_bitboard = board.query().piece_type(PieceType::Queen).result();
        assert_eq!(queen_bitboard, Bitboard::new(0x0800_0000_0000_0008));
        let pawn_bitboard = board.query().piece_type(PieceType::Pawn).result();
        assert_eq!(pawn_bitboard, Bitboard::new(0x00FF_0000_0000_FF00));
    }

    #[test]
    #[should_panic]
    fn test_bitboard_query_on_out_of_bounds_square()
    {
        let board = Board::new_blank_board();
        _ = board.query().piece_at_unchecked(Square::new(8,0));
    }

    #[test]
    #[should_panic]
    fn test_bitboard_query_on_out_of_bounds_square_2()
    {
        let board = Board::new_blank_board();
        _ = board.query().piece_at_unchecked(Square::new(0,8));
    }

    #[test]
    #[should_panic]
    fn test_bitboard_query_on_out_of_bounds_square_3()
    {
        let board = Board::new_blank_board();
        _ = board.query().piece_at_unchecked(Square::new(8,8));
    }

    #[test]
    fn test_bitboard_query_on_out_of_bounds_square_4()
    {
        let board = Board::new_blank_board();
        assert!(board.query().piece_at(Square::new(100, 100)).is_err());
    }
}

