use crate::parse::MoveCommand;

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
    pub fn initialize_board_with_configuration(board_configuration: BoardConfiguration) -> Self
    {
        todo!();
    }

    /// Gets the board configuration associated with the current board state.
    pub fn get_board_configuration(&self) -> BoardConfiguration
    {
        todo!();
    }

    /// Attempts to make a move on the board, returning a new board state wrapped in [Ok] if
    /// successful, or a [MoveError] if the move was invalid for whatever reason.
    ///
    /// Does not modify the board, but instead returns a new board with the move made.
    pub fn attempt_move(&self, attempted_move: MoveCommand) -> Result<Self, MoveError>
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
    ///
    /// To further check if a move is legal, use [check_move()]
    ///
    /// Because this function returns an unchecked move, it is a private function to prevent bad
    /// state from leaking out into the world.
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
    fn get_move(&self, move_command: MoveCommand) -> Result<Move, MoveError>
    {
        todo!();
    }

    /// Checks whether or not a move is legal.
    ///
    /// Does this check if a move is possible? Probably!
    ///
    /// Returns false
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

    /// Consumes a move and returns a new board where the move has been made.
    ///
    /// This function assumes that the move is valid and does not do any checks
    /// to see if the attempted move is valid, hence this being a private function.
    ///
    /// If you want to attempt to make a move with all the expected checks in place, use
    /// [Board::attempt_move]
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

#[cfg(test)]
mod tests
{
    use std::str::FromStr;

    use crate::{board::BoardConfiguration, parse::MoveCommand};

    use super::Board;

    #[test]
    fn test_making_legal_move_works()
    {
        let board = Board::default_starting_board();
        let move_command = MoveCommand::from_str("e4").unwrap();
        let new_board = board.attempt_move(move_command).unwrap();
        let expected_new_board_config = BoardConfiguration::from_str("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1").unwrap();
        assert_eq!(expected_new_board_config, new_board.get_board_configuration());
    }

    #[test]
    fn test_making_impossible_move_fails()
    {
        let board = Board::default_starting_board();
        let move_command = MoveCommand::from_str("Qe3").unwrap();
        let new_board = board.attempt_move(move_command);
        assert!(new_board.is_err());
    }

    #[test]
    fn test_moving_through_occupancy_fails()
    {
        let board = Board::default_starting_board();
        let move_command = MoveCommand::from_str("Qd3").unwrap();
        let new_board = board.attempt_move(move_command);
        assert!(new_board.is_err());
    }

    #[test]
    fn test_moving_while_in_check_fails()
    {
        let board = Board::initialize_board_with_configuration(BoardConfiguration::from_str("q6k/8/8/3B4/8/8/8/K7 w - - 0 1").unwrap());
        let move_command = MoveCommand::from_str("Bc4").unwrap();
        let new_board = board.attempt_move(move_command);
        assert!(new_board.is_err());
    }

    #[test]
    fn test_moving_out_of_check_succeeds()
    {
        let board = Board::initialize_board_with_configuration(BoardConfiguration::from_str("q6k/8/8/3B4/8/8/8/K7 w - - 0 1").unwrap());
        let move_command = MoveCommand::from_str("Kb2").unwrap();
        let new_board = board.attempt_move(move_command);
        assert!(new_board.is_ok());
    }

    #[test]
    fn test_blocking_check_works()
    {
        let board = Board::initialize_board_with_configuration(BoardConfiguration::from_str("q6k/8/8/3B4/8/8/8/K7 w - - 0 1").unwrap());
        let move_command = MoveCommand::from_str("Ba2").unwrap();
        let new_board = board.attempt_move(move_command);
        assert!(new_board.is_ok());
    }

    #[test]
    fn test_capturing_checking_piece_works()
    {
        let board = Board::initialize_board_with_configuration(BoardConfiguration::from_str("q6k/8/8/3B4/8/8/8/K7 w - - 0 1").unwrap());
        let move_command = MoveCommand::from_str("Bxa8").unwrap();
        let new_board = board.attempt_move(move_command);
        assert!(new_board.is_ok());
    }
}

