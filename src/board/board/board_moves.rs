//! Helper module for the [board](super) module, exposing various bit-twiddling operations
//! for different piece moves and such.
//!
//! This module mostly operates on [Bitboards](crate::Bitboard), but we generally consider
//! the exact sequence of operations needed to test different moves outside of the scope of a
//! `Bitboard`, and the [Board](super::Board) impl is long enough as is.
//!
//! Most of the operations implemented here take a [Square](super::Square) as input and output
//! a [Bitboard](crate::Bitboard) representing the valid moves that piece can make.
//!

use crate::{bitboard::Bitboard, board::{PlayerColor, Square}};

use super::Board;

// All possible directions, orthogonal and directional, specified at compile time.
// This is used to calculate moves for queens, bishops, and rooks.
const DIRECTION_UP: Direction = Direction(4);
const DIRECTION_UP_RIGHT: Direction = Direction(7);
const DIRECTION_RIGHT: Direction = Direction(6);
const DIRECTION_DOWN_RIGHT: Direction = Direction(2);
const DIRECTION_DOWN: Direction = Direction(1);
const DIRECTION_DOWN_LEFT: Direction = Direction(0);
const DIRECTION_LEFT: Direction = Direction(3);
const DIRECTION_UP_LEFT: Direction = Direction(5);

impl Board {
    /// Generates a bitboard where all valid squares that a knight on a given square can move to are set to 1.
    /// The square that the knight is currently on is not included in this set.
    ///
    /// # Arguments
    ///
    /// * `from` - The square that the knight is on.
    ///
    /// # Examples
    ///
    /// ```
    /// let knight_moves: Bitboard = Board::knight_moves(Square::new(0,0));
    /// let knight_move_squares: Vec<Square> = knight_moves.squares().collect();
    /// // This should be 2 since a knight in the corner only ever has two moves.
    /// assert_eq!(knight_move_squares.len(), 2);
    /// assert_eq!(knight_move_squares, vec![Square::new(1, 2), Square::new(2, 1)]);
    /// ```
    pub fn knight_moves(&self, active_color: PlayerColor, from: Square) -> Bitboard
    {
        // For each possible knight move, we want to mask out the areas of the board that the
        // knight can't move from.
        // I.e if the knight is on the edge of the board, we can't move further to the left.
        let knight_on_rank_1_or_higher = Bitboard::from(from) & Bitboard::rank_mask_iter(1..8);
        let knight_on_rank_2_or_higher = Bitboard::from(from) & Bitboard::rank_mask_iter(2..8);
        let knight_on_rank_6_or_lower  = Bitboard::from(from) & Bitboard::rank_mask_iter(0..7);
        let knight_on_rank_5_or_lower  = Bitboard::from(from) & Bitboard::rank_mask_iter(0..6);
        let knight_on_file_b_to_h      = Bitboard::from(from) & Bitboard::file_mask_iter(1..8);
        let knight_on_file_c_to_h      = Bitboard::from(from) & Bitboard::file_mask_iter(2..8);
        let knight_on_file_a_to_g      = Bitboard::from(from) & Bitboard::file_mask_iter(0..7);
        let knight_on_file_a_to_f      = Bitboard::from(from) & Bitboard::file_mask_iter(0..6);

        // Each bitshift is the number of squares between the source and target square
        // With a bitmask to block out "bad" areas that can't be moved from.
        let north_north_east = (knight_on_rank_5_or_lower & knight_on_file_a_to_g) << 17;
        let north_east_east = (knight_on_rank_6_or_lower & knight_on_file_a_to_f) << 10;
        let south_east_east = (knight_on_rank_1_or_higher & knight_on_file_a_to_f) >> 6;
        let south_south_east = (knight_on_rank_2_or_higher & knight_on_file_a_to_g) >> 15;
        let south_south_west = (knight_on_rank_2_or_higher & knight_on_file_b_to_h) >> 17;
        let south_west_west = (knight_on_rank_1_or_higher & knight_on_file_c_to_h) >> 10;
        let north_west_west = (knight_on_rank_6_or_lower & knight_on_file_c_to_h) << 6;
        let north_north_west = (knight_on_rank_5_or_lower & knight_on_file_b_to_h) << 15;

        Bitboard::default() 
            | north_north_east
            | north_east_east
            | south_east_east
            | south_south_east
            | south_south_west
            | south_west_west
            | north_west_west
            | north_north_west
            // We exclude squares that are occupied by our own pieces.
            & !self.query().color(active_color).result()
    }

