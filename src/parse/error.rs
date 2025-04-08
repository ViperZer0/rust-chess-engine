//! Defines error types encountered when parsing a string into a [super::MoveCommand].

use std::num::ParseIntError;

use thiserror::Error;

/// The error type returned by functions in the [crate::parse] module, covering invalid syntax and
/// formatting in a given algebraically notated command
#[derive(Debug, Error, PartialEq)]
pub enum NotationParseError
{
    #[error("Something went wrong when parsing the rank: `{0:?}`")]
    ParseIntError(#[from] ParseIntError),
    #[error("The rank coordinates start at 1, not 0!")]
    ZeroRankIndex,
    #[error("An invalid character was found when attempting to parse the file name: `{0}`")]
    InvalidCharacterInFileName(String),
    #[error("The string was empty")]
    EmptyString(),
    #[error("The string `{0}` was not a correctly formatted algebraic notation coordinate string")]
    InvalidFormat(String),
}


