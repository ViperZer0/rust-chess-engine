//! Defines error types encountered when parsing a string into a [super::MoveCommand].

use std::num::ParseIntError;

use thiserror::Error;

/// The error type returned by functions in the [crate::parse] module, covering invalid syntax and
/// formatting in a given algebraically notated command
#[derive(Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum NotationParseError
{
    /// The error variant returned when parsing the string as a u32 fails.
    #[error("Something went wrong when parsing the rank: `{0:?}`")]
    ParseIntError(#[from] ParseIntError),
    /// This error variant is returned when a rank of 0 is passed in.
    #[error("The rank coordinates start at 1, not 0!")]
    ZeroRankIndex,
    /// An invalid character was found in the name of the file (only a-z in uppercase or lowercase
    /// is allowed)
    #[error("An invalid character was found when attempting to parse the file name: `{0}`")]
    InvalidCharacterInFileName(String),
    /// The error variant returned specifically when converting the file name to a number returned
    /// 0. This will probably only happen when the string is empty, but there could be other edge
    /// cases where this is thrown.
    #[error("The string was empty")]
    EmptyString(),
    /// The regex that captures and parses algebraic notation failed.
    #[error("The string `{0}` was not a correctly formatted algebraic notation coordinate string")]
    InvalidFormat(String),
    /// The command was given an invalid piece character/type.
    #[error("`{0}` is not a valid piece type")]
    InvalidPieceCharacter(String),
    /// The error variant returned when [alphabetic_file_to_numeric()] overflows.
    #[error("Integer overflow trying to convert {0} to a number.")]
    Overflow(String),
}
