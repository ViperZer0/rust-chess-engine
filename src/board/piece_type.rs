//! Specifies the [PieceType] type.

use std::str::FromStr;

use crate::parse::NotationParseError;

pub const PIECE_TYPES: [PieceType; 6] = [
    PieceType::Pawn,
    PieceType::Knight,
    PieceType::Bishop,
    PieceType::Rook,
    PieceType::Queen,
    PieceType::King];

/// One of the six valid chess piece types.
/// Can be:
/// - Pawn
/// - Knight
/// - Bishop
/// - Rook
/// - Queen
/// - King
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceType {
    /// A pawn piece
    Pawn,
    /// A knight piece
    Knight,
    /// A bishop piece
    Bishop,
    /// A rook piece
    Rook,
    /// A queen piece
    Queen,
    /// A king piece
    King
}

impl FromStr for PieceType
{
    type Err = NotationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // We only care about the first character.
        let first_char = s.chars().next();
        if first_char.is_none()
        {
            // We want to return a pawn if the input string was blank!
            return Ok(Self::Pawn)
        }
        match first_char.unwrap().to_ascii_lowercase()
        {
            'n' => Ok(Self::Knight),
            'b' => Ok(Self::Bishop),
            'r' => Ok(Self::Rook),
            'q' => Ok(Self::Queen),
            'k' => Ok(Self::King),
            // Normal algebraic notation doesn't have this but FEN *does* have this.
            'p' => Ok(Self::Pawn),
            _ => Err(NotationParseError::InvalidPieceCharacter(first_char.unwrap().to_string()))
        }
    }
}
