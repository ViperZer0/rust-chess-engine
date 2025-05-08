//! This is a helper module for [board](super) that handles checking whether a given [Move]
//! is legal or not.

use crate::board::{r#move::CastlingDirection, Move, PlayerColor, Square};

use super::Board;

impl Board
{
    /// Checks if a given [Move] would leave the current player's king in check after it.
    /// 
    /// This covers both leaving a king in check (bad) and putting a king into check (bad).
    ///
    /// Returns true if the move is illegal/ends up putting the king in check, false otherwise.
    ///
    /// # Arguments
    ///
    /// * `r#move` - The attempted move.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{Board, Move, MoveData, Square};
    /// let board = Board::new_default_starting_board();
    /// let r#move = Move::NormalMove(MoveData::new(Square::new(1,4), Square::new(3,4), false));
    /// assert!(!board.move_leaves_king_in_check(&r#move));
    /// ```
    pub fn move_leaves_king_in_check(&self, r#move: &Move) -> bool
    {
        // Attempt to make the move on the board and see if the king would be in check.
        // If so, returns true. Otherwise returns false.
        let theoretical_next_board = self.make_move(r#move);
        // The active color here is the player on the CURRENT board, not the next board.
        // If white moves, we check to see if their king is still in check on the next board.
        theoretical_next_board.is_king_in_check(self.active_color)
    }

    /// Checks whether the king moves through check if it kingside castled right now.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{Board, PlayerColor};
    /// let board = Board::new_default_starting_board();
    /// // Even though white can't acutally castle here,
    /// // we can still check to see if castling *would* move through check.
    /// assert!(!board.kingside_castle_moves_through_check(PlayerColor::White));
    /// ```
    pub fn kingside_castle_moves_through_check(&self, moving_color: PlayerColor) -> bool
    {
        let rank = match moving_color 
        {
            PlayerColor::White => 0,
            PlayerColor::Black => 7,
        };
        let castling_squares = vec![Square::new(rank, 4), Square::new(rank, 5), Square::new(rank, 6)];
        self.check_squares_for_attack(&castling_squares)
    }

    /// Checks whether the king would move through check if it queenside castled right now.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{Board, PlayerColor};
    /// let board = Board::new_default_starting_board();
    /// // Even though white can't actually castle here,
    /// // we can still check to see if castling *would* move through check.
    /// assert!(!board.queenside_castle_moves_through_check(PlayerColor::White));
    /// ```
    pub fn queenside_castle_moves_through_check(&self, moving_color: PlayerColor) -> bool
    {
        let rank = match moving_color
        {
            PlayerColor::White => 0,
            PlayerColor::Black => 7,
        };
        let castling_squares = vec![Square::new(rank, 4), Square::new(rank, 3), Square::new(rank, 2)];
        self.check_squares_for_attack(&castling_squares)
    }

    /// Returns true if the given side can castle in the given direction, false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{Board, CastlingDirection, PlayerColor};
    /// let board = Board::new_default_starting_board();
    /// assert!(!board.has_castled_already(PlayerColor::White, CastlingDirection::Kingside));
    /// assert!(!board.has_castled_already(PlayerColor::White, CastlingDirection::Queenside));
    /// assert!(!board.has_castled_already(PlayerColor::Black, CastlingDirection::Kingside));
    /// assert!(!board.has_castled_already(PlayerColor::Black, CastlingDirection::Queenside));
    /// ```
    pub fn has_castled_already(&self, color: PlayerColor, check_direction: CastlingDirection) -> bool
    {
        match (color, check_direction)
        {
            (PlayerColor::White, CastlingDirection::Kingside) => *self.castling_availability.white_castle_kingside(),
            (PlayerColor::White, CastlingDirection::Queenside) => *self.castling_availability.white_castle_queenside(),
            (PlayerColor::Black, CastlingDirection::Kingside) => *self.castling_availability.black_castle_kingside(),
            (PlayerColor::Black, CastlingDirection::Queenside) => *self.castling_availability.black_castle_queenside(),
        }
    }

    fn check_squares_for_attack(&self, check_squares: &[Square]) -> bool
    {
        for square in check_squares
        {
            // We check the OPPOSITE color from the side currently moving 
            if !self.all_squares_that_can_capture_square(!self.active_color, *square).is_empty()
            {
                return true;
            }
        }
        return false;
    }
}
