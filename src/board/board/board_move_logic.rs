//! This is a helper module for [Board](super::Board) that handles interpreting
//! [MoveCommand](crate::parse::MoveCommand) and just figuring out how to handle a given move.
//!
//! This is all private to [Board](super::Board), it's just meant to clean up the module a bit.

use crate::{bitboard::Bitboard, board::{r#move::MoveData, Move, MoveError, PieceType, PlayerColor, Square}, parse::MoveCommandData};

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
    /// [TODO:write some example code]
    /// ```
    pub fn parse_normal_move(&self, move_data: &MoveCommandData) -> Result<Move, MoveError>
    {
        let starting_squares = match move_data.is_capture()
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
    /// # Arguments
    ///
    /// * `piece_color` - The piece color to search for.
    /// * `piece_type` - The piece type to search for
    /// * `square` - The target square that is being moved to.
    ///
    /// # Examples
    ///
    /// ```
    /// [TODO:write some example code]
    /// ```
    // Ugh I hate how stupid this function is getting... I need to figure out a better way to
    // do this lol
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
            |square| !(move_type(&self, self.active_color, *square) & target_square_bitboard).is_empty()
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
    /// [TODO:write some example code]
    /// ```
    pub fn squares_of_type_that_can_capture_square(&self, piece_color: PlayerColor, piece_type: PieceType, square: Square) -> Vec<Square>
    {
        let piece_map = self.pieces_of_type(piece_type) & self.pieces_of_color(piece_color);
        let target_square_bitboard: Bitboard = square.into();

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
            |square| !(move_type(&self, self.active_color, *square) & target_square_bitboard).is_empty()
        ).collect()
    }

    pub fn all_squares_that_can_capture_square(&self, attacking_color: PlayerColor, target_square: Square) -> Vec<Square>
    {
        let piece_map = self.pieces_of_color(attacking_color);

        // Check every single attacking piece to see if it can attack the target square lmao.
        piece_map.squares().map(
            |square| self.squares_of_type_that_can_capture_square(attacking_color, self.piece_at(&square).expect("Somehow there wasn't a piece at this square???").piece_type(), target_square)
        ).collect::<Vec<Vec<Square>>>().concat()
    }
}

