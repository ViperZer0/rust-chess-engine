//! Error types for the [crate::board] module

use thiserror::Error;

/// Errors that are returned by the board when attempting to make an invalid move.
#[derive(Debug, Error)]
pub enum MoveError
{
    /// The error returned when there was *no* possible moves.
    #[error("No possible moves")]
    NoPossibleMove,
    /// The error returned when there *was* a move found,
    /// but making this move would be illegal (violates check, etc.)
    #[error("The given move would be illegal")]
    IllegalMove,
    /// The error returned when we found more than one possible move.
    #[error("Too many moves found that matched the command given! Do you need to add a discriminant?")]
    TooManMoves,
}
