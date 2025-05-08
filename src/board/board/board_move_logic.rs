//! This is a helper module for [Board](super::Board) that handles interpreting
//! [MoveCommand](crate::parse::MoveCommand) and just figuring out how to handle a given move.
//!
//! This is all private to [Board](super::Board), it's just meant to clean up the module a bit.

use crate::{bitboard::Bitboard, board::{r#move::MoveData, piece_type::PIECE_TYPES, Move, MoveError, PieceType, PlayerColor, Square}, parse::MoveCommandData};

use super::Board;

impl Board
{
    /// Helper function for [Self::get_move].
    ///
    /// Converts a [MoveCommandData] into a [Move]. 
    ///
    /// # Arguments
    ///
    /// * `move_data` - The [MoveCommandData] to process.
    ///
    /// # Errors
    ///
    /// Returns a [MoveError] if there are 
    /// 1) No moves match the given [MoveCommandData].
    /// 2) There are too many matches and the given discriminant (if there is one) is insufficient
    ///    to narrow the available moves down to one.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{Board, Move, Square};
    /// # use rust_chess_engine::parse::MoveCommand;
    /// # use std::str::FromStr;
    /// let board = Board::new_default_starting_board();
    /// let move_command = MoveCommand::from_str("e4").unwrap();
    /// let MoveCommand::NormalMove(move_data) = move_command else { unreachable!() };
    /// let r#move = board.parse_normal_move(&move_data).unwrap();
    /// let Move::NormalMove(r#move) = r#move else { unreachable!() };
    /// assert_eq!(r#move.starting_square(), Square::new(1, 4));
    /// assert_eq!(r#move.capture(), false);
    /// assert_eq!(r#move.target_square(), Square::new(3, 4));
    /// ```
    pub fn parse_normal_move(&self, move_data: &MoveCommandData) -> Result<Move, MoveError>
    {
        let starting_squares = match move_data.capture()
        {
            false => self.squares_of_type_that_can_move_to_square(self.active_color, move_data.piece_type(), move_data.target_square()),
            true => self.squares_of_type_that_can_capture_square(self.active_color, move_data.piece_type(), move_data.target_square()),
        };

        match (starting_squares.len(), move_data.discriminant())
        {
            (0, _) => Err(MoveError::NoPossibleMove),
            (1, _) => Ok(Move::NormalMove(
                    MoveData::from_move_command_data(&move_data, starting_squares[0])
            )),
            // Cover having too many pieces!
            (2.., Some(discriminant)) => {
                // If we do have a discriminant, try and use it to further filter down which square
                // we are moving from.
                let starting_squares: Vec<&Square> = starting_squares.iter().filter(|square| discriminant.has_square(square)).collect();
                match starting_squares.len()
                {
                    0 => Err(MoveError::NoPossibleMove),
                    1 => Ok(Move::NormalMove(
                            MoveData::from_move_command_data(&move_data, *starting_squares[0])
                    )),
                    _ => Err(MoveError::TooManyMoves),
                }
            },
            (2.., None) => Err(MoveError::TooManyMoves)
        }
    }

    /// Gets all of the pieces of a type that can move to a given square.
    ///
    /// Since we know from a [MoveCommand] what piece is being moved and where it is being moved
    /// to, but not where it is being moved from, we have to return a list of all pieces to be
    /// filtered down according to a discriminant if there is more than one.
    ///
    /// This function actually returns the SQUARES of all valid pieces that can move to that
    /// square, given they are of the right type and all.
    ///
    /// If you want to see which pieces can *capture* a square, use
    /// [Self::squares_of_type_that_can_capture_square], which is functionally identical for all
    /// pieces except pawns which have a disjoint capture/move set.
    ///
    ///
    /// # Arguments
    ///
    /// * `piece_color` - The piece color to search for.
    /// * `piece_type` - The piece type to search for
    /// * `square` - The target square that is being moved to.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{Board, PlayerColor, PieceType, Square};
    /// # use std::str::FromStr;
    /// let board = Board::new_default_starting_board();
    /// let squares = board.squares_of_type_that_can_move_to_square(
    ///     PlayerColor::White, PieceType::Pawn, Square::from_str("e4").unwrap()
    /// );
    /// // There should be one piece that can move to the specified square.
    /// assert_eq!(squares.len(), 1);
    /// assert!(squares.contains(&Square::from_str("e2").unwrap()));
    /// ```
    pub fn squares_of_type_that_can_move_to_square(&self, piece_color: PlayerColor, piece_type: PieceType, square: Square) -> Vec<Square>
    {
        let piece_map = self.pieces_of_type(piece_type) & self.pieces_of_color(piece_color);
        let target_square_bitboard: Bitboard = square.into();

        // We pick one of the move types depending on what type of piece this is.
        let move_type: fn(&Board, PlayerColor, Square) -> Bitboard = match piece_type
        {
            PieceType::Pawn => Self::pawn_moves,
            PieceType::Knight => Self::knight_moves,
            PieceType::Bishop => Self::bishop_moves,
            PieceType::Rook => Self::rook_moves,
            PieceType::Queen => Self::queen_moves,
            PieceType::King => Self::king_moves
        };
        // Then we filter the moves down to only those pieces which can REACH the target square.
        // This could be no pieces, one piece (correct), or two pieces (needs to be filtered down
        // by discriminant.
        piece_map.squares().filter(
            |starting_square| !((move_type(&self, self.active_color, *starting_square) & target_square_bitboard).is_empty())
        ).collect()
    }

    /// This method is the same as [Self::squares_of_type_that_can_move_to_square], but checks for
    /// pieces/[Square]s that can *capture* the given square, not move to it. The only meaningful
    /// distinction here is for pawns, which have disjoint capture vs move sets.
    ///
    /// # Arguments
    ///
    /// * `piece_color` - The color of the capturing piece.
    /// * `piece_type` - The type of the capturing piece.
    /// * `square` - The target square being captured *on*.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{Board, BoardConfiguration, PlayerColor, PieceType, Square};
    /// # use std::str::FromStr;
    /// // Default board but with a black pawn on b3
    /// let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("rnbqkbnr/pppppppp/8/8/8/1p6/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap());
    /// let squares = board.squares_of_type_that_can_capture_square(
    ///     PlayerColor::White, PieceType::Pawn, Square::from_str("b3").unwrap()
    /// );
    /// // There should be two pawns that can capture on b3.
    /// assert_eq!(squares.len(), 2);
    /// assert!(squares.contains(&Square::from_str("a2").unwrap()));
    /// assert!(squares.contains(&Square::from_str("c2").unwrap()));
    /// ```
    pub fn squares_of_type_that_can_capture_square(&self, piece_color: PlayerColor, piece_type: PieceType, target_square: Square) -> Vec<Square>
    {
        let piece_map = self.pieces_of_type(piece_type) & self.pieces_of_color(piece_color);
        let target_square_bitboard: Bitboard = target_square.into();

        // We pick one of the move types depending on what type of piece this is.
        let move_type: fn(&Board, PlayerColor, Square) -> Bitboard = match piece_type
        {
            PieceType::Pawn => Self::pawn_attacks,
            PieceType::Knight => Self::knight_moves,
            PieceType::Bishop => Self::bishop_moves,
            PieceType::Rook => Self::rook_moves,
            PieceType::Queen => Self::queen_moves,
            PieceType::King => Self::king_moves
        };
        // Then we filter the moves down to only those pieces which can REACH the target square.
        // This could be no pieces, one piece (correct), or two pieces (needs to be filtered down
        // by discriminant.

        piece_map.squares().filter(
            |start_square| !((move_type(&self, piece_color, *start_square) & target_square_bitboard).is_empty())
        ).collect()
    }

    /// Similar to [Self::squares_of_type_that_can_capture_square] but instead of returning pieces
    /// of a specific type this returns *all* pieces. 
    ///
    /// # Arguments
    ///
    /// * `attacking_color` - The attacking color
    /// * `target_square` - The square to check for being targeted
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{Board, Square, PlayerColor, BoardConfiguration};
    /// # use std::str::FromStr;
    /// // Default board but with a black pawn on c3
    /// let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("rnbqkbnr/pppppppp/8/8/8/2p5/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap());
    /// let squares = board.all_squares_that_can_capture_square(
    ///     PlayerColor::White, Square::from_str("c3").unwrap()
    /// );
    /// println!("{:?}", squares);
    /// assert_eq!(squares.len(), 3);
    /// assert!(squares.contains(&Square::from_str("b2").unwrap()));
    /// assert!(squares.contains(&Square::from_str("b1").unwrap()));
    /// assert!(squares.contains(&Square::from_str("d2").unwrap()));
    /// ```
    pub fn all_squares_that_can_capture_square(&self, attacking_color: PlayerColor, target_square: Square) -> Vec<Square>
    {
        // Check every single attacking piece type to see if it can attack the target square lmao.
        PIECE_TYPES.iter().map(
            |piece_type| self.squares_of_type_that_can_capture_square(attacking_color, *piece_type, target_square)
        ).collect::<Vec<Vec<Square>>>().concat()
    }
}

#[cfg(test)]
mod tests
{
    use std::str::FromStr;

    use crate::board::BoardConfiguration;

    use super::*;

    #[test]
    fn check_queen_can_reach_king_square()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("k7/Q7/K7/8/8/8/8/8 b - - 0 1").unwrap());
        let target_square = Square::new(7, 0);
        let queen_attacks_square = board.squares_of_type_that_can_capture_square(PlayerColor::White, PieceType::Queen, target_square);
        assert_eq!(queen_attacks_square.len(), 1);
        assert_eq!(queen_attacks_square[0], Square::new(6, 0));
    }

    #[test]
    fn check_knight_can_reach_king_square()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("k7/2N5/4p3/1K6/8/8/8/8 b - - 0 1").unwrap());
        let target_square = Square::new(7, 0);
        let knight_attacks_square = board.squares_of_type_that_can_capture_square(PlayerColor::White, PieceType::Knight, target_square);
        assert_eq!(knight_attacks_square.len(), 1);
        assert_eq!(knight_attacks_square[0], Square::new(6, 2));
    }

    #[test]
    fn check_all_pieces_that_can_reach_target_square()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("k6Q/2N5/8/8/4B3/2K5/R7/8 b - - 0 1").unwrap());
        let target_square = Square::new(7, 0);
        let all_squares = board.all_squares_that_can_capture_square(PlayerColor::White, target_square);
        assert_eq!(all_squares.len(), 4);
        assert!(all_squares.contains(&Square::new(7, 7)));
        assert!(all_squares.contains(&Square::new(6, 2)));
        assert!(all_squares.contains(&Square::new(1, 0)));
        assert!(all_squares.contains(&Square::new(3, 4)));
    }

    #[test]
    fn check_only_some_pieces_can_reach_target_square()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("7k/8/1K6/8/2BB4/8/8/8 w - - 0 1").unwrap());
        let target_square = Square::new(7, 7);
        let bishop_attacks_square = board.squares_of_type_that_can_capture_square(PlayerColor::White, PieceType::Bishop, target_square);
        assert_eq!(bishop_attacks_square.len(), 1);
        assert!(bishop_attacks_square.contains(&Square::new(3, 3)));
        assert!(!bishop_attacks_square.contains(&Square::new(3, 2)));
    }

    #[test]
    fn check_pawn_can_move_to_square()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("7k/8/8/8/8/8/P7/7K w - - 0 1").unwrap());
        let target_square = Square::new(3, 0);
        let pawn_move_squares = board.squares_of_type_that_can_move_to_square(PlayerColor::White, PieceType::Pawn, target_square);
        let pawn_attack_squares = board.squares_of_type_that_can_capture_square(PlayerColor::White, PieceType::Pawn, target_square);
        assert_eq!(pawn_move_squares.len(), 1);
        assert!(pawn_move_squares.contains(&Square::new(1, 0)));
        assert_eq!(pawn_attack_squares.len(), 0);
    }

    #[test]
    fn check_pawn_can_capture_square()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("7k/8/8/8/8/1p6/P7/7K w - - 0 1").unwrap());
        let target_square = Square::new(2, 1);
        let pawn_move_squares = board.squares_of_type_that_can_move_to_square(PlayerColor::White, PieceType::Pawn, target_square);
        let pawn_attack_squares = board.squares_of_type_that_can_capture_square(PlayerColor::White, PieceType::Pawn, target_square);
        assert_eq!(pawn_move_squares.len(), 0);
        assert_eq!(pawn_attack_squares.len(), 1);
        assert!(pawn_attack_squares.contains(&Square::new(1, 0)));
    }

}
