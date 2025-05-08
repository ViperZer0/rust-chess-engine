//! This module is a helper module for [Board](super::Board). This just cleans up the (main
//! module)[super] somewhat so it's not so big.

use crate::{bitboard::Bitboard, board::{PieceType, PlayerColor}};

use super::Board;

impl Board
{
    /// Gets the bitboard corresponding with pieces of a specific color.
    /// 
    /// This is intended internal operations within [Board].
    /// If you are an external user, you should use the [BoardQuery](super::board_query) API.
    ///
    /// To get pieces of both a specific color and type, get a [Bitboard] for piece types and 
    /// one for piece colors and use [&](core::ops::BitAnd) to get the intersection of both.
    ///
    /// # Arguments
    ///
    /// * `color` - The player color to select the bitboard of.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{Board, PlayerColor};
    /// let board = Board::new_default_starting_board();
    /// let white_pieces = board.pieces_of_color(PlayerColor::White);
    /// let black_pieces = board.pieces_of_color(PlayerColor::Black);
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
    /// Be careful with this! Boards are intended to be immutable, this function is intended for
    /// internal use only!
    ///
    /// # Arguments
    ///
    /// * `color` - The player color to select the bitboard of.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{Board, PlayerColor};
    /// # use rust_chess_engine::bitboard::Bitboard;
    /// let mut board = Board::new_default_starting_board();
    /// let mut white_pieces = board.pieces_of_color_as_mut(PlayerColor::White);
    /// white_pieces.set_bit(10, true);
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
    ///
    /// As with [Self::pieces_of_color] this is an internal operation. If you're an external user,
    /// you should use the [BoardQuery](super::board_query) API instead.
    ///
    /// To get pieces of both a specific color and type, get a [Bitboard] for piece types and 
    /// one for piece colors and use [&](core::ops::BitAnd) to get the intersection of both.
    ///
    /// # Arguments
    ///
    /// * `piece_type` - The piece type to select for.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{Board, PieceType};
    /// let board = Board::new_default_starting_board();
    /// let knights = board.pieces_of_type(PieceType::Knight);
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
    /// # use rust_chess_engine::board::{Board, PieceType};
    /// # use rust_chess_engine::bitboard::Bitboard;
    /// let mut board = Board::new_default_starting_board();
    /// let mut pawn_pieces = board.pieces_of_type_as_mut(PieceType::Pawn);
    /// pawn_pieces.set_bit(10, true);
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
