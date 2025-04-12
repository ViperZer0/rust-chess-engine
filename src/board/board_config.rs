use std::{collections::HashMap, fmt::Display};

use super::{Piece, Square};

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
