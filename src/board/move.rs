use std::marker::PhantomData;

// We're setting up some weird type state schenanigans!!!!
pub trait Legality {}

pub struct Legal;
pub struct UnknownLegal;

impl Legality for Legal {}
impl Legality for UnknownLegal {}

/// Represents a valid, potential move on the board.
///
/// This struct is created via [board.get_move()](crate::board::Board::get_move). It comes with two
/// type variants: Move<[UnknownLegal]> is a move that has not been verified via
/// [board.check_move()](crate::board::Board::check_move), while Move<[Legal]> is.
///
/// The [Move<Legal>] can be used to enter new moves into the board and update/change the board
/// state, while [Move<UnknownLegal>] cannot.
///
/// See [Board](crate::board::Board) for more information.
///
/// Note that in theory you could create two boards and use one board to create the move and check
/// it, but then plug that move into a *different* board. In theory, totally valid according to the
/// type system but you should never do that. That being said, the API for interacting with a
/// [Board](crate::board::Board) might change to prevent it all the same.
pub struct Move<L: Legality>
{
    _phantom: PhantomData<L>,
}

impl Move<UnknownLegal> 
{
    /// Returns false
    ///
    /// A move of unknown legality is treated as illegal until it has been verified against a
    /// board.
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
    /// // A move is NOT legal until it has been checked against the board.
    /// assert!(!r#move.unwrap().is_legal());
    /// ```
    pub fn is_legal(&self) -> bool
    {
        return false;
    }
}

impl Move<Legal>
{
    /// Returns true
    ///
    /// This state confirms that the board has verified this move as true.
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
    /// assert!(r#move.is_some());
    /// // This move has been checked, and e4 should be a legal move.
    /// // If it wasn't, board.check_move() would've returned None instead.
    /// assert!(r#move.unwrap().is_legal());
    /// ```
    pub fn is_legal(&self) -> bool
    {
        return true;
    }
}



