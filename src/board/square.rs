//! Specifies the [Square] type.

use std::str::FromStr;

use regex::Regex;

use crate::{parse::{alphabetic_file_to_numeric, rank_to_numeric, NotationParseError}, UInt};

/// Represents a space on the board, specified by its coordinates in both
/// rank and file.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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

impl FromStr for Square
{
    type Err = NotationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([a-zA-Z]+)([0-9]+)").unwrap();
        let captures = re.captures(s);
        if captures.is_none()
        {
            return Err(NotationParseError::InvalidFormat(s.to_string()));
        }

        let captures = captures.unwrap();
        let file_str = captures.get(1).unwrap().as_str();
        let rank_str = captures.get(2).unwrap().as_str();
        let file = alphabetic_file_to_numeric(file_str)?;
        let rank = rank_to_numeric(rank_str)?;

        Ok(Square::new(rank, file))
    }
}
