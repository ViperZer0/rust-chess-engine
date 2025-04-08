//! Specifies the [PieceType] type.

use std::str::FromStr;

use thiserror::Error;

/// One of the six valid chess piece types.
/// Can be:
/// - Pawn
/// - Knight
/// - Bishop
/// - Rook
/// - Queen
/// - King
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

/// Any error type returned by [PieceType::from_str]
#[derive(Error, Debug)]
#[error("The piece type `{first_char}` was not a valid or expected piece type")]
pub struct PieceTypeParseError
{
    /// The first character of the parsed string that didn't match any 
    /// of the expected standard notations for piece types.
    first_char: String  
}

impl PieceTypeParseError
{
    fn new(piece_type_string: &impl ToString) -> Self
    {
        Self {
            first_char: piece_type_string.to_string(),
        }
    }
}

impl FromStr for PieceType
{
    type Err = PieceTypeParseError;

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
            _ => Err(PieceTypeParseError::new(&first_char.unwrap()))
        }
    }
}
