use std::{collections::HashMap, fmt::Display};

use crate::{bitboard::Bitboard, board::{DrawReason, PieceType, PlayerColor}, parse::MoveCommand};

use super::{board_config::BoardConfigurationBuilder, error::MoveError, r#move::{CastlingDirection, Move}, BoardConfiguration, BoardResult, CastlingAvailability, Piece, Square};
mod board_move;
mod board_query;
mod board_move_logic;
mod board_move_checks;
mod board_move_generation;
mod mut_get_bitboards;

/// A given board state.
///
/// This represents a certain arrangement of pieces on a 8x8 chessboard. Standard stuff.
/// The board is expected to begin and stay in a valid configuration, all exposed functions should
/// maintain the internal state and not violate the rules of chess or anything. Boards are
/// immutable: making moves on a board does not modify the existing board but instead returns a new
/// one.
#[derive(Clone, Debug)]
pub struct Board 
{
    /// Tracks what piece is on a given square.
    /// If a key does not exist, that square is considered empty.
    piece_mailbox: HashMap<Square, Piece>,
    /// All white pieces
    white_pieces: Bitboard,
    /// All black pieces
    black_pieces: Bitboard,

    // These hold *all* pieces of the given type, for each color.
    // To get a specific color's pieces, use the AND of color_pieces and type_pieces,
    // i.e to get the white king: `white_pieces & king_pieces`.
    king_pieces: Bitboard,
    queen_pieces: Bitboard,
    rook_pieces: Bitboard,
    knight_pieces: Bitboard,
    bishop_pieces: Bitboard,
    pawn_pieces: Bitboard,

    // Other board state stuff
    active_color: PlayerColor,
    castling_availability: CastlingAvailability,
    en_passant_target_square: Option<Square>,
    halfmove_clock: u8,
    fullmove_number: u8,
}

impl Board
{
    /// Creates a new board with the "standard" configuration seen in classical chess.
    ///
    /// Creates and returns a new Board with the default arrangement and number of pieces, 16
    /// pawns, 8 per side, 4 knights, 4 rooks, etc.
    pub fn new_default_starting_board() -> Self
    {
        // Not sure if the board should rely on the default constructor for the
        // BoardConfiguration?
        // 
        // This is probably correct and valid?
        Self::new_board_with_configuration(&BoardConfiguration::default())
    }

    /// Creates a new empty board.
    ///
    /// This board has no pieces on it.
    pub fn new_blank_board() -> Self
    {
        // Unlike BoardConfig, a default BoardConfigurationBuilder is all blank.
        let board_config_builder = BoardConfigurationBuilder::default();
        let board_config = board_config_builder.build();
        Self::new_board_with_configuration(&board_config)
    }

    /// Allows creation of a new board with a custom configuration.
    ///
    /// Takes a [BoardConfiguration] which represents the desired starting state of the board.
    pub fn new_board_with_configuration(board_configuration: &BoardConfiguration) -> Self
    {
        let mut new_board = Self {
            active_color: board_configuration.active_color(),
            castling_availability: board_configuration.castling_availability(),
            en_passant_target_square: board_configuration.en_passant_target_square(),
            halfmove_clock: board_configuration.halfmove_clock(),
            fullmove_number: board_configuration.fullmove_number(),
            // Set remaining defaults. These will be set programatically below.
            piece_mailbox: HashMap::new(),
            white_pieces: Bitboard::default(),
            black_pieces: Bitboard::default(),
            king_pieces: Bitboard::default(),
            queen_pieces: Bitboard::default(),
            rook_pieces: Bitboard::default(),
            knight_pieces: Bitboard::default(),
            bishop_pieces: Bitboard::default(),
            pawn_pieces: Bitboard::default()
        };

        // Add all pieces to the board, setting the state of all the bitboards accordingly
        for (square, piece) in board_configuration.get_pieces().iter()
        {
            new_board.add_piece(*piece, square);
        }

        new_board
    }

    /// Gets the board configuration associated with the current board state.
    pub fn board_configuration(&self) -> BoardConfiguration
    {
        BoardConfiguration::new(
            self.piece_mailbox.clone(),
            self.active_color,
            self.castling_availability,
            self.en_passant_target_square,
            self.halfmove_clock,
            self.fullmove_number
        )
    }

    /// Returns the current result of the board as a [BoardResult].
    ///
    /// This contains information about if the game is over (and who won) or if the game is still
    /// in progress.
    pub fn game_result(&self) -> BoardResult
    {
        // Draw after 50 moves without a pawn push or capture.
        if self.halfmove_clock >= 50
        {
            return BoardResult::Draw(DrawReason::FiftyMoveRule);
        }

        // Other things we should check:
        // - Threefold repitition. This is probably beyond the scope of a board,
        //   since a board doesn't have any information about past moves.
        // - Checkmate impossible. A game is drawn when neither player has sufficient material
        //   to checkmate the other king. We shouild check that.
        // - Draw by agreement? Can agents agree to a draw?
        //
        // For now we're only covering:
        // - FiftyMoveRule. A draw is automatic when it has been 50 moves without a pawn push
        //   or capture.
        // - Stalemate. 
        // - Checkmate.

        match (self.is_king_in_check(self.active_color), self.generate_moves_for_side(self.active_color).is_empty())
        {
            // King is in check *and* the player has no valid moves
            // Then the *other* player wins.
            (true, true) => BoardResult::Win(!self.active_color),
            // King is NOT in check, but the player has no valid moves.
            (false, true) => BoardResult::Draw(DrawReason::Stalemate),
            // The player can still move, the game is not over yet.
            (_, false) => BoardResult::InProgress,
        }
    }

    /// Attempts to make a move on the board, returning a new board state wrapped in [Ok] if
    /// successful, or a [MoveError] if the move was invalid for whatever reason.
    ///
    /// Does not modify the board, but instead returns a new board with the move made.
    pub fn attempt_move(&self, attempted_move: &MoveCommand) -> Result<Self, MoveError>
    {
        let r#move = self.get_move(attempted_move)?;
        // If the move is illegal, we abort.
        if !self.check_move(&r#move)
        {
            return Err(MoveError::IllegalMove);
        }

        // Otherwise, we make the move!
        Ok(self.make_move(&r#move))
    }

    /// Gets the piece located on a given square.
    ///
    /// If the square is currently empty, this function returns [None].
    /// If the square is occupied, this function returns a reference to the [Piece].
    pub fn piece_at(&self, square: &Square) -> Option<&Piece>
    {
        self.piece_mailbox.get(square)
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
    fn get_move(&self, move_command: &MoveCommand) -> Result<Move, MoveError>
    {
        match move_command
        {
            MoveCommand::KingsideCastle => Ok(Move::Castle(CastlingDirection::Kingside)),
            MoveCommand::QueensideCastle => Ok(Move::Castle(CastlingDirection::Queenside)),
            MoveCommand::NormalMove(data) => self.parse_normal_move(data),
        }
    }

    /// Checks whether or not a move is legal. Because we consume a valid [Move], we know that the
    /// move is possible. Technically you could pass in a [Move] generated by a different board but
    /// that's considered a logic error. The [Move] should ALWAYS be obtained by *this* board's
    /// [Self::get_move] function. This function is basically just additional validation to check
    /// whether or not a move is allowed, following additional situational rules like check and
    /// stuff.
    ///
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
    fn check_move(&self, attempted_move: &Move) -> bool 
    {
        // 1. Check if the move would either put the player's king in check or leave the player's
        //    king in check on the next board.
        if self.move_leaves_king_in_check(attempted_move)
        {
            return false;
        }

        // 2. Check if the move is a castle. if the move IS a castle, does it move the king through
        //    check?
        match attempted_move
        {
            Move::Castle(CastlingDirection::Kingside) => 
            {
                if self.kingside_castle_moves_through_check(self.active_color)
                {
                    return false;
                }
            },
            Move::Castle(CastlingDirection::Queenside) =>
            {
                if self.queenside_castle_moves_through_check(self.active_color)
                {
                    return false;
                }
            },
            _ => (),
        }

        // I don't think there's any other situations where a move is possible but illegal? 
        // I think it's all about check?
        return true;
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
    fn make_move(&self, r#move: &Move) -> Self
    {
        let mut new_board = self.clone();
        new_board.make_move_in_place(r#move);
        return new_board;
    }

    /// The in-place version of [Board::make_move]. 
    ///
    /// Instead of returning a new board, this function modifies it in place, altering the board
    /// state as necessary to represent a valid configuration.
    fn make_move_in_place(&mut self, r#move: &Move)
    {
        match r#move
        {
            Move::Castle(direction) => {
                // We hard code the castling squares. At some point we may want to not do this, but
                // idk what that looks like at allll.
                let rank = match self.active_color
                {
                    PlayerColor::White => 0,
                    PlayerColor::Black => 7,
                };
                let (king_to_file, rook_from_file, rook_to_file) = match direction
                {
                    CastlingDirection::Kingside => (6, 7, 5),
                    CastlingDirection::Queenside => (2, 0, 3),
                };
                let king = self.remove_piece(&Square::new(rank, 4)).expect("Expected king to be on starting square for castling.");
                let rook = self.remove_piece(&Square::new(rank, rook_from_file)).expect("Expected rook to be on starting square for castling.");
                self.add_piece(king, &Square::new(rank, king_to_file));
                self.add_piece(rook, &Square::new(rank, rook_to_file));

                // Castles count as non-captures/pawn moves, so we increment the halfmove clock
                self.halfmove_clock += 1;

            }
            Move::NormalMove(move_data) => {
                let piece = self.remove_piece(&move_data.starting_square());
                let piece = piece.expect("There was no piece at the starting square!");
                if move_data.is_capture()
                {
                    self.remove_piece(&move_data.target_square());
                }
                self.add_piece(piece, &move_data.target_square());

                // Check to see if we increment the halfmove_clock.
                match (piece.piece_type(), move_data.is_capture())
                {
                    // If we are moving a pawn we reset the halfmove clock
                    (PieceType::Pawn, _) => { self.halfmove_clock = 0 },
                    // If we are capturing we reset the halfmove clock
                    (_, true) => { self.halfmove_clock = 0 },
                    // Otherwise we increment the halfmove clock.
                    _ => { self.halfmove_clock += 1 },
                }
            },
        };
        // Increment fullmove number
        match self.active_color
        {
            PlayerColor::Black => { self.fullmove_number += 1; }
            PlayerColor::White => (),
        };
        // We also want to disable future castling for that player.
        self.castling_availability.update_with_move(self.active_color, r#move);
        // Switch to next player
        self.active_color = !self.active_color;
    }

    /// Adds a piece onto the board in the set position.
    ///
    /// This function modifies the current board and sets the given square to be the provided
    /// piece. This doesn't do any checks to see if the provided square is already occupied, and
    /// instead just forcibly overwrites the given location. This also doesn't move the given piece
    /// from its old location, so this function is intended to be used in conjunction with and
    /// after [Self::remove_piece].
    ///
    /// This also means that it doesn't check the state of the bitboards to ensure the state is
    /// valid (i.e not more than one piece in a square.)
    fn add_piece(&mut self, piece: Piece, position: &Square)
    {
        self.piece_mailbox.insert(*position, piece);
        let add_bitmask = Bitboard::from(*position);
        *self.pieces_of_color_as_mut(piece.color()) |= add_bitmask;
        *self.pieces_of_type_as_mut(piece.piece_type()) |= add_bitmask;
    }

    /// Erases a piece from the board.
    /// 
    /// This function modifies the current board and removes whatever piece, if any, exists on the
    /// board in that location. This doesn't do any checks to see what, if any, piece is there, it
    /// simply removes all references to a piece on that square. This can be the result of a
    /// capture, where the captured piece is removed from its square before it is replaced, or
    /// simply a move where there is no longer a piece on the starting square.
    ///
    /// This function also returns the removed piece from the piece_mailbox so it can be placed
    /// again somewhere else or you can do stuff with it. If there was no piece there,
    /// it simply returns [None]
    fn remove_piece(&mut self, position: &Square) -> Option<Piece>
    {
        let piece = self.piece_mailbox.remove(position);
        let remove_bitmask = !Bitboard::from(*position);
        self.white_pieces &= remove_bitmask;
        self.black_pieces &= remove_bitmask;
        self.pawn_pieces &= remove_bitmask;
        self.knight_pieces &= remove_bitmask;
        self.bishop_pieces &= remove_bitmask;
        self.rook_pieces &= remove_bitmask;
        self.queen_pieces &= remove_bitmask;
        self.king_pieces &= remove_bitmask;
        piece
    }

    /// Returns true if the king is in check *on* the current board state.
    ///
    /// # Arguments
    ///
    /// * `king_color` - The color of the king to check for.
    ///
    /// # Examples
    ///
    /// ```
    /// [TODO:write some example code]
    /// ```
    fn is_king_in_check(&self, king_color: PlayerColor) -> bool
    {
        let king = self.query().color(king_color).piece_type(PieceType::King).result();
        let king_square: Vec<Square> = king.squares().collect();
        // This should always be true?
        assert!(king_square.len() == 1);
        let king_square = king_square[0];
        // Check each piece type to see if any pieces are attacking the king's square.
        let pawn_attacks = self.squares_of_type_that_can_capture_square(!king_color, PieceType::Pawn, king_square);
        if pawn_attacks.len() > 0
        {
            return true;
        }
        let knight_attacks = self.squares_of_type_that_can_capture_square(!king_color, PieceType::Knight, king_square);
        if knight_attacks.len() > 0
        {
            return true;
        }

        let bishop_attacks = self.squares_of_type_that_can_capture_square(!king_color, PieceType::Bishop, king_square);
        if bishop_attacks.len() > 0
        {
            return true;
        }
        let rook_attacks = self.squares_of_type_that_can_capture_square(!king_color, PieceType::Rook, king_square);
        if rook_attacks.len() > 0
        {
            return true;
        }
        let queen_attacks = self.squares_of_type_that_can_capture_square(!king_color, PieceType::Queen, king_square);
        if queen_attacks.len() > 0
        {
            return true;
        }
        let king_attacks = self.squares_of_type_that_can_capture_square(!king_color, PieceType::King, king_square);
        if king_attacks.len() > 0
        {
            return true;
        }

        // We shouldn't have to check for king attacks???
        return false;
    }
}

impl Display for Board
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Formats the board to look as follows:
        /*    a  b  c  d  e  f  g  h 
         * 8 [r][n][b][q][k][b][n][r]
         * 7 [p][p][p][p][p][p][p][p]
         * 6 [ ][ ][ ][ ][ ][ ][ ][ ]
         * 5 [ ][ ][ ][ ][ ][ ][ ][ ]
         * 4 [ ][ ][ ][ ][ ][ ][ ][ ]
         * 3 [ ][ ][ ][ ][ ][ ][ ][ ]
         * 2 [P][P][P][P][P][P][P][P]
         * 1 [R][N][B][Q][K][B][N][R]
         */
        // Initial spacing for top left corner
        // Two spaces.
        write!(f, "  ")?;
        let spacing = 3;
        // We map 0-8 to a-h for writing the files.
        for char in (0..8).map(|x| char::from_u32(x + 'a' as u32).expect(&format!("{} was not a valid character", x + 'a' as u32)))
        {
            // Writes the file with a spacing of 3, with the file letter centered.
            write!(f, "{:^spacing$}", char, spacing=spacing)?;
        }
        write!(f, "\n")?;
        // Now we can write the ranks, yay!!!
        for rank in (0..8).rev()
        {
            // Prefix the line with the rank coordinates
            write!(f, "{:<2}", rank + 1)?;
            for file in 0..8
            {
                // Visit each square and print what piece is on that square, if any.
                let square = Square::new(rank, file);
                let piece = self.piece_mailbox.get(&square);
                match piece
                {
                    Some(x) => write!(f, "[{}]", x)?,
                    None => write!(f, "[ ]")?,
                };
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests
{
    use std::str::FromStr;

    use crate::{board::{BoardConfiguration, Piece, PieceType, PlayerColor, Square}, hashmap_diff::print_hashmap_differences, parse::MoveCommand};

    use super::Board;

    #[test]
    fn test_making_legal_move_works()
    {
        let board = Board::new_default_starting_board();
        let move_command = MoveCommand::from_str("e4").unwrap();
        let new_board = board.attempt_move(&move_command).unwrap();
        let expected_new_board_config = BoardConfiguration::from_str("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1").unwrap();
        assert_eq!(expected_new_board_config, new_board.board_configuration());
    }

    #[test]
    fn test_making_impossible_move_fails()
    {
        let board = Board::new_default_starting_board();
        let move_command = MoveCommand::from_str("Qe3").unwrap();
        let new_board = board.attempt_move(&move_command);
        assert!(new_board.is_err());
    }

    #[test]
    fn test_moving_through_occupancy_fails()
    {
        let board = Board::new_default_starting_board();
        let move_command = MoveCommand::from_str("Qd3").unwrap();
        let new_board = board.attempt_move(&move_command);
        assert!(new_board.is_err());
    }

    #[test]
    fn test_moving_while_in_check_fails()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("q6k/8/8/3B4/8/8/8/K7 w - - 0 1").unwrap());
        let move_command = MoveCommand::from_str("Bc4").unwrap();
        let new_board = board.attempt_move(&move_command);
        assert!(new_board.is_err());
    }

    #[test]
    fn test_moving_out_of_check_succeeds()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("q6k/8/8/3B4/8/8/8/K7 w - - 0 1").unwrap());
        let move_command = MoveCommand::from_str("Kb2").unwrap();
        let new_board = board.attempt_move(&move_command);
        assert!(new_board.is_ok());
    }

    #[test]
    fn test_blocking_check_works()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("q6k/8/8/3B4/8/8/8/K7 w - - 0 1").unwrap());
        let move_command = MoveCommand::from_str("Ba2").unwrap();
        let new_board = board.attempt_move(&move_command);
        assert!(new_board.is_ok());
    }

    #[test]
    fn test_capturing_checking_piece_works()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("q6k/8/8/3B4/8/8/8/K7 w - - 0 1").unwrap());
        let move_command = MoveCommand::from_str("Bxa8").unwrap();
        let new_board = board.attempt_move(&move_command);
        assert!(new_board.is_ok());
    }

    #[test]
    fn test_clone_configuration_identical()
    {
        let board = Board::new_default_starting_board();
        let new_board = board.clone();
        assert_eq!(board.board_configuration(), new_board.board_configuration());
    }

    #[test]
    fn test_clone_identical_for_custom_board_config()
    {
        let board_config = BoardConfiguration::from_str("q7/8/8/8/8/8/1Q6/1K6 w - - 0 1").unwrap();
        let board = Board::new_board_with_configuration(&board_config);
        let new_board = board.clone();
        assert_eq!(board_config, new_board.board_configuration());
    }

    #[test]
    fn initialize_board_in_progress()
    {
        let board = Board::new_default_starting_board();
        assert!(board.game_result().is_in_progress());
    }

    #[test]
    fn initialize_board_in_end_position()
    {
        let board_config= BoardConfiguration::from_str("3k4/3Q4/3K4/8/8/8/8/8 b - - 0 1").unwrap();
        let board = Board::new_board_with_configuration(&board_config);
        assert!(board.game_result().is_over());
        assert!(!board.game_result().is_draw());
        assert_eq!(PlayerColor::White, board.game_result().get_winner().unwrap());
    }

    #[test]
    fn initialize_board_move_into_end_position()
    {
        let board_config = BoardConfiguration::from_str("3k4/8/3K4/8/Q7/8/8/8 w - - 0 1").unwrap();
        let board = Board::new_board_with_configuration(&board_config);
        let r#move = MoveCommand::from_str("Qd7").unwrap();
        let new_board = board.attempt_move(&r#move).unwrap();
        assert!(new_board.game_result().is_over());
        assert!(!new_board.game_result().is_draw());
        assert_eq!(PlayerColor::White, new_board.game_result().get_winner().unwrap());
    }

    #[test]
    fn adding_piece_to_board_works()
    {
        let mut board = Board::new_blank_board();
        let square = Square::new(0, 0);
        let new_piece = Piece::new(PlayerColor::White, PieceType::Pawn);
        assert!(board.piece_at(&square).is_none());
        board.add_piece(new_piece, &square);
        assert!(board.piece_at(&square).is_some());
        assert_eq!(new_piece, *board.piece_at(&square).unwrap());
    }

    #[test]
    fn removing_piece_from_board_works()
    {
        let mut board = Board::new_default_starting_board();
        let square = Square::new(0, 0);
        assert!(board.piece_at(&square).is_some());
        board.remove_piece(&square);
        assert!(board.piece_at(&square).is_none());
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
            let result = board.piece_at(&Square::from_str(square_str).unwrap());
            assert!(result.is_some());
            assert_eq!(piece, *result.unwrap());
        }
    }

    #[test]
    fn display_default_starting_board_string()
    {
        let default_board_str = 
        "   a  b  c  d  e  f  g  h \n\
         8 [r][n][b][q][k][b][n][r]\n\
         7 [p][p][p][p][p][p][p][p]\n\
         6 [ ][ ][ ][ ][ ][ ][ ][ ]\n\
         5 [ ][ ][ ][ ][ ][ ][ ][ ]\n\
         4 [ ][ ][ ][ ][ ][ ][ ][ ]\n\
         3 [ ][ ][ ][ ][ ][ ][ ][ ]\n\
         2 [P][P][P][P][P][P][P][P]\n\
         1 [R][N][B][Q][K][B][N][R]\n";
        let board = Board::new_default_starting_board();

        assert_eq!(default_board_str, format!("{}", board));
    }

    #[test]
    fn test_white_castle_kingside()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4").unwrap());
        let castle_move = MoveCommand::from_str("O-O").unwrap();
        let new_board = board.attempt_move(&castle_move).unwrap();
        let expected_board_configuration = BoardConfiguration::from_str("r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 5 4").unwrap();
        print_hashmap_differences(new_board.board_configuration().pieces(), expected_board_configuration.pieces());
        assert_eq!(new_board.board_configuration(),expected_board_configuration);
        assert_eq!(new_board.piece_at(&Square::new(0, 5)).unwrap().piece_type(), PieceType::Rook);
        assert_eq!(new_board.piece_at(&Square::new(0, 6)).unwrap().piece_type(), PieceType::King);
    }

    #[test]
    fn test_black_castle_kingside()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQ1RK1 b kq - 0 5").unwrap());
        let castle_move = MoveCommand::from_str("O-O").unwrap();
        let new_board = board.attempt_move(&castle_move).unwrap();
        assert_eq!(new_board.board_configuration(), BoardConfiguration::from_str("r1bq1rk1/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQ1RK1 w - - 1 6").unwrap());
        assert_eq!(new_board.piece_at(&Square::new(7, 5)).unwrap().piece_type(), PieceType::Rook);
        assert_eq!(new_board.piece_at(&Square::new(7, 6)).unwrap().piece_type(), PieceType::King);
    }

    #[test]
    fn test_white_castle_queenside()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("rnb1k2r/pppqbppp/4pn2/3p4/3P1B2/2P1P3/PPQN1PPP/R3KBNR w KQkq - 5 7").unwrap());
        let castle_move = MoveCommand::from_str("O-O-O").unwrap();
        let new_board = board.attempt_move(&castle_move).unwrap();
        assert_eq!(new_board.board_configuration(), BoardConfiguration::from_str("rnb1k2r/pppqbppp/4pn2/3p4/3P1B2/2P1P3/PPQN1PPP/2KR1BNR b kq - 6 7").unwrap());
        assert_eq!(new_board.piece_at(&Square::new(0, 3)).unwrap().piece_type(), PieceType::Rook);
        assert_eq!(new_board.piece_at(&Square::new(0, 2)).unwrap().piece_type(), PieceType::King);
    }

    #[test]
    fn test_black_castle_queenside()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("r3k2r/pp1bbppp/2nqpn2/2ppN3/3P4/2PBP3/PPQN1PPP/2KR3R b kq - 3 11").unwrap());
        let castle_move = MoveCommand::from_str("O-O-O").unwrap();
        let new_board = board.attempt_move(&castle_move).unwrap();
        assert_eq!(new_board.board_configuration(), BoardConfiguration::from_str("2kr3r/pp1bbppp/2nqpn2/2ppN3/3P4/2PBP3/PPQN1PPP/2KR3R w - - 4 12").unwrap());
        assert_eq!(new_board.piece_at(&Square::new(7, 3)).unwrap().piece_type(), PieceType::Rook);
        assert_eq!(new_board.piece_at(&Square::new(7, 2)).unwrap().piece_type(), PieceType::King);
    }

    #[test]
    fn white_move_increments_halfmove_clock_but_not_fullmove_number()
    {
        let board = Board::new_default_starting_board();
        let r#move = MoveCommand::from_str("Nc3").unwrap();
        let new_board = board.attempt_move(&r#move).unwrap();
        assert_eq!(1, new_board.board_configuration().halfmove_clock());
        assert_eq!(1, new_board.board_configuration().fullmove_number());
    }

    #[test]
    fn black_move_increment_halfmove_clock_and_fullmove_number()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b KQkq - 1 1").unwrap());
        let r#move = MoveCommand::from_str("Nf6").unwrap();
        let new_board = board.attempt_move(&r#move).unwrap();
        assert_eq!(2, new_board.board_configuration().halfmove_clock());
        assert_eq!(2, new_board.board_configuration().fullmove_number());
    }

    #[test]
    fn pawn_move_resets_halfmove_clock()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("rnbqkb1r/pppppppp/5n2/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 2 2").unwrap());
        let r#move = MoveCommand::from_str("e4").unwrap();
        let new_board = board.attempt_move(&r#move).unwrap();
        assert_eq!(0, new_board.board_configuration().halfmove_clock());
    }

    #[test]
    fn capture_resets_halfmove_clock()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("rnbqkb1r/pppppppp/5n2/8/4N3/8/PPPPPPPP/R1BQKBNR b KQkq - 3 2").unwrap());
        let r#move = MoveCommand::from_str("Nxe4").unwrap();
        let new_board = board.attempt_move(&r#move).unwrap();
        assert_eq!(0, new_board.board_configuration().halfmove_clock());
    }
}