    /// Generates a bitmask of all valid squares that a king can move to.
    ///
    /// This is typically the 8 surrounding squares, unless it's on a corner or edge.
    /// This does not include the square the king is on.
    ///
    /// Note that this does NOT check whether this places the king in check, nor does it check for
    /// occupancy or overlapping squares! But maybe it should... hmm...
    ///
    /// # Arguments
    ///
    /// * `from` - The square the king is currently on.
    ///
    /// # Examples
    ///
    /// ```
    /// let king_moves = Board::king_moves(Square::new(0, 0));
    /// let king_move_squares: Vec<Square> = king_moves.squares().collect();
    /// assert_eq!(king_move_squares.len(), 3);
    /// assert_eq!(king_move_squares, vec![Square::new(1,0), Square::new(0,1), Square::new(1,1)]);
    /// ```
    pub fn king_moves(from: Square) -> Bitboard
    {
        let king_on_rank_1_or_higher = Bitboard::from(from) & Bitboard::rank_mask_iter(1..8);
        let king_on_rank_6_or_lower = Bitboard::from(from) & Bitboard::rank_mask_iter(0..7);
        let king_on_file_b_to_h = Bitboard::from(from) & Bitboard::file_mask_iter(1..8);
        let king_on_file_a_to_g = Bitboard::from(from) & Bitboard::file_mask_iter(0..7);

        let north_east = (king_on_rank_6_or_lower & king_on_file_a_to_g) << 9;
        let north = (king_on_rank_6_or_lower) << 8;
        let north_west = (king_on_rank_6_or_lower & king_on_file_b_to_h) << 7;
        let east = (king_on_file_a_to_g) << 1;
        let west = (king_on_file_b_to_h) >> 1;
        let south_east = (king_on_rank_1_or_higher & king_on_file_a_to_g) >> 7;
        let south = (king_on_rank_1_or_higher) >> 8;
        let south_west = (king_on_rank_1_or_higher & king_on_file_b_to_h) >> 9;

        Bitboard::default() 
            | north_east
            | north
            | north_west
            | east
            | west
            | south_east
            | south
            | south_west
    }

    pub fn queen_moves(&self, active_color: PlayerColor, from: Square) -> Bitboard
    {
        // A queen can move anywhere and everywhere a bishop or a rook can.
        self.bishop_moves(active_color, from) | self.rook_moves(active_color, from)
    }

    pub fn bishop_moves(&self, active_color: PlayerColor, from: Square) -> Bitboard
    {
        self.moves_in_direction(active_color, from, DIRECTION_UP_LEFT) |
        self.moves_in_direction(active_color, from, DIRECTION_UP_RIGHT) |
        self.moves_in_direction(active_color, from, DIRECTION_DOWN_RIGHT) |
        self.moves_in_direction(active_color, from, DIRECTION_DOWN_LEFT)
    }

    pub fn rook_moves(&self, active_color: PlayerColor, from: Square) -> Bitboard
    {
        self.moves_in_direction(active_color, from, DIRECTION_UP) |
        self.moves_in_direction(active_color, from, DIRECTION_RIGHT) | 
        self.moves_in_direction(active_color, from, DIRECTION_DOWN) | 
        self.moves_in_direction(active_color, from, DIRECTION_LEFT)
    }

