//! Specifies the [MoveCommand] type.

use std::str::FromStr;

/// Represents a parsed move from algebraic notation
///
/// This type guarantees that the contained move is sound (in this case that means the given
/// notation was gramatically correct) but does not check if the move is valid (does not break
/// movement rules, rules about escaping check, etc.)
/// 
/// This struct should be a mostly 1-to-1 representation of algebraic notation.
/// 
/// For example, the move Bxa3 would be split up as follows:
/// - The piece type is a bishop
/// - Capture is true
/// - Target square is a3.
///
/// A MoveCommand can also handle a discriminant, i.e Nca3 would mean:
/// - The piece type is a knight
/// - Between the two knights, the one on the c file is the one being moved (discriminant)
/// - Capture is false
/// - Target square is a3.
pub struct MoveCommand {
    /// What piece type is being moved. Defaults to pawn if no letter was specified. 
    piece_type: PieceType,
    /// An optional filter that distinguishes between multiple pieces, if more than one 
    /// is allowed to move to the same location. 
    discriminant: Option<Line>,
    /// Whether or not this move is a capture.
    capture: bool,
    /// The destination square.
    target_square: Square,
}

impl FromStr for MoveCommand
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
