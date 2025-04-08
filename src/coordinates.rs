//! This module handles converting between coordinates in algebraic notation (i.e a3, c6, h1) and
//! their tuple/numerical representations.

use std::{fmt::Display, num::ParseIntError};

use log::{debug, error, trace};
use regex::Regex;
use thiserror::Error;

/// The type returned by [algebraic_to_tuple], containing the two-part conversion of the alphabetic
/// part of the string into the file and the numeric part of the string into the rank.
/// Either conversion can fail for different reasons, but the other part *can* still be intact and
/// worth parsing.
#[derive(Debug, PartialEq)]
pub struct Coordinate 
{
    /// The result of parsing the numerical part of the string. Is [Ok] if the parse succeeded,
    /// otherwise is an error of type [RankToNumericError]
    pub rank: Result<u32, RankToNumericError>,
    /// The result of parsing the alphabetical part of the string. Is [Ok] if the parse succeeded,
    /// otherwise is an error of type [AlphabeticFileToNumericError]
    pub file: Result<u32, AlphabeticFileToNumericError>,
}

impl Display for Coordinate
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rank = match &self.rank {
            Ok(i) => i as &dyn Display,
            Err(e) => e as &dyn Display,
        };

        let file = match &self.file {
            Ok(i) => i as &dyn Display,
            Err(e) => e as &dyn Display,
        };

        write!(f, "(\n\t{}\n\t{}\n)", rank, file)
    }
}


/// The error type returned by [algebraic_to_tuple]. This error variant covers the error cases
/// where the attempt to split the string up into rank and file failed, before either subsection
/// could even be attempted to be parsed.
#[derive(Debug, Error)]
pub enum AlgebraicToTupleError
{
    /// The error variant returned if the string did not conform to the regex used to check if the
    /// string represents a valid algebraic coordinate.
    #[error("The string `{0}` was not a correctly formatted algebraic notation coordinate string")]
    InvalidFormat(String)
}


/// Converts an algebraic notated string (such as "a5", "c8", etc)
/// into a tuple of the form 
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
/// # use rust_chess_engine::coordinates::algebraic_to_tuple;
/// # use rust_chess_engine::coordinates::Coordinate;
/// let tuple = algebraic_to_tuple("a4").unwrap();
/// assert_eq!(tuple, Coordinate { rank: Ok(3), file: Ok(0) });
/// ```
///
/// Note that algebraic notation is 1-based, but the tuple notation is 0-based.
/// Thus a = 0 for files, and the first rank (or rank 1) is rank 0.
pub fn algebraic_to_tuple(algebraic_notated_string: &str) -> Result<Coordinate, AlgebraicToTupleError>
{
    let re = Regex::new(r"([a-zA-Z]+)([0-9]+)").unwrap();
    let captures = re.captures(algebraic_notated_string);
    if captures.is_none()
    {
        return Err(AlgebraicToTupleError::InvalidFormat(algebraic_notated_string.to_string()));
    }

    let captures = captures.unwrap();
    let file_str = captures.get(1).unwrap().as_str();
    let rank_str = captures.get(2).unwrap().as_str();
    let file = alphabetic_file_to_numeric(file_str);
    let rank = rank_to_numeric(rank_str);

    Ok(Coordinate 
    {
        rank,
        file
    })
}

/// The error type returned by [alphabetic_file_to_numeric]
#[derive(Debug, Error, PartialEq)]
pub enum AlphabeticFileToNumericError 
{
    /// The error variant returned if an invalid character was found while parsing the string.
    #[error("An invalid character was found when attempting to parse the file name: `{0}`")]
    InvalidCharacterInFileName(String),
    #[error("The string `{0}` was either empty or consisted entirely of invalid characters")]
    EmptyString(String),
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
/// # use rust_chess_engine::coordinates::alphabetic_file_to_numeric;
/// # env_logger::init();
/// assert_eq!(alphabetic_file_to_numeric("A").unwrap(), 0);
/// assert_eq!(alphabetic_file_to_numeric("h").unwrap(), 7);
/// // Non-standard chess files
/// assert_eq!(alphabetic_file_to_numeric("i").unwrap(), 8);
/// assert_eq!(alphabetic_file_to_numeric("z").unwrap(), 25);
/// assert_eq!(alphabetic_file_to_numeric("aa").unwrap(), 26);
/// // Invalid chess file notation
/// assert!(alphabetic_file_to_numeric("!@#dsf234").is_err());
/// assert!(alphabetic_file_to_numeric("").is_err());
/// ```
pub fn alphabetic_file_to_numeric(alphabetic_file: &str) -> Result<u32, AlphabeticFileToNumericError>
{
    trace!("Entering alphabetic_file_to_numeric()");
    trace!("alphabetic_file: {}", alphabetic_file);
    let mut number: u32 = 0;
    let mut power: u32 = 1;

    for char in alphabetic_file.chars().rev()
    {
        let new_char = char.to_ascii_lowercase();
        // We are going to skip whitespace characters as if they don't exist but otherwise continue
        // to parse/handle the string. A whitespace character isn't an error.
        if new_char.is_whitespace()
        {
            debug!("Whitespace character found while converting file to numeric, skipping");
            continue;
        }
        // Otherwise check if it's alphabetic and do some MATH
        if new_char.is_alphabetic()
        {
            debug!("Alphabetic character found. Converting.");
            // Convert ASCII into their digits such that 'a' has a value of 1, 'b' has a value of
            // 2, etc.
            number += (new_char as u32 - 'a' as u32 + 1) * power;
            power *= 26;
            debug!("New running total: {}", number);
            debug!("Power: {}", power);
        }
        else
        {
            error!("Invalid character found while converting file to numeric, returning with error.");
            return Err(AlphabeticFileToNumericError::InvalidCharacterInFileName(new_char.to_string()));
        }
    }

    if number == 0
    {
        error!("Input file string was empty or for some reason the total was 0. It should be at least 1.");
        return Err(AlphabeticFileToNumericError::EmptyString(alphabetic_file.to_string()));
    }
    number -= 1;
    Ok(number)
}

/// The error type returned by [rank_to_numeric]
#[derive(Debug, Error, PartialEq)]
pub enum RankToNumericError
{
    /// The error variant returned if the rank could not be parsed as a number.
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    /// The error variant returned if the rank was "0".
    #[error("Attempted to return a rank with a negative index (did you try to pass rank_to_numeric(\"0\")?)")]
    InvalidCoordinate,
}

/// Converts a rank as a string into a rank as a number. Ranks are already numeric so this is just
/// a very thin wrapper around [str::Parse]. As noted in [alphabetic_file_to_numeric], a rank
/// of 1 represents a coordinate with an index of 0, so passing "0" into this function is actually
/// an error and will return as such.
pub fn rank_to_numeric(rank_str: &str) -> Result<u32, RankToNumericError>
{
    let rank = rank_str.parse::<u32>()?;
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
        let tests = [("1", 0), ("2", 1), ("10", 9)];
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
