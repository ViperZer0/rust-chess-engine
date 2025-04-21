use std::collections::HashMap;

use crate::parse::MoveCommand;

use super::{error::MoveError, r#move::Move, BoardConfiguration, BoardResult, Piece, Square};

/// A given board state.
///
/// This represents a certain arrangement of pieces on a 8x8 chessboard. Standard stuff.
/// The board is expected to begin and stay in a valid configuration, all exposed functions should
/// maintain the internal state and not violate the rules of chess or anything. Boards are
/// immutable: making moves on a board does not modify the existing board but instead returns a new
/// one.
#[derive(Clone)]
pub struct Board 
{
    /// Tracks what piece is on a given square.
    /// If a key does not exist, that square is considered empty.
    piece_mailbox: HashMap<Square, Piece>
}

impl Board {
    /// Creates a new board with the "standard" configuration seen in classical chess.
    ///
    /// Creates and returns a new Board with the default arrangement and number of pieces, 16
    /// pawns, 8 per side, 4 knights, 4 rooks, etc.
    pub fn new_default_starting_board() -> Self
    {
        todo!();
    }

    /// Creates a new empty board.
    ///
    /// This board has no pieces on it.
    pub fn new_blank_board() -> Self
    {
        todo!();
    }

    /// Allows creation of a new board with a custom configuration.
    ///
    /// Takes a [BoardConfiguration] which represents the desired starting state of the board.
    pub fn new_board_with_configuration(board_configuration: &BoardConfiguration) -> Self
    {
        todo!();
    }

    /// Gets the board configuration associated with the current board state.
    pub fn get_board_configuration(&self) -> BoardConfiguration
    {
        todo!();
    }

    /// Returns the current result of the board as a [BoardResult].
    ///
    /// This contains information about if the game is over (and who won) or if the game is still
    /// in progress.
    pub fn get_game_result(&self) -> BoardResult
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

    /// Gets the piece located on a given square.
    ///
    /// If the square is currently empty, this function returns [None].
    /// If the square is occupied, this function returns a reference to the [Piece].
    pub fn get_piece(&self, square: &Square) -> Option<&Piece>
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
    /// * `r#move` - A legal move which will be made on the board.
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
    fn make_move(&self, r#move: Move) -> Self
    {
        let mut new_board = self.clone();
        new_board.make_move_in_place(r#move);
        return new_board;
    }

    /// The in-place version of [Board::make_move]. 
    ///
    /// Instead of returning a new board, this function modifies it in place, altering the board
    /// state as necessary to represent a valid configuration.
    fn make_move_in_place(&mut self, r#move: Move)
    {
        todo!();
    }

    /// Adds a piece onto the board in the set position.
    ///
    /// This function modifies the current board and sets the given square to be the provided
    /// piece. This doesn't do any checks to see if the provided square is already occupied, and
    /// instead just forcibly overwrites the given location. This also doesn't move the given piece
    /// from its old location, so this function is intended to be used in conjunction with and
    /// after [remove_piece].
    fn add_piece(&mut self, piece: Piece, position: &Square)
    {
        self.piece_mailbox.insert(*position, piece);
    }

    /// Erases a piece from the board.
    /// 
    /// This function modifies the current board and removes whatever piece, if any, exists on the
    /// board in that location. This doesn't do any checks to see what, if any, piece is there, it
    /// simply removes all references to a piece on that square. This can be the result of a
    /// capture, where the captured piece is removed from its square before it is replaced, or
    /// simply a move where there is no longer a piece on the starting square.
    fn remove_piece(&mut self, position: &Square)
    {
        self.piece_mailbox.remove(position);
    }
}

#[cfg(test)]
mod tests
{
    use std::str::FromStr;

    use crate::{board::{BoardConfiguration, Piece, PieceType, PlayerColor, Square}, parse::MoveCommand};

    use super::Board;

    #[test]
    fn test_making_legal_move_works()
    {
        let board = Board::new_default_starting_board();
        let move_command = MoveCommand::from_str("e4").unwrap();
        let new_board = board.attempt_move(move_command).unwrap();
        let expected_new_board_config = BoardConfiguration::from_str("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1").unwrap();
        assert_eq!(expected_new_board_config, new_board.get_board_configuration());
    }

    #[test]
    fn test_making_impossible_move_fails()
    {
        let board = Board::new_default_starting_board();
        let move_command = MoveCommand::from_str("Qe3").unwrap();
        let new_board = board.attempt_move(move_command);
        assert!(new_board.is_err());
    }

    #[test]
    fn test_moving_through_occupancy_fails()
    {
        let board = Board::new_default_starting_board();
        let move_command = MoveCommand::from_str("Qd3").unwrap();
        let new_board = board.attempt_move(move_command);
        assert!(new_board.is_err());
    }

    #[test]
    fn test_moving_while_in_check_fails()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("q6k/8/8/3B4/8/8/8/K7 w - - 0 1").unwrap());
        let move_command = MoveCommand::from_str("Bc4").unwrap();
        let new_board = board.attempt_move(move_command);
        assert!(new_board.is_err());
    }

    #[test]
    fn test_moving_out_of_check_succeeds()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("q6k/8/8/3B4/8/8/8/K7 w - - 0 1").unwrap());
        let move_command = MoveCommand::from_str("Kb2").unwrap();
        let new_board = board.attempt_move(move_command);
        assert!(new_board.is_ok());
    }

    #[test]
    fn test_blocking_check_works()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("q6k/8/8/3B4/8/8/8/K7 w - - 0 1").unwrap());
        let move_command = MoveCommand::from_str("Ba2").unwrap();
        let new_board = board.attempt_move(move_command);
        assert!(new_board.is_ok());
    }

    #[test]
    fn test_capturing_checking_piece_works()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("q6k/8/8/3B4/8/8/8/K7 w - - 0 1").unwrap());
        let move_command = MoveCommand::from_str("Bxa8").unwrap();
        let new_board = board.attempt_move(move_command);
        assert!(new_board.is_ok());
    }

    #[test]
    fn test_clone_configuration_identical()
    {
        let board = Board::new_default_starting_board();
        let new_board = board.clone();
        assert_eq!(board.get_board_configuration(), new_board.get_board_configuration());
    }

    #[test]
    fn test_clone_identical_for_custom_board_config()
    {
        let board_config = BoardConfiguration::from_str("q7/8/8/8/8/8/1Q6/1K6 w - - 0 1").unwrap();
        let board = Board::new_board_with_configuration(&board_config);
        let new_board = board.clone();
        assert_eq!(board_config, new_board.get_board_configuration());
    }

    #[test]
    fn initialize_board_in_progress()
    {
        let board = Board::new_default_starting_board();
        assert!(board.get_game_result().is_in_progress());
    }

    #[test]
    fn initialize_board_in_end_position()
    {
        let board_config= BoardConfiguration::from_str("3k4/3Q4/3K4/8/8/8/8/8 b - - 0 1").unwrap();
        let board = Board::new_board_with_configuration(&board_config);
        assert!(board.get_game_result().is_over());
        assert!(!board.get_game_result().is_draw());
        assert_eq!(PlayerColor::White, board.get_game_result().get_winner().unwrap());
    }

    #[test]
    fn initialize_board_move_into_end_position()
    {
        let board_config = BoardConfiguration::from_str("3k4/8/3K4/8/Q7/8/8/8 w - - 0 1").unwrap();
        let board = Board::new_board_with_configuration(&board_config);
        let r#move = MoveCommand::from_str("Qd7").unwrap();
        let new_board = board.attempt_move(r#move).unwrap();
        assert!(new_board.get_game_result().is_over());
        assert!(!new_board.get_game_result().is_draw());
        assert_eq!(PlayerColor::White, new_board.get_game_result().get_winner().unwrap());
    }

    #[test]
    fn adding_piece_to_board_works()
    {
        let mut board = Board::new_blank_board();
        let square = Square::new(0, 0);
        let new_piece = Piece::new(PlayerColor::White, PieceType::Pawn);
        assert!(board.get_piece(&square).is_none());
        board.add_piece(new_piece, &square);
        assert!(board.get_piece(&square).is_some());
        assert_eq!(new_piece, *board.get_piece(&square).unwrap());
    }

    #[test]
    fn removing_piece_from_board_works()
    {
        let mut board = Board::new_default_starting_board();
        let square = Square::new(0, 0);
        assert!(board.get_piece(&square).is_some());
        board.remove_piece(&square);
        assert!(board.get_piece(&square).is_none());
    }

    #[test]
    fn default_starting_board_has_all_correct_pieces()
    {
        let squares_to_pieces = [
            ("a1", Piece::new(PlayerColor::White, PieceType::Rook)),
            ("b1", Piece::new(PlayerColor::White, PieceType::Knight)),
            ("c1", Piece::new(PlayerColor::White, PieceType::Bishop)),
            ("d1", Piece::new(PlayerColor::White, PieceType::Queen)),
            ("e1", Piece::new(PlayerColor::White, PieceType::King)),
            ("f1", Piece::new(PlayerColor::White, PieceType::Bishop)),
            ("g1", Piece::new(PlayerColor::White, PieceType::Knight)),
            ("h1", Piece::new(PlayerColor::White, PieceType::Rook)),
            ("a2", Piece::new(PlayerColor::White, PieceType::Pawn)),
            ("b2", Piece::new(PlayerColor::White, PieceType::Pawn)),
            ("c2", Piece::new(PlayerColor::White, PieceType::Pawn)),
            ("d2", Piece::new(PlayerColor::White, PieceType::Pawn)),
            ("e2", Piece::new(PlayerColor::White, PieceType::Pawn)),
            ("f2", Piece::new(PlayerColor::White, PieceType::Pawn)),
            ("g2", Piece::new(PlayerColor::White, PieceType::Pawn)),
            ("a8", Piece::new(PlayerColor::Black, PieceType::Rook)),
            ("b8", Piece::new(PlayerColor::Black, PieceType::Knight)),
            ("c8", Piece::new(PlayerColor::Black, PieceType::Bishop)),
            ("d8", Piece::new(PlayerColor::Black, PieceType::Queen)),
            ("e8", Piece::new(PlayerColor::Black, PieceType::King)),
            ("f8", Piece::new(PlayerColor::Black, PieceType::Bishop)),
            ("g8", Piece::new(PlayerColor::Black, PieceType::Knight)),
            ("h8", Piece::new(PlayerColor::Black, PieceType::Rook)),
            ("a7", Piece::new(PlayerColor::Black, PieceType::Pawn)),
            ("b7", Piece::new(PlayerColor::Black, PieceType::Pawn)),
            ("c7", Piece::new(PlayerColor::Black, PieceType::Pawn)),
            ("d7", Piece::new(PlayerColor::Black, PieceType::Pawn)),
            ("e7", Piece::new(PlayerColor::Black, PieceType::Pawn)),
            ("f7", Piece::new(PlayerColor::Black, PieceType::Pawn)),
            ("g7", Piece::new(PlayerColor::Black, PieceType::Pawn)),
            ("h7", Piece::new(PlayerColor::Black, PieceType::Pawn)),
        ];

        let board = Board::new_default_starting_board();
        for (square_str, piece) in squares_to_pieces
        {
            let result = board.get_piece(&Square::from_str(square_str).unwrap());
            assert!(result.is_some());
            assert_eq!(piece, *result.unwrap());
        }
    }
}
