//! Specifies the [Line] type.

use std::str::FromStr;

use regex::Regex;

use crate::{parse::{alphabetic_file_to_numeric, rank_to_numeric, NotationParseError}, UInt};

use super::Square;

/// A [Line] represents a specific rank, file, or both (i.e a square.)
///
/// It is used as the discriminant for [crate::parse::MoveCommandData] when
/// more than one identical piece could reach the same destination square.
/// In chess, when this happens, we use either the rank or file to distinguish between them.
/// In exceedingly rare situations we may require both the rank and file to fully distinguish
/// between them.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Line {
    /// This line represents a rank of squares of the given index.
    Rank(UInt),
    /// This line represents a file of squares of the given index. (0 = a, 1 = b, etc.)
    File(UInt),
    /// On the rare case that neither is enough to distinguish between two pieces, we CAN specify
    /// both I suppose.
    RankAndFile(UInt, UInt),
}

impl Line
{
    /// Returns true if a square lies along the specified line, false otherwise.
    ///
    /// # Arguments
    ///
    /// * `square` - A square to test against.
    ///
    /// # Examples
    ///
    /// ```
    /// let line = Line::Rank(0)
    /// assert!(line.has_square(Square::new(0,0)));
    /// assert!(line.has_square(Square::new(0,7)));
    /// assert!(!line.has_square(Square::new(1,3)));
    ///
    /// let line = Line::File(3)
    /// assert!(line.has_square(Square::new(0, 3)));
    /// assert!(line.has_square(Square::new(7, 3)));
    /// assert!(!line.has_square(Square::new(4, 2)));
    ///
    /// let line = Line::RankAndFile(5,5);
    /// assert!(line.has_square(Square::new(5, 5)));
    /// assert!(!line.has_square(Square::new(4, 5)));
    /// assert!(!line.has_square(Square::new(5, 6)));
    /// assert!(!line.has_square(Square::new(6, 6)));
    /// ```
    pub fn has_square(&self, square: &Square) -> bool
    {
        match self
        {
            Self::Rank(rank) => square.rank == *rank,
            Self::File(file) => square.file == *file,
            Self::RankAndFile(rank, file) => square.rank == *rank && square.file == *file,
        }
    }
}

impl FromStr for Line
{
    type Err = NotationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([a-zA-Z]*)([0-9]*)").unwrap();
        let captures = re.captures(s);
        if captures.is_none()
        {
            return Err(NotationParseError::InvalidFormat(s.to_string()));
        }

        let captures = captures.unwrap();
        let file_str = captures.get(1).unwrap().as_str();
        let rank_str = captures.get(2).unwrap().as_str();

        match (file_str.is_empty(), rank_str.is_empty())
        {
            // There should be at least one of the two.
            (true, true) => Err(NotationParseError::InvalidFormat(s.to_string())),
            // File is blank, so we just have a rank.
            (true, false) => Ok(Line::Rank(rank_to_numeric(rank_str)?)),
            // Rank is blank, so we just have a file to convert.
            (false, true) => Ok(Line::File(alphabetic_file_to_numeric(file_str)?)),
            (false, false) => Ok(Line::RankAndFile(
                    rank_to_numeric(rank_str)?,
                    alphabetic_file_to_numeric(file_str)?
            )),
        }
    }
}
