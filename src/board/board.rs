use crate::parse::{InvalidFENError, MoveCommand};

use super::{error::MoveError, r#move::Move, BoardConfiguration};

/// A given board state.
///
/// This represents a certain arrangement of pieces on a 8x8 chessboard. Standard stuff.
/// The board is expected to begin and stay in a valid configuration, all exposed functions should
/// maintain the internal state and not violate the rules of chess or anything. Boards are
/// immutable: making moves on a board does not modify the existing board but instead returns a new
/// one.
pub struct Board 
{
}

impl Board {
    /// Creates a new board with the "standard" configuration seen in classical chess.
    ///
    /// Creates and returns a new Board with the default arrangement and number of pieces, 16
    /// pawns, 8 per side, 4 knights, 4 rooks, etc.
    pub fn default_starting_board() -> Self
    {
        todo!();
    }

    /// Allows creation of a new board with a custom configuration.
    ///
    /// Takes a [BoardConfiguration] which represents the desired starting state of the board.
    pub fn initialize_board_with_configuration(board_configuration: BoardConfiguration) -> Result<Self, InvalidFENError>
    {
        todo!();
    }

    /// Gets the board configuration associated with the current board state.
    pub fn get_board_configuration() -> BoardConfiguration
    {
        todo!();
    }

    /// Attempts to make a move on the board, returning a new board state wrapped in [Ok] if
    /// successful, or a [MoveError] if the move was invalid for whatever reason.
    ///
    /// Does not modify the board, but instead returns a new board with the move made.
    pub fn attempt_move(attempted_move: MoveCommand) -> Result<Self, MoveError>
    {
        todo!();
    }

    /// Converts a [MoveCommand] into a [Move] that may or may not be legal.
    ///
    /// Where a [MoveCommand] represents a 1-to-1 relationship with algebraic notation, it's not
    /// super helpful for figuring out what a valid move entails. We know nothing about the
    /// starting position of the piece being moved, what, if anything is captured, etc.
    ///
    /// get_move() consumes a [MoveCommand] and returns a more comprehensive [Move] that includes
    /// that information. If there is no possible move that meets the criteria specified by the
    /// MoveCommand, this function returns [None] instead.
    ///
    /// Note that get_move() returns [Some] if a move is *possible*, even if that move is not valid
    /// (i.e violates rules such as escaping check).
    /// To get a legal move ([Move<Legal>]) use [check_move()]
    ///
    /// # Arguments
    ///
    /// * `move_command` - The [MoveCommand] to analyze.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use rust_chess_engine::board::Board;
    /// # use rust_chess_engine::board::Move;
    /// # use rust_chess_engine::parse::MoveCommand;
    /// let board = Board::default_starting_board();
    /// let move_command = MoveCommand::from_str("e4").unwrap();
    /// let r#move = board.get_move(move_command);
    /// assert!(r#move.is_some());
    /// // We don't know whether the move is legal yet,
    /// // only that it is possible.
    /// assert_eq!(false, r#move.unwrap().is_legal());
    /// ```
    fn get_move(&self, move_command: MoveCommand) -> Result<Move, MoveErrors>
    {
        todo!();
    }

    /// Checks whether or not a move is legal.
    ///
    /// # Arguments
    ///
    /// * `attempted_move` - The attempted move. This move has been proven to be possible, but may
    /// or may not be legal.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use rust_chess_engine::board::Board;
    /// # use rust_chess_engine::board::Move;
    /// # use rust_chess_engine::parse::MoveCommand;
    /// let board = Board::default_starting_board();
    /// let move_command = MoveCommand::from_str("e4").unwrap();
    /// let r#move = board.get_move(move_command).unwrap();
    /// let r#move = board.check_move(r#move);
    /// // e4 should be a valid move
    /// assert!(r#move.is_some());
    /// // e4 should be a legal move
    /// assert!(r#move.unwrap().is_legal());
    /// ```
    fn check_move(&self, attempted_move: Move) -> bool 
    {
        todo!();
    }

    /// Consumes a legal move and returns a new copy of the board where the piece has been moved
    /// and all valid state changed.
    ///
    /// # Arguments
    ///
    /// * `legal_move` - A legal move which will be made on the board.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use rust_chess_engine::board::Board;
    /// # use rust_chess_engine::board::Move;
    /// # use rust_chess_engine::parse::MoveCommand;
    /// let board = Board::default_starting_board();
    /// let move_command = MoveCommand::from_str("e4").unwrap();
    /// let r#move = board.get_move(move_command).unwrap();
    /// let r#move = board.check_move(r#move).unwrap();
    /// let new_board = board.make_move(r#move);
    /// ```
    fn make_move(&self, legal_move: Move) -> Self
    {
        todo!();
    }
}
