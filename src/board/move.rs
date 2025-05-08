use getset::CopyGetters;

use crate::parse::MoveCommandData;

use super::Square;

/// The direction the player is castling.
#[derive(Debug, Copy, Clone)]
pub enum CastlingDirection
{
    Kingside,
    Queenside
}

/// Represents a valid move on the board.
///
/// This covers "normal" moves which includes captures and basically any moves other than castling,
/// which are special and get treated differently than any normal move.
///
/// Whereas a [crate::parse::MoveCommand] is a 1-to-1 representation of the exact notation passed
/// into the program, a [Move] contains more information about the context around a move which
/// allows for the board to more easily modify its own state.
///
/// A [Move] is assumed to be possible by the board, but does not assume that a move is legal.
/// Basically when a [crate::board::Board] parses a [crate::parse::MoveCommand] it checks to see
/// whether the move:
/// - has a valid piece
/// - has a valid source square
/// - has a valid destination square
/// - Whether the destination square is occupied
///
/// But doesn't check things like:
/// - Occupancy in line of sight
/// - Whether the king is in check
#[derive(Debug, Copy, Clone)]
pub enum Move
{
    /// Basically any move that is not a castle.
    ///
    /// (Also probably won't include pawn promotions, which are currently unhandled)
    NormalMove(MoveData),
    /// A castle.
    Castle(CastlingDirection),
}

/// Contains information about the move relevant to the [crate::board::Board]
#[derive(Debug, CopyGetters, Copy, Clone)]
#[getset(get_copy="pub")]
pub struct MoveData
{
    /// The square that the move starts on
    starting_square: Square,
    /// Whether or not this move is a capture
    capture: bool,
    /// The square that this move ends on
    target_square: Square,
}

impl MoveData
{
    /// Constructs a new [MoveData]
    ///
    /// # Arguments
    ///
    /// * `piece_type` - The [PieceType]
    /// * `starting_square` - The starting square that the piece is on
    /// * `target_square` - The target square that the piece will end up on
    /// * `capture` - Whether or not we are capturing an existing piece.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{MoveData, Square};
    /// let move_data = MoveData::new(Square::new(0,0), Square::new(3,3), true);
    /// ```
    pub const fn new(starting_square: Square, target_square: Square, capture: bool) -> Self
    {
        Self
        {
            starting_square,
            capture,
            target_square,
        }
    }

    /// This function makes it slightly easier to construct a [MoveData] by taking a
    /// [MoveCommandData] and reading its information internally. The only supplemental information
    /// needed to construct a [MoveData], then, is what square the piece starts on, which is done
    /// here by taking it as an argument.
    ///
    /// # Arguments
    ///
    /// * `move_command_data` - The [MoveCommandData] to convert to a [MoveData]
    /// * `starting_square` - The starting square that the piece is on. This makes it so we only
    /// have to search the board for possible starting pieces only once.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use rust_chess_engine::parse::MoveCommandData;
    /// # use rust_chess_engine::board::{Move, Square, MoveData};
    /// let move_command_data = MoveCommandData::from_str("Nxe4").unwrap();
    /// // This function does not ensure that the MoveData makes sense
    /// // (i.e that the piece can move from the starting square to the target square)!!!
    /// let r#move: MoveData = MoveData::from_move_command_data(&move_command_data, Square::new(2,6));
    /// ```
    pub fn from_move_command_data(move_command_data: &MoveCommandData, starting_square: Square) -> Self
    {
        Self
        {
            starting_square,
            capture: move_command_data.capture(),
            target_square: move_command_data.target_square(),
        }
    }
}
