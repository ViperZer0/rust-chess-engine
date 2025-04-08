//! Specifies the [Line] type.

use crate::UInt;

/// A [Line] represents EITHER a rank or file.
pub enum Line {
    /// This line represents a rank of squares of the given index.
    Rank(UInt),
    /// This line represents a file of squares of the given index. (0 = a, 1 = b, etc.)
    File(UInt),
}
