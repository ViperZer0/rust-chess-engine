//! Specifies the [Line] type.

/// A [Line] represents EITHER a rank or file.
pub enum Line {
    Rank(u8),
    File(u8),
}
