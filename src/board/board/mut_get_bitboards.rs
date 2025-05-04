//! This module is a helper module for [Board](super::Board). This just cleans up the (main
//! module)[super] somewhat so it's not so big.

use crate::{bitboard::Bitboard, board::{PieceType, PlayerColor}};

use super::Board;

impl Board
{
    /// Gets the bitboard corresponding with pieces of a specific color.
    /// 
    /// This is private and only used for internal operations within [Board].
    ///
    /// If you are an external user, you should use the [BoardQuery](super::board_query) API.
    ///
    ///
    /// # Arguments
    ///
    /// * `color` - The player color to select the bitboard of.
    ///
    /// # Examples
    ///
    /// ```
    /// [TODO:write some example code]
    /// ```
    pub fn pieces_of_color(&self, color: PlayerColor) -> Bitboard
    {
        match color
        {
            PlayerColor::White => self.white_pieces,
            PlayerColor::Black => self.black_pieces,
        }
    }

    /// Gets the bitboard corresponding with pieces of a specific color, like
    /// [Self::pieces_of_color], but returns a &mut [Bitboard] instead of a copy of the bitboard,
    /// allowing the internal bitboard to be modified.
    ///
    /// # Arguments
    ///
    /// * `color` - The player color to select the bitboard of.
    ///
    /// # Examples
    ///
    /// ```
    /// [TODO:write some example code]
    /// ```
    pub fn pieces_of_color_as_mut(&mut self, color: PlayerColor) -> &mut Bitboard
    {
        match color
        {
            PlayerColor::White => &mut self.white_pieces,
            PlayerColor::Black => &mut self.black_pieces,
        }
    }

    /// Gets the bitboard corresponding with pieces of a specific type. 
    /// As with [Self::pieces_of_color] this is an internal operation. If you're an external user,
    /// you should use the [BoardQuery](super::board_query) API instead.
    ///
    /// # Arguments
    ///
    /// * `piece_type` - The piece type to select for.
    ///
    /// # Examples
    ///
    /// ```
    /// [TODO:write some example code]
    /// ```
    pub fn pieces_of_type(&self, piece_type: PieceType) -> Bitboard
    {
        match piece_type
        {
            PieceType::Pawn => self.pawn_pieces,
            PieceType::Rook => self.rook_pieces,
            PieceType::King => self.king_pieces,
            PieceType::Queen => self.queen_pieces,
            PieceType::Knight => self.knight_pieces,
            PieceType::Bishop => self.bishop_pieces,
        }
    }

    /// Gets the bitboard corresponding with pieces of a specific type, but
    /// makes it mutable so that [Board] can mutate its internal bitboard state.
    ///
    /// Returns a &mut [Bitboard].
    ///
    /// # Arguments
    ///
    /// * `piece_type` - The piece type to select for.
    ///
    /// # Examples
    ///
    /// ```
    /// [TODO:write some example code]
    /// ```
    pub fn pieces_of_type_as_mut(&mut self, piece_type: PieceType) -> &mut Bitboard
    {
        match piece_type
        {
            PieceType::Pawn => &mut self.pawn_pieces,
            PieceType::Rook => &mut self.rook_pieces,
            PieceType::King => &mut self.king_pieces,
            PieceType::Queen => &mut self.queen_pieces,
            PieceType::Knight => &mut self.knight_pieces,
            PieceType::Bishop => &mut self.bishop_pieces,
        }
    }
}
