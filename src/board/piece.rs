use super::{PieceType, PlayerColor};

/// Represents one of the player's pieces.
/// 
/// Stores the piece type and the player's color.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Piece
{
    piece_type: PieceType,
    color: PlayerColor,
}

impl Piece
{
    pub fn new(color: PlayerColor, piece_type: PieceType) -> Self
    {
        Self 
        {
            piece_type,
            color,
        }
    }
}
