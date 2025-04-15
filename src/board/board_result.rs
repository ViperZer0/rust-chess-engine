use thiserror::Error;

use super::PlayerColor;

/// The error type returned by [BoardResult::get_winner()].
#[derive(Debug, Error, PartialEq)]
pub enum GetWinnerError
{
    /// There is no winner because the game is still in progress.
    #[error("No winner; the game is still in progress!")]
    StillInProgress,
    /// There is no winner because the game ended in a draw.
    #[error("No winner, the game ended in a draw.")]
    Draw(DrawReason),
}

/// The current game outcome. A game still in progress is [InProgress],
/// while a game that has ended will have one of various enum values recording the game outcome,
/// whether it was a victory for one of the players or one of various draw conditions.
pub enum BoardResult
{
    /// A game is still in progress.
    InProgress,
    /// White won.
    WhiteWin,
    /// Black won.
    BlackWin,
    /// The game was a draw, the reason is recorded in [DrawReason]
    Draw(DrawReason),
}

/// If the game is a draw, this enum records the reason for the draw.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DrawReason
{
    /// A draw where the same board position was reached three times.
    ThreefoldRepitition,
    /// A draw where 50 moves have been made without a pawn move or capture.
    FiftyMoveRule,
    /// A dead position where neither player can possibly give checkmate.
    CheckmateImpossible,
    /// A stalemate, where one player has no legal moves but is not currently in check.
    Stalemate,
    /// A draw where both players have agreed to draw.
    Agreement
}

impl BoardResult
{
    /// Returns true if the game is still in progress, 
    /// returns false if the game is over.
    /// See also [Self::is_over]
    pub fn is_in_progress(&self) -> bool
    {
        match self
        {
            Self::InProgress => true,
            _ => false,
        }
    }

    /// Returns true if the game is over,
    /// returns false if the game is still in progress.
    /// See also [Self::is_in_progress]
    pub fn is_over(&self) -> bool
    {
        !self.is_in_progress()
    }

    /// Returns true if the game ended in a victory for either White or Black.
    /// Returns false if the game is either in progress or ended in a draw.
    pub fn has_winner(&self) -> bool
    {
        match self
        {
            Self::WhiteWin => true,
            Self::BlackWin => true,
            Self::InProgress => false,
            Self::Draw(_) => false,
        }
    }

    /// Returns the [PlayerColor] of the winning side,
    /// or [None] if the game is still in progress or ended in a draw.
    pub fn get_winner(&self) -> Result<PlayerColor, GetWinnerError>
    {
        match self
        {
            Self::WhiteWin => Ok(PlayerColor::White),
            Self::BlackWin => Ok(PlayerColor::Black),
            Self::InProgress => Err(GetWinnerError::StillInProgress),
            Self::Draw(x) => Err(GetWinnerError::Draw(*x)),
        }
    }

    /// Returns true if the game ended in a draw, false if the game is still in progress or ended
    /// with a winner.
    pub fn is_draw(&self) -> bool
    {
        match self
        {
            Self::Draw(_) => true,
            _ => false,
        }
    }

    /// Returns the reason for a draw, or [None] if the game is either in progress or was won 
    /// without a draw.
    pub fn get_draw_reason(&self) -> Option<DrawReason>
    {
        match self
        {
            Self::InProgress => None,
            Self::WhiteWin => None,
            Self::BlackWin => None,
            // Trivial copy here.
            Self::Draw(x) => Some(*x),
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_is_in_progress()
    {
        assert_eq!(false, BoardResult::BlackWin.is_in_progress());
        assert_eq!(false, BoardResult::WhiteWin.is_in_progress());
        assert_eq!(false, BoardResult::Draw(DrawReason::Agreement).is_in_progress());
        assert_eq!(true, BoardResult::InProgress.is_in_progress());
    }

    #[test]
    fn test_is_over()
    {
        assert_eq!(true, BoardResult::BlackWin.is_in_progress());
        assert_eq!(true, BoardResult::WhiteWin.is_in_progress());
        assert_eq!(true, BoardResult::Draw(DrawReason::Agreement).is_in_progress());
        assert_eq!(false, BoardResult::InProgress.is_in_progress());
    }

    #[test]
    fn test_winner()
    {
        assert!(BoardResult::InProgress.get_winner().is_err());
        assert!(BoardResult::Draw(DrawReason::Agreement).get_winner().is_err());
        assert_eq!(Ok(PlayerColor::White), BoardResult::WhiteWin.get_winner());
        assert_eq!(Ok(PlayerColor::Black), BoardResult::BlackWin.get_winner());
    }

    #[test]
    fn test_is_draw()
    {
        assert_eq!(false, BoardResult::BlackWin.is_draw());
        assert_eq!(false, BoardResult::WhiteWin.is_draw());
        assert_eq!(true, BoardResult::Draw(DrawReason::Agreement).is_draw());
        assert_eq!(false, BoardResult::InProgress.is_draw());
    }

    #[test]
    fn test_get_draw_reason()
    {
        assert_eq!(None, BoardResult::BlackWin.get_draw_reason());
        assert_eq!(None, BoardResult::WhiteWin.get_draw_reason());
        assert_eq!(None, BoardResult::InProgress.get_draw_reason());
        assert_eq!(Some(DrawReason::Agreement), BoardResult::Draw(DrawReason::Agreement).get_draw_reason());
    }
}
