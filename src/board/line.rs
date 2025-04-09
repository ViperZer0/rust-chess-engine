//! Specifies the [Line] type.

use crate::UInt;

/// A [Line] represents a specific rank, file, or both (i.e a square.)
///
/// It is used as the discriminant for [crate::parse::MoveData] when
/// more than one identical piece could reach the same destination square.
/// In chess, when this happens, we use either the rank or file to distinguish between them.
/// In exceedingly rare situations we may require both the rank and file to fully distinguish
/// between them.
#[derive(Debug)]
pub enum Line {
    /// This line represents a rank of squares of the given index.
    Rank(UInt),
    /// This line represents a file of squares of the given index. (0 = a, 1 = b, etc.)
    File(UInt),
    /// On the rare case that neither is enough to distinguish between two pieces, we CAN specify
    /// both I suppose.
    RankAndFile(UInt, UInt),
}
