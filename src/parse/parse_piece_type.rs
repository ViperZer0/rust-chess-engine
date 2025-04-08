use std::str::FromStr;
use crate::board::PieceType;

use super::NotationParseError;

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
            _ => Err(NotationParseError::InvalidPieceCharacter(first_char.unwrap().to_string()))
        }
    }
}
