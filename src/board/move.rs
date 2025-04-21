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
pub enum Move
{
    NormalMove(MoveData),
    KingsideCastle,
    QueensideCastle,
}

/// Contains information about the move relevant to the [crate::board::Board]
pub struct MoveData
{
}
