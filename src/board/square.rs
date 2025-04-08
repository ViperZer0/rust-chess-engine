//! Specifies the [Square] type.

use crate::UInt;

/// Represents a space on the board, specified by its coordinates in both
/// rank and file.
#[derive(Debug, PartialEq)]
pub struct Square {
    rank: UInt,
    file: UInt,
}

impl Square {
    /// Creates a new square with the given rank and file coordinates.
    ///
    /// Note that in standard algebraic notation, the file precedes the rank,
    /// but in the tuple coordinate system used here and throughout this code base, the rank
    /// preceds the file; so the coordinate system is flipped.
    ///
    /// # Arguments
    ///
    /// * `rank` - The rank (or row) of the square.
    /// * `file` - The file (or column) of the square.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::Square;
    /// // Represents the square a1.
    /// let square = Square::new(0,0);
    /// // Represents the square b3.
    /// let square2 = Square::new(2,1);
    /// ```
    pub fn new(rank: UInt, file: UInt) -> Self
    {
        Square
        {
            rank,
            file
        }
    }
}
