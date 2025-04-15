use std::{collections::HashMap, fmt::Display};

use super::{Piece, PlayerColor, Square};

/// A specified arrangement of pieces.
///
/// This is equivalent to [FEN](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)
/// thus records the following information:
/// - Piece placement
/// - Active color (whose turn it is)
/// - Whether or not black or white have castled yet
/// - Whether or not a square can be en passant captured
/// - Halfmoves (used to track fifty-move rule)
///     - Number of moves since last capture or pawn advance.
/// - Fullmoves
#[derive(Debug, PartialEq)]
pub struct BoardConfiguration
{
    // I'm not sure if a vec or a hashmap is better here.
    pieces: HashMap<Square, Piece>,
    active_color: PlayerColor,
    castling_availability: CastlingAvailability,
    en_passant_target_square: Option<Square>,
    halfmove_clock: u8,
    fullmove_number: u8,
}

/// This struct records information on who can castle and where.
/// 
/// In chess, castling is only available to a player if:
/// - The king has not moved from its starting square
/// - The rook on the side being castled towards has not moved.
/// 
/// FEN notation does not include information on the previous state of the board,
/// and so has a dedicated field to indicate whether or not the players
/// are allowed to castle or not.
///
/// Note that this doesn't include information on *temporary* scenarios in which castling are
/// prevented. If castling would put the king in check, the option is still available to the king
/// later.
#[derive(Debug, PartialEq)]
pub struct CastlingAvailability
{
    white_castle_kingside: bool,
    white_castle_queenside: bool,
    black_castle_kingside: bool,
    black_castle_queenside: bool,
}

impl BoardConfiguration
{
    /// Gets the pieces of the board as a hashmap.
    pub fn get_pieces(&self) -> &HashMap<Square, Piece>
    {
        return &self.pieces;
    }
}

impl Display for BoardConfiguration
{
    // This will print out the FEN notation of a board configuration.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
