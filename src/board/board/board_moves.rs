//! Helper module for the [board](super) module, exposing various bit-twiddling operations
//! for different piece moves and such.
//!
//! This module mostly operates on [Bitboards](crate::Bitboard), but we generally consider
//! the exact sequence of operations needed to test different moves outside of the scope of a
//! `Bitboard`, and the [Board](super::Board) impl is long enough as is.
//!
//! Most of the operations implemented here take a [Square](super::Square) and the active player's
//! color ([PlayerColor](super::PlayerColor)) as input and output
//! a [Bitboard](crate::Bitboard) representing the valid moves that piece can make.
//!
//! These operations take a [PlayerColor] because "occupied" squares are different depending on the
//! piece color. See [Occupancy](#Occupancy) for more information.
//!
//! # Occupancy
//!
//! Occupancy refers to whether or not a square is occupied. Here, when we say we "take occupancy
//! into account", that means we are checking the current state of the board to find valid squares
//! to move to. *All* pieces, and all moves here that generate a bitboard of valid moves
//! consider occupancy. 
//!
//! **A square that can be captured is not considered occupied.**
//!
//! But all squares beyond the capturable square are.
//!
//! Consider the following:
//! ```none
//! [O][][][][X][#][#][#]
//! ```
//! We'll say this represents a rank, and O is our player's rook, and X is another player's piece.
//! The square that X is on is unoccupied, because O can capture it, but all the squares beyond it
//! (represented as #) are considered "occupied", or rather, *unoccupyable* might be a better term.
//! 
//! **All squares with allied pieces on it are occupied.**
//!
//! This is why all operations in this module take a [PlayerColor], because it is possible for a
//! piece to move onto a currently occupied square as long as the piece being displaced is the
//! other player's. Under no circumstances is a piece allowed to move into a space occupied by
//! another piece of the same color. The only exception to that COULD be considered to be checking,
//! but checking is weird and is handled in its own edge case anyways.

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

        (Bitboard::default() 
            | north_north_east
            | north_east_east
            | south_east_east
            | south_south_east
            | south_south_west
            | south_west_west
            | north_west_west
            | north_north_west )
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
    /// let board = Board::new_blank_board();
    /// let king_moves = board.king_moves(PlayerColor::White, Square::new(0, 0));
    /// let king_move_squares: Vec<Square> = king_moves.squares().collect();
    /// assert_eq!(king_move_squares.len(), 3);
    /// assert_eq!(king_move_squares, vec![Square::new(1,0), Square::new(0,1), Square::new(1,1)]);
    /// ```
    pub fn king_moves(&self, active_color: PlayerColor, from: Square) -> Bitboard
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

        (Bitboard::default() 
            | north_east
            | north
            | north_west
            | east
            | west
            | south_east
            | south
            | south_west)
            // Check for occupancy
            & !self.query().color(active_color).result()
    }

    /// Generates a bitmask of all valid squares that a queen can move to, taking into account
    /// occupancy (i.e not moving through other pieces).
    ///
    /// Since a queen moves like a bishop + a rook, `queen_moves` is 
    /// equivalent to `bishop_moves | rook_moves`
    ///
    /// # Arguments
    ///
    /// * `active_color` - The moving/active piece's color
    /// * `from` - The square the piece is moving from
    ///
    /// # Examples
    ///
    /// ```
    /// [TODO:write some example code]
    /// ```
    pub fn queen_moves(&self, active_color: PlayerColor, from: Square) -> Bitboard
    {
        // A queen can move anywhere and everywhere a bishop or a rook can.
        self.bishop_moves(active_color, from) | self.rook_moves(active_color, from)
    }

    /// Generates a bit mask of all valid squares that a bishop can move to, taking into account
    /// occupancy.
    ///
    /// # Arguments
    ///
    /// * `active_color` - The active piece's color
    /// * `from` - The square the piece is moving from.
    ///
    /// # Examples
    ///
    /// ```
    /// [TODO:write some example code]
    /// ```
    pub fn bishop_moves(&self, active_color: PlayerColor, from: Square) -> Bitboard
    {
        self.moves_in_direction(active_color, from, DIRECTION_UP_LEFT) |
        self.moves_in_direction(active_color, from, DIRECTION_UP_RIGHT) |
        self.moves_in_direction(active_color, from, DIRECTION_DOWN_RIGHT) |
        self.moves_in_direction(active_color, from, DIRECTION_DOWN_LEFT)
    }

    /// Generates a bitmask of all valid squares that a rook can move to, taking into account
    /// occupancy.
    ///
    /// # Arguments
    ///
    /// * `active_color` - The active piece's color
    /// * `from` - The square the piece is moving from
    ///
    /// # Examples
    ///
    /// ```
    /// [TODO:write some example code]
    /// ```
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
                bitboard |= Bitboard::from(checked_square.unwrap());
                break;
            }
            // Finally if NONE of the following was true, we can add the current square and
            // continue onto the next square in the given direction
            bitboard |= Bitboard::from(checked_square.unwrap());
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
    /// invalid, as is a direction component where `abs(component) > 1`.
    ///
    /// # Arguments
    ///
    /// * `vertical_direction` - The vertical component. Can be -1, 0 (unless horizontal_direction is also 0), or 1 
    /// * `horizontal_direction` - The horizontal component. Can be -1, 0 (unless vertical_direction is also 0), or 1.
    ///
    /// # Examples
    ///
    /// ```
    /// let direction_up = Direction::new(1, 0);
    /// let direction_down_left = Direction::new(-1, -1);
    /// ```
    pub const fn new(vertical_direction: i8, horizontal_direction: i8) -> Option<Self>
    {
        match (vertical_direction, horizontal_direction)
        {
            (-1, -1) => Some(Self(0)),
            (-1, 0) => Some(Self(1)),
            (-1, 1) => Some(Self(2)),
            (0, -1) => Some(Self(3)),
            // We skip the direction (0, 0).
            (0, 1) => Some(Self(4)),
            (1, -1) => Some(Self(5)),
            (1, 0) => Some(Self(6)),
            (1, 1) => Some(Self(7)),
            _ => None
        }
    }

    /// Same as [Self::new] but panics if either of `vertical_direction`
    /// or `horizontal_direction` are outside of the allowed values.
    ///
    /// # Arguments
    ///
    /// * `vertical_direction` - The vertical component. Can be -1, 1, or 0 (unless
    /// `horizontal_direction` is also 0).
    /// * `horizontal_direction` - The horizontal component. Can be -1, 1, or 0 (unless 
    /// `vertical_direction` is also 0).
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// let 
    /// ```
    pub const fn new_unchecked(vertical_direction: i8, horizontal_direction: i8) -> Self
    {
        match Self::new(vertical_direction, horizontal_direction)
        {
            Some(x) => x,
            None => panic!("Failed to create new Direction!"),
        }
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
    pub const fn vertical_component(&self) -> i8
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
    pub const fn horizontal_component(&self) -> i8
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
    pub const fn as_tuple(&self) -> (i8, i8)
    {
        (self.vertical_component(), self.horizontal_component())
    }
}

