//! This module handles converting between coordinates in algebraic notation (i.e a3, c6, h1) and
//! their tuple/numerical representations.

use std::num::ParseIntError;

use thiserror::Error;

/// Converts an algebraic notated string (such as "a5", "c8", etc)
/// into a tuple of the form ([u8], [u8]), where the first entry represents 
/// the file (i.e the alphabetical part of the coordinate) and the second represents the rank (i.e
/// the numerical part of the coordinate). Note that this effectively reverses the order of the
/// coordinate system.
///
/// This function works with both uppercase and lowercase ASCII characters.
///
/// # Arguments
///
/// * `algebraic_notated_string` - An algebraically notated string like b5, c7, d3, etc.
///
/// # Examples
/// 
/// ```
/// let tuple = algebraic_to_tuple("a4");
/// assert_eq!(tuple, (0, 3))
/// ```
///
/// Note that algebraic notation is 1-based, but the tuple notation is 0-based.
/// Thus a = 0 for files, and the first rank (or rank 1) is rank 0.
pub fn algebraic_to_tuple(algebraic_notated_string: &str) -> (u8, u8)
{
    todo!()
}

/// Converts an alphabetical string into its numeric representation as if 
/// the string were a file on a chessboard. While a standard chess board only works with this from
/// the letters a-h, this function supports chessboards up to 26 files and uses all letters from
/// a-z. After that, the function uses "spreadsheet" logic, where the next file after Z is AA, and
/// after ZZ comes AAA, etc.
///
/// This function works with both uppercase and lowercase ASCII characters.
///
/// # Arguments
///
/// * `alphabetic_file` - The coordinates of the file
///
/// # Examples
///
/// ```
/// assert_eq!(alphabetic_file_to_numeric("A"), 0);
/// assert_eq!(alphabetic_file_to_numeric("h"), 7);
/// // Non-standard chess files
/// assert_eq!(alphabetic_file_to_numeric("i"), 8);
/// assert_eq!(alphabetic_file_to_numeric("z"), 25);
/// assert_eq!(alphabetic_file_to_numeric("aa"), 26);
/// ```
pub fn alphabetic_file_to_numeric(alphabetic_file: &str) -> u8
{
    todo!()
}

#[derive(Debug, Error, PartialEq)]
pub enum RankToNumericError
{
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("Attempted to return a rank with a negative index (did you try to pass rank_to_numeric(\"0\")?)")]
    InvalidCoordinate,
}

/// Converts a rank as a string into a rank as a number. Ranks are already numeric so this is just
/// a very thin wrapper around [str::Parse]. As noted in [alphabetic_file_to_numeric], a rank
/// of 1 represents a coordinate with an index of 0, so passing "0" into this function is actually
/// an error and will return as such.
pub fn rank_to_numeric(rank_str: &str) -> Result<u8, RankToNumericError>
{
    let rank = rank_str.parse::<u8>()?;
    // We need to subtract one since a rank of 1 is the 0th rank. Thus if we pass "0", that is
    // actually invalid.
    if rank == 0
    {
        return Err(RankToNumericError::InvalidCoordinate);
    }
    return Ok(rank - 1);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_rank_to_numeric_valid()
    {
        let tests = [("1", 0), ("2", 1u8), ("10", 9u8)];
        for test in tests
        {
            assert_eq!(test.1, rank_to_numeric(test.0).unwrap());
        }
    }

    #[test]
    fn test_rank_to_numeric_zero_fails()
    {
        assert_eq!(RankToNumericError::InvalidCoordinate, rank_to_numeric("0").unwrap_err());
    }

    #[test]
    fn test_rank_to_numeric_invalid_parse_fails()
    {
        let result = rank_to_numeric("ajshdkfljhasld81u2943hasijdfh8&AEWYOUI4o128971y4*").unwrap_err();
        match result {
            RankToNumericError::ParseIntError(_) => assert!(true),
            RankToNumericError::InvalidCoordinate => panic!("Expected ParseIntError, got InvalidCoordinate error"),
        }
    }
}
