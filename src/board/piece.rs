use std::fmt::Display;

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
    /// Creates a new [Piece]
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the piece
    /// * `piece_type` - The type of piece.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{PieceType, PlayerColor, Piece};
    /// let piece = Piece::new(PlayerColor::White, PieceType::Pawn);
    /// ```
    pub fn new(color: PlayerColor, piece_type: PieceType) -> Self
    {
        Self 
        {
            piece_type,
            color,
        }
    }

    /// Gets the [PieceType] of the piece.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{PieceType, PlayerColor, Piece};
    /// let piece = Piece::new(PlayerColor::White, PieceType::Pawn);
    /// assert_eq!(piece.piece_type(), PieceType::Pawn);
    /// ```
    pub fn piece_type(&self) -> PieceType
    {
        self.piece_type
    }

    /// Gets the [PlayerColor] of a piece.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{PieceType, PlayerColor, Piece};
    /// let piece = Piece::new(PlayerColor::White, PieceType::Pawn);
    /// assert_eq!(piece.color(), PlayerColor::White);
    /// ```
    pub fn color(&self) -> PlayerColor
    {
        self.color
    }
}

impl Display for Piece
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // White pieces show up as uppercase letters,
        // Black pieces show up as lowercase letters,
        // as in FEN.
        let piece_symbol = match (self.color, self.piece_type)
        {
            (PlayerColor::White, PieceType::Pawn) => "P",
            (PlayerColor::White, PieceType::Bishop) => "B",
            (PlayerColor::White, PieceType::Knight) => "N",
            (PlayerColor::White, PieceType::Rook) => "R",
            (PlayerColor::White, PieceType::Queen) => "Q",
            (PlayerColor::White, PieceType::King) => "K",
            (PlayerColor::Black, PieceType::Pawn) => "p",
            (PlayerColor::Black, PieceType::Bishop) => "b",
            (PlayerColor::Black, PieceType::Knight) => "n",
            (PlayerColor::Black, PieceType::Rook) => "r",
            (PlayerColor::Black, PieceType::Queen) => "q",
            (PlayerColor::Black, PieceType::King) => "k",
        };
        write!(f, "{}", piece_symbol)
    }
}
