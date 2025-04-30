//! Contains functions for queries on the current state of a board.
//!
//! This module allows one to query the board for information on what pieces are where and such.
//! These functions operate on [Board](super::Board)s but for the sake of space are isolated into
//! their own sub-module.

use crate::{bitboard::Bitboard, board::{PieceType, PlayerColor}};

use super::Board;

impl Board
{
    /// Gets all the pieces of a specific color on one bitboard,
    /// where a 1 means a piece of the desired color is on that square,
    /// and 0 means a piece of the desired color is not on it.
    ///
    /// # Arguments
    ///
    /// * `color` - The [PlayerColor] to filter for.
    ///
    /// # Examples
    ///
    /// ```
    /// let board = Board::default();
    /// let white_pieces = board.color_pieces(PlayerColor::White);
    /// // There should be 16 white pieces.
    /// assert!(white_pieces.squares().len(), 16);
    /// ```
    pub fn color_pieces(&self, color: PlayerColor) -> Bitboard
    {
        match color
        {
            PlayerColor::White => self.white_pieces,
            PlayerColor::Black => self.black_pieces,
        }
    }

    /// Gets all the pieces of a specific type on one bitboard.
    ///
    /// Note that this gets BOTH white and black pieces. If you want pieces of a specific type
    /// AND color use [Self::color_type_pieces].
    ///
    /// # Arguments
    ///
    /// * `piece_type` - The [PieceType] to filter for.
    ///
    /// # Examples
    ///
    /// ```
    /// let board = Board::default();
    /// let knights = board.type_pieces(PieceType::Knight);
    /// // There should be 4 knights.
    /// assert!(knights.squares().len(), 4);
    /// ```
    pub fn type_pieces(&self, piece_type: PieceType) -> Bitboard
    {
        match piece_type
        {
            PieceType::Pawn => self.pawn_pieces,
            PieceType::Bishop => self.bishop_pieces,
            PieceType::Knight => self.knight_pieces,
            PieceType::Rook => self.rook_pieces,
            PieceType::Queen => self.queen_pieces,
            PieceType::King => self.king_pieces,
        }
    }

    /// Gets all the pieces of a specific type AND color on one bitboard.
    ///
    /// # Arguments
    ///
    /// * `color` - The player color to filter by.
    /// * `piece_type` - The piece type to filter by.
    ///
    /// # Examples
    ///
    /// ```
    /// let board = Board::default();
    /// let white_rooks = board.color_type_pieces(PlayerColor::White, PieceType::Rook);
    /// // there should be 2 white rooks
    /// assert!(white_rooks.squares().len(), 2);
    /// ```
    pub fn color_type_pieces(&self, color: PlayerColor, piece_type: PieceType) -> Bitboard
    {
        self.color_pieces(color) & self.type_pieces(piece_type)
    }
}