    /// Calculates available moves in a certain direction from a square,
    /// while taking into account occupancy (i.e squares that are blocked by other pieces)
    ///
    /// "Occupancy" considers any squares occupied by a same-colored piece (or past one) occupied.
    /// Any squares occupied by one opposite-color piece are considered movable (since we can
    /// capture pieces), but any squares beyond that are occupied.
    ///
    /// This is a private helper method but forms all of the core logic for [Self::queen_moves], 
    /// [Self::bishop_moves], and [Self::rook_moves], all of which move in the exact same way.
    /// # Arguments
    ///
    /// * `active_color` - What color is currently moving. We need this to determine which occupied
    /// squares are still fair game to move into.
    /// * `from` - The starting squares of the piece.
    /// * `direction` - The direction to check in.
    fn moves_in_direction(&self, active_color: PlayerColor, from: Square, direction: Direction) -> Bitboard
    {
        let mut bitboard = Bitboard::new(0);
        // We set this to true after we hit another piece or the edge of the board.
        let mut checked_square = increment_square_in_direction(&from, direction);
        loop
        {
            // We hit an edge, break out.
            if checked_square.is_none()
            {
                break;
            }
            // If there is a same colored piece on the target square, we break out.
            if self.query().color(active_color).piece_at_unchecked(checked_square.unwrap()).result()
            {
                break;
            }
            // if there is an opposite colored piece on the target square, we include that square
            // in the final bitboard, but we still break out bc we can't move past it.
            if self.query().color(!active_color).piece_at_unchecked(checked_square.unwrap()).result()
            {
                // Enable the bit for the current square.
                bitboard |= Bitboard::new(0).set_bit(Bitboard::coords_to_index_unchecked(checked_square.unwrap()), true);
                break;
            }
            // Finally if NONE of the following was true, we can add the current square and
            // continue onto the next square in the given direction
            bitboard |= Bitboard::new(0).set_bit(Bitboard::coords_to_index_unchecked(checked_square.unwrap()), true);
            checked_square = increment_square_in_direction(&checked_square.unwrap(), direction);
        }
        bitboard
    }
}


// Returns a new square with rank and file incremented or decremented depending on the Direction
// provided.
fn increment_square_in_direction(square: &Square, direction: Direction) -> Option<Square>
{
    let new_rank: i8 = square.rank as i8 + direction.vertical_component();
    let new_file: i8 = square.file as i8 + direction.horizontal_component();
    if new_rank < 0 || new_rank >= 8
    {
        None
    }
    else if new_file < 0 || new_file >= 8
    {
        None
    }
    else
    {
        Some(Square {
            rank: new_rank as u8,
            file: new_file as u8,
        })
    }
}

/// Represents one of 8 directions that rooks/queens/bishops can move in. Used for
/// occupancy/movement checks. 
///
/// We use a u8 internally bc Rust isn't actually smart enough to know that there are only 8
/// possible directions when using a tuple, so we map those 8 possible directions to a u8.
#[derive(Copy, Clone)]
struct Direction(u8);

impl Direction
{
    /// Creates a new direction from the two components/basis of the direction space.
    ///
    /// Note that the magnitude of the direction vector cannot be 0, so `Direction::new(0, 0)` is
    /// invalid.
    ///
    /// # Arguments
    ///
    /// * `vertical_direction` - The vertical component. Can be -1, 0 (unless horizontal_direction is also 0), or 1 
    /// * `horizontal_direction` - The horizontal component. Can be -1, 0 (unless vertical_direction is also 0), or 1.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - Both vertical_direction and horizontal_direction are 0.
    /// - vertical_direction > 1 or veritcal_direction < -1
    /// - horizontal_direction > 1 or horizontal_direction < -1
    ///
    /// # Examples
    ///
    /// ```
    /// let direction_up = Direction::new(1, 0);
    /// let direction_down_left = Direction::new(-1, -1);
    /// ```
    pub fn new(vertical_direction: i8, horizontal_direction: i8) -> Self
    {
        let x = match (vertical_direction, horizontal_direction)
        {
            (-1, -1) => 0,
            (-1, 0) => 1,
            (-1, 1) => 2,
            (0, -1) => 3,
            // We skip the direction (0, 0).
            (0, 1) => 4,
            (1, -1) => 5,
            (1, 0) => 6,
            (1, 1) => 7,
            _ => panic!("Invalid direction components specified!"),
        };
        Self(x)
    }

    /// Returns the magnitude of the vertical component.
    ///
    /// This can be either -1, 0, or 1.
    ///
    /// # Examples
    ///
    /// ```
    /// let direction = Direction::new(1, 0);
    /// assert_eq!(direction.vertical_component(), 1);
    /// let direction = Direction::new(0, 1);
    /// assert_eq!(direction.vertical_component(), 0);
    /// let direction = Direction::new(-1, -1);
    /// assert_eq!(direction.vertical_component(), -1);
    /// ```
    pub fn vertical_component(&self) -> i8
    {
        match self.0
        {
            0|1|2 => -1,
            3|4 => 0,
            5|6|7 => 1,
            _ => unreachable!(),
        }
    }