#[cfg(test)]
mod tests
{
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
    fn knight_moves_with_occupancy_checks()
    {
        let board = Board::new_default_starting_board();
        // This is the b-file knight's default starting position
        let square = Square::new(0, 1);
        let knight_move_mask = board.knight_moves(PlayerColor::White, square);
        // Knight cannot move to d2 since a pawn is there.
        let expected_bitboard = 
            Bitboard::from(Square::new(2, 0)) |
            Square::new(2,2).into();
        assert_eq!(knight_move_mask, expected_bitboard);
    }

    #[test]
    fn knight_moves_with_occupancy_check_enemy_side()
    {
        let board = Board::new_default_starting_board();
        let square = Square::new(5, 2);
        let knight_move_mask = board.knight_moves(PlayerColor::White, square);
        // Test that a knight CAN move into squares occupied by the other side's pieces.
        let expected_bitboard =
            Bitboard::from(Square::new(7, 1)) |
            Square::new(7,3).into() |
            Square::new(6,4).into() |
            Square::new(4,4).into() |
            Square::new(3,1).into() |
            Square::new(4,0).into() |
            Square::new(6,0).into() |
            Square::new(7,1).into();
        assert_eq!(knight_move_mask, expected_bitboard);
    }

    #[test]
    fn king_moves_from_center()
    {
        let board = Board::new_blank_board();
        let king_move_mask = board.king_moves(PlayerColor::White, Square::new(1, 1));
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
        let board = Board::new_blank_board();
        let king_move_mask = board.king_moves(PlayerColor::White, Square::new(0, 0));
        let expected_bitboard =
            Bitboard::from(Square::new(0, 1)) |
            Square::new(1, 1).into() |
            Square::new(1, 0).into();
        assert_eq!(king_move_mask, expected_bitboard);
    }

