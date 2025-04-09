//! Specifies the [PieceType] type.

/// One of the six valid chess piece types.
/// Can be:
/// - Pawn
/// - Knight
/// - Bishop
/// - Rook
/// - Queen
/// - King
#[derive(Debug)]
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