    /// Returns the magnitude of the horizontal component.
    ///
    /// This can be either -1, 0, or 1.
    ///
    /// # Examples
    ///
    /// ```
    /// let direction = Direction::new(1, -1);
    /// assert_eq!(direction.horizontal_component(), -1);
    /// let direction = Direction::new(-1, 0);
    /// assert_eq!(direction.horizontal_component(), 0);
    /// let direction = Direction::new(0, 1);
    /// assert_eq!(direction.horizontal_component(), 1);
    /// ```
    pub fn horizontal_component(&self) -> i8
    {
        match self.0
        {
            0|3|5 => -1,
            1|6   => 0,
            2|4|7 => 1,
            _ => unreachable!(),
        }
    }

    /// Returns the internal state represented by the Direction as a tuple,
    /// where the first element is the vertical component and the second is the horizontal
    /// component.
    ///
    /// Note that this is basically flipped from standard notation but matches better with the
    /// representation used by [Square] and such.
    ///
    /// # Examples
    ///
    /// ```
    /// let direction = Direction::new(1, 0);
    /// assert_eq!(direction.as_tuple(), (1,0));
    /// ```
    pub fn as_tuple(&self) -> (i8, i8)
    {
        (self.vertical_component(), self.horizontal_component())
    }
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn knight_moves_from_center()
    {
        let board = Board::new_blank_board();
        let knight_move_mask = board.knight_moves(PlayerColor::White, Square::new(3, 3));
        let expected_bitboard: Bitboard = 
            Bitboard::from(Square::new(1, 2)) |
            Bitboard::from(Square::new(1, 4)) |
            Bitboard::from(Square::new(2, 1)) |
            Bitboard::from(Square::new(2, 5)) |
            Bitboard::from(Square::new(4, 1)) |
            Bitboard::from(Square::new(4, 5)) |
            Bitboard::from(Square::new(5, 2)) |
            Bitboard::from(Square::new(5, 4));
        assert_eq!(knight_move_mask, expected_bitboard);
    }

    #[test]
    fn knight_moves_from_corner_1()
    {
        let board = Board::new_blank_board();
        let knight_move_mask = board.knight_moves(PlayerColor::White, Square::new(0, 0));
        let expected_bitboard: Bitboard = 
            Bitboard::from(Square::new(1, 2)) |
            Square::new(2, 1).into();
        assert_eq!(knight_move_mask, expected_bitboard);
    }

    #[test]
    fn knight_moves_from_corner_2()
    {
        let board = Board::new_blank_board();
        let knight_move_mask = board.knight_moves(PlayerColor::White, Square::new(7, 7));
        let expected_bitboard =
            Bitboard::from(Square::new(6, 5)) |
            Square::new(5, 6).into();
        assert_eq!(knight_move_mask, expected_bitboard);
    }

    #[test]
    fn knight_moves_from_side()
    {
        let board = Board::new_blank_board();
        let knight_move_mask = board.knight_moves(PlayerColor::White, Square::new(3, 7));
        let expected_bitboard =
            Bitboard::from(Square::new(1, 6)) |
            Square::new(2, 5).into() |
            Square::new(4, 5).into() |
            Square::new(5, 6).into();
        assert_eq!(knight_move_mask, expected_bitboard);
    }

    #[test]
    fn king_moves_from_center()
    {
        let king_move_mask = Board::king_moves(Square::new(1, 1));
        let expected_bitboard = 
            Bitboard::from(Square::new(0, 0)) |
            Square::new(0, 1).into() |
            Square::new(0, 2).into() |
            Square::new(1, 2).into() |
            Square::new(2, 2).into() |
            Square::new(2, 1).into() |
            Square::new(2, 0).into() |
            Square::new(1, 0).into();
        assert_eq!(king_move_mask, expected_bitboard);
    }

    #[test]
    fn king_moves_from_corner()
    {
        let king_move_mask = Board::king_moves(Square::new(0, 0));
        let expected_bitboard =
            Bitboard::from(Square::new(0, 1)) |
            Square::new(1, 1).into() |
            Square::new(1, 0).into();
        assert_eq!(king_move_mask, expected_bitboard);
    }
}