    #[test]
    fn king_occupancy_default_board()
    {
        let board = Board::new_default_starting_board();
        let king_move_mask = board.king_moves(PlayerColor::White, Square::new(0, 4));
        assert_eq!(king_move_mask, Bitboard::new(0));
    }

    #[test]
    fn king_occupancy_enemy_pieces()
    {
        let board = Board::new_default_starting_board();
        let king_move_mask = board.king_moves(PlayerColor::White, Square::new(5, 7));
        let expected_bitboard =
            Bitboard::from(Square::new(6, 7)) |
            Square::new(6, 6).into() |
            Square::new(5, 6).into() |
            Square::new(4, 6).into() |
            Square::new(4, 7).into();
        assert_eq!(king_move_mask, expected_bitboard);
    }

    #[test]
    fn rook_moves_on_empty_board()
    {
        const AVAILABLE_ROOK_MOVES: usize = 14;
        let board = Board::new_blank_board();
        let rook_move_mask = board.rook_moves(PlayerColor::White, Square::new(0, 0));
        let rook_squares: Vec<Square> = rook_move_mask.squares().collect();
        assert_eq!(rook_squares.len(), AVAILABLE_ROOK_MOVES);

        let mut expected_rook_squares: Vec<Square> = Vec::with_capacity(AVAILABLE_ROOK_MOVES);
        for rank in 1..8
        {
            expected_rook_squares.push(Square::new(rank,0));
        }

        for file in 1..8
        {
            expected_rook_squares.push(Square::new(0, file));
        }

        assert_eq!(rook_squares, expected_rook_squares);
    }

    #[test]
    fn rook_cant_move_on_full_board()
    {
        let board = Board::new_default_starting_board();
        let rook_move_mask = board.rook_moves(PlayerColor::White, Square::new(0, 0));
        // Rook in the corner starts with no valid moves.
        assert_eq!(rook_move_mask, Bitboard::new(0));
    }

    #[test]
    fn rook_moves_into_opponent_pieces_occupancy_check()
    {
        const AVAILABLE_ROOK_MOVES: usize = 11;
        let board = Board::new_default_starting_board();
        let rook_move_mask = board.rook_moves(PlayerColor::White, Square::new(2, 0));
        let rook_squares: Vec<Square> = rook_move_mask.squares().collect();
        assert_eq!(rook_squares.len(), AVAILABLE_ROOK_MOVES);

        let mut expected_rook_squares: Vec<Square> = Vec::with_capacity(AVAILABLE_ROOK_MOVES);
        for file in 1..8
        {
            expected_rook_squares.push(Square::new(2, file));
        }

        for rank in 3..7
        {
            expected_rook_squares.push(Square::new(rank, 0));
        }
        // Seal against mutating vector anymore
        let expected_rook_squares = expected_rook_squares;

        assert_eq!(rook_squares, expected_rook_squares);
    }

    #[test]
    fn bishop_moves_on_empty_board()
    {
        const AVAILABLE_BISHOP_MOVES: usize = 7;
        let board = Board::new_blank_board();
        let bishop_move_mask = board.bishop_moves(PlayerColor::White, Square::new(0, 0));
        let bishop_squares: Vec<Square> = bishop_move_mask.squares().collect();
        assert_eq!(bishop_squares.len(), AVAILABLE_BISHOP_MOVES);

        let mut expected_bishop_squares: Vec<Square> = Vec::new();
        for i in 1..8
        {
            expected_bishop_squares.push(Square::new(i, i));
        }
        // Seal against mutating vector anymore.
        let expected_bishop_squares = expected_bishop_squares;

        assert_eq!(bishop_squares, expected_bishop_squares);
    }

    #[test]
    fn bishop_cant_move_on_full_board()
    {
        let board = Board::new_default_starting_board();
        let bishop_move_mask = board.bishop_moves(PlayerColor::White, Square::new(0, 2));
        assert_eq!(bishop_move_mask, Bitboard::new(0));
    }
}
