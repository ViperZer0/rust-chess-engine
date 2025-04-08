//! This module handles converting between coordinates in algebraic notation (i.e a3, c6, h1) and
//! their tuple/numerical representations.

use log::{debug, error, trace};
use regex::Regex;

use crate::{board::Square, UInt};

use super::error::NotationParseError;

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
pub fn algebraic_to_tuple(algebraic_notated_string: &str) -> Result<Square, NotationParseError>
{
    let re = Regex::new(r"([a-zA-Z]+)([0-9]+)").unwrap();
    let captures = re.captures(algebraic_notated_string);
    if captures.is_none()
    {
        return Err(NotationParseError::InvalidFormat(algebraic_notated_string.to_string()));
    }

    let captures = captures.unwrap();
    let file_str = captures.get(1).unwrap().as_str();
    let rank_str = captures.get(2).unwrap().as_str();
    let file = alphabetic_file_to_numeric(file_str)?;
    let rank = rank_to_numeric(rank_str)?;

    Ok(Square::new(rank, file))
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
pub fn alphabetic_file_to_numeric(alphabetic_file: &str) -> Result<UInt, NotationParseError>
{
    trace!("Entering alphabetic_file_to_numeric()");
    trace!("alphabetic_file: {}", alphabetic_file);
    let mut number: UInt = 0;
    let mut power: UInt = 1;

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
            number += (new_char as UInt - 'a' as UInt + 1) * power;
            power *= 26;
            debug!("New running total: {}", number);
            debug!("Power: {}", power);
        }
        else
        {
            error!("Invalid character found while converting file to numeric, returning with error.");
            return Err(NotationParseError::InvalidCharacterInFileName(new_char.to_string()));
        }
    }

    if number == 0
    {
        error!("Input file string was empty or for some reason the total was 0. It should be at least 1.");
        return Err(NotationParseError::EmptyString());
    }
    number -= 1;
    Ok(number)
}

/// Converts a rank as a string into a rank as a number. Ranks are already numeric so this is just
/// a very thin wrapper around [str::Parse]. As noted in [alphabetic_file_to_numeric], a rank
/// of 1 represents a coordinate with an index of 0, so passing "0" into this function is actually
/// an error and will return as such.
pub fn rank_to_numeric(rank_str: &str) -> Result<UInt, NotationParseError>
{
    let rank = rank_str.parse::<UInt>()?;
    // We need to subtract one since a rank of 1 is the 0th rank. Thus if we pass "0", that is
    // actually invalid.
    if rank == 0
    {
        return Err(NotationParseError::ZeroRankIndex);
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
        assert_eq!(NotationParseError::ZeroRankIndex, rank_to_numeric("0").unwrap_err());
    }

    #[test]
    fn test_rank_to_numeric_invalid_parse_fails()
    {
        let result = rank_to_numeric("ajshdkfljhasld81u2943hasijdfh8&AEWYOUI4o128971y4*").unwrap_err();
        match result {
            NotationParseError::ParseIntError(_) => assert!(true),
            other => panic!("Expected ParseIntError, got {:?}", other),
        }
    }
}
