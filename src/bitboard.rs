//! Implementation of bitboards, a wrapper around a u64.
//!
//! A bitboard (see [Wikipedia](https://en.wikipedia.org/wiki/Bitboard) and [ChessProgramming.org](https://www.chessprogramming.org/Bitboards) for more information)
//! is a method of representing the state of a chess board by taking advantage of the fact that a
//! chessboard has exactly 64 squares: and a 64-bit integer has 64 bits.
//!
//! Hence, we can use each bit of a 64-bit integer to represent whether or not a given piece is
//! present on a given square.
//!
//! While a single 64-bit integer is not enough to represent all of the pieces on the board, we can
//! use multiple integers and compose them together to deduce the entire board state.
//!
//! ```
//! let white_pawns: u64 = 0;
//! let white_knights: u64 = 0;
//! // etc...
//! ```
//! 
//! This has several advantages and a few disadvantages.
//!
//! One primary advantage is that a bitboard has a fixed size of 8 bytes (64 bits),
//! and since the set of all possible game pieces is finite, we can create board states with a
//! fixed, guaranteed size, allowing us to allocate on the stack instead of using a heap-allocated
//! structure like a [Vec] or [HashMap](std::collections::HashMap). It can also make certain calculations 
//! easier.
//!
//! Some downsides are that some of the operations done on bitboards are incredibly opaque, difficult
//! to reason about, and require extra math. It's also not uncommon to end up implementing a
//! HashMap or Vector based piece "mailbox" for caching *anyways*, reducing the effectiveness of
//! the fixed-size guarantee of bitboards.
//!
//! # Implementation
//! There are different ways to map a 64-bit integer to a chess board.
//!
//! Currently, we use least significant file mapping, where bit 0 (the LSB) represents a1,
//! then bit 1 represents b1, then c1, then bit 8 wraps around to represent a2.
//!
//! Hence:
//! - Each consecutive set of 8 bits represents one row or *rank* of squares.
//! - Thus, every 8th bit represents a square on the same *file* as the first.
//! - The LSB is defined as a1. The next LSB is b1, c1, etc.
//!
//! This can cause some confusion as a 64 bit integer is represented in code
//! from MSB to LSB going left to right, but the LSB is the leftmost, bottommost square
//! on a White-oriented chess board.
//!
//! This means that for a bit mask of file 0 ("a"), the bit representation looks like 
//! this when mapped to a chess board:
//!
//! ```none
//! 10000000
//! 10000000
//! 10000000
//! 10000000
//! 10000000
//! 10000000
//! 10000000
//! => 0b00000001000000010000000100000001000000010000000100000001
//! ```
//! Again, note that the LSB starts on the bottom left of a chessboard,
//! but is typically represented as the rightmost bit when writing out binary notation.
//!
//! For file 7 ("f"), the bit representation is
//! ```none
//! 00000001
//! 00000001
//! 00000001
//! 00000001
//! 00000001
//! 00000001
//! 00000001
//! => 0b10000000100000001000000010000000100000001000000010000000
//! ```
//!
//! And for rank 0, the bitmask looks like this:
//! 
//! ```none
//! 00000000
//! 00000000
//! 00000000
//! 00000000
//! 00000000
//! 00000000
//! 00000000
//! 11111111
//! => 0b11111111
//! ```
//!
//! And so on.
//!
//! # Other
//!
//! Note that while most of this library uses [UInt](crate::UInt) to choose whether or not to use u8s
//! or another integer type for coordinates, this library mandates u8s because of the hard-coded,
//! fixed size of bitboards.
//!
//! Generally, providing a rank, file or index out of bounds (>= 8, >=8, and >= 64 respectively)
//! will result in a panic. Hopefully at some point there will be try_* operations that return a [Result] instead.

use std::fmt::Display;

use bitboard_square_iterator::BitboardSquareIterator;
use derive_more::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, From, Into, Mul, MulAssign, Not, Shl, ShlAssign, Shr, ShrAssign};
use thiserror::Error;

use crate::board::Square;

mod bitboard_square_iterator;

/// The error type passed by a bitboard when invalid coordinates were used.
///
/// Invalid coordinates are basically any rank or file greater than or equal to 8,
/// or an index greater than or equal to 64.
#[derive(Debug, Error)]
#[error("Bitboard was given a {type_of_index} of {given}, cannot exceed {max}")]
pub struct OutOfBoundsError
{
    max: u8,
    given: u8,
    type_of_index: OutOfBoundsTypeError
}

impl OutOfBoundsError
{
    const fn check_input_coords(rank: u8, file: u8) -> Result<(), Self>
    {
        if rank >= 8
        {
            Err(Self {
                max: 7,
                given: rank,
                type_of_index: OutOfBoundsTypeError::Rank
            })
        }
        else if file >= 8
        {
            Err(Self {
                max: 7,
                given: file,
                type_of_index: OutOfBoundsTypeError::File
            })
        }
        else
        {
            Ok(())
        }
    }

    const fn check_input_index(index: u8) -> Result<(), Self>
    {
        if index >= 64
        {
            Err(Self {
                max: 63,
                given: index,
                type_of_index: OutOfBoundsTypeError::Index
            })
        }
        else
        {
            Ok(())
        }
    }
}

#[derive(Debug)]
enum OutOfBoundsTypeError
{
    /// A rank was passed in with a value greater than 8.
    Rank,
    /// A file was passed in with a value greater than 8.
    File,
    /// An index was passed in with a value greater than 64.
    Index
}

impl Display for OutOfBoundsTypeError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self
        {
            Self::Rank => write!(f,"rank")?,
            Self::File => write!(f,"file")?,
            Self::Index => write!(f, "index")?,
        };

        Ok(())
    }
}

/// See the [module-level documentation](./index.html) for more information on bitboards in general.
#[derive(Debug, Clone, Copy, From, Add, Mul, Into, AddAssign, MulAssign, BitOrAssign, BitOr, BitAndAssign, BitAnd, BitXor, BitXorAssign, Default, Shl, Shr, ShlAssign, ShrAssign, PartialEq, Not)]
pub struct Bitboard(u64);

impl Bitboard
{
    /// Returns a new bitboard with the corresponding 64 bit integer representation.
    ///
    /// # Arguments
    /// 
    /// * `input` - The 64 bit representation this integer should represent.
    /// # Examples
    ///
    /// ```
    /// let bitboard = Bitboard::new(0);
    /// assert_eq!(0, bitboard.into());
    /// ```
    pub const fn new(input: u64) -> Self
    {
        Self(input)
    }

    /// Converts a 2D set of coordinates to a 1D index in the bitboard "array".
    /// 
    /// Panics if the rank or file of the [Square] is greater than 7. See [Self::coords_to_index]
    /// for a fallible, non-panicking alternative.
    ///
    /// # Arguments
    ///
    /// * `square` - The square of the board. Neither file nor rank can be 8 or higher or this function will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// [TODO:write some example code]
    /// ```
    pub fn coords_to_index_unchecked(square: Square) -> u8
    {
        Self::coords_to_index(square).expect("The rank or file arguments exceeds the maximum of 7.")
    }

    /// Converts a 1D "index" into the bitboard array to the 2D coordinates that represents the
    /// same square.
    ///
    /// Panics if `index >= 64`. See [Self::index_to_coords] for the checked version that returns
    /// a [Result]
    ///
    /// # Arguments
    ///
    /// * `index` - The index. 0 -> a1, 1 -> b1, etc... Cannot exceed 63 or will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// [TODO:write some example code]
    /// ```
    pub fn index_to_coords_unchecked(index: u8) -> Square
    {
        Self::index_to_coords(index).expect("The index provided exceeds the maximum of 63.")
    }

    /// Like [Self::coords_to_index_unchecked] but fallible, returns an [Err] if the input args 
    /// exceed the maximum instead of panicking.
    ///
    /// # Arguments
    ///
    /// * `square` - The square, throws an [OutOfBoundsError] if the rank or file is 8 or greater.
    ///
    /// # Errors
    ///
    /// An [OutOfBoundsError] is returned if either of the input args are 8 or higher.
    ///
    /// # Examples
    ///
    /// ```
    /// [TODO:write some example code]
    /// ```
    pub fn coords_to_index(square: Square) -> Result<u8, OutOfBoundsError>
    {
        _ = OutOfBoundsError::check_input_coords(square.rank, square.file)?;
        Ok(8*square.rank + square.file)
    }

    /// Like [Self::index_to_coords_unchecked] but fallible, returns an [Err] if the input
    /// exceeds the maximum instead of panicking.
    ///
    /// # Arguments
    ///
    /// * `index` - The index. Returns an [OutOfBoundsError] if this is greater than 63.
    ///
    /// # Errors
    ///
    /// An [OutOfBoundsError] is returned if the input is 63 or higher.
    ///
    /// # Examples
    ///
    /// ```
    /// [TODO:write some example code]
    /// ```
    pub fn index_to_coords(index: u8) -> Result<Square, OutOfBoundsError>
    {
        _ = OutOfBoundsError::check_input_index(index)?;
        // this is equivalent to
        // (index / 8, index % 8)
        Ok(Square::new(index >> 3, index & 7))
    }

    /// Generates a bitmask that has only one rank of 8 bits set to 1,
    /// all other cells are set to 0.
    ///
    /// # Panics
    ///
    /// This function panics if `rank >= 8`.
    ///
    /// # Arguments
    ///
    /// * `rank` - The rank (from 0-7) to expose
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(0b11111111, Bitboard::rank_mask(0));
    /// assert_eq!(0b1111111100000000, Bitboard::rank_mask(1));
    /// assert_eq!(0b111111110000000000000000, Bitboard::rank_mask(2));
    /// ```
    pub const fn rank_mask(rank: u8) -> Self
    {
        assert!(rank < 8, "Error: Rank provided was greater than or equal to 8.");
        Bitboard::new(0b11111111 << (rank * 8))
    }

    /// Similar to [Self::rank_mask] but accepts an iterator over multiple ranks,
    /// returning the OR sum of all the ranks specified.
    ///
    /// # Panics
    /// This function panics if any of the values are `>= 8`.
    ///
    /// # Arguments
    ///
    /// * `rank_iter` - Anything that implements [`IntoIterator<Item = u8>`].
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(0b11111111, Bitboard::rank_mask_range(0..1));
    /// assert_eq!(0b11111111111111111, Bitboard::rank_mask_range(0..2));
    /// assert_eq!(0b1111111111111111100000000, Bitboard:rank_mask_range(1..3));
    /// ```
    pub fn rank_mask_iter<I>(rank_iter: I) -> Self
    where I: IntoIterator<Item = u8>
    {
        rank_iter.into_iter().map(Self::rank_mask).fold(Bitboard::default(), |acc, v| acc | v)
    }

    /// Generates a bitmask that has only one file of 8 bits set to 1.
    /// all other cells are set to 0.
    ///
    /// See the [module-level documentation](./index.html) for more information on the layout of
    /// files.
    ///
    /// # Arguments
    ///
    /// * `file` - The file to set the bitmask to.
    ///
    /// # Panics
    ///
    /// This function panics if `file >= 8`
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(0x0101010101010101, Bitboard::file_mask(0));
    /// assert_eq!(0x0202020202020202, Bitboard::file_mask(1));
    /// assert_eq!(0x0404040404040404, Bitboard::file_mask(2));
    /// ```
    pub const fn file_mask(file: u8) -> Self
    {
        // Yes, this is super hacky, it doesn't look great,
        // but it's about 10x faster than doing a for loop.
        // Yes, this is absolutely premature optimization, but I had fun with it so.
        Bitboard::new(match file
        {
            0 => 0b0000000100000001000000010000000100000001000000010000000100000001,
            1 => 0b0000001000000010000000100000001000000010000000100000001000000010,
            2 => 0b0000010000000100000001000000010000000100000001000000010000000100,
            3 => 0b0000100000001000000010000000100000001000000010000000100000001000,
            4 => 0b0001000000010000000100000001000000010000000100000001000000010000,
            5 => 0b0010000000100000001000000010000000100000001000000010000000100000,
            6 => 0b0100000001000000010000000100000001000000010000000100000001000000,
            7 => 0b1000000010000000100000001000000010000000100000001000000010000000,
            _ => panic!("Expected 0-7"),
        })
    }

    /// Same as [Self::file_mask] but accepts anything that implements [`IntoIterator<Item = u8>`],
    /// allowing this function to work over a range of files.
    ///
    /// The file masks will be OR summed together.
    ///
    /// # Panics
    ///
    /// This function panics if any of the files are greater than 8.
    ///
    /// # Arguments
    ///
    /// * `files` - Anything that implements [`IntoIterator<Item=u8>`],
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(0x0101010101010101, Bitboard::file_mask_iter(0..1));
    /// assert_eq!(0x0303030303030303, Bitboard::file_mask_iter(0..2));
    /// assert_eq!(0x0606060606060606, Bitboard::file_mask_iter(1..2));
    /// ```
    pub fn file_mask_iter<I>(files: I) -> Self
    where I: IntoIterator<Item = u8>
    {
        files.into_iter().map(Self::file_mask).fold(Self::default(), |acc, v| acc | v)
    }

    /// Returns an iterator over the current bitboard. This iterator
    /// passes through the bitboard from LSB to MSB, finds each currently active 1 bit,
    /// and returns the Square that that bit represents.
    ///
    /// # Examples
    ///
    /// ```
    /// // All bits set to 0.
    /// let bitboard = Bitboard::new(0);
    /// let squares: Vec<Square> = bitboard.squares().collect();
    /// assert!(squares.is_empty());
    /// // All bits set to 1.
    /// let bitboard = Bitboard::new(u64::MAX);
    /// let squares: Vec<Square> = bitboard.squares().collect();
    /// assert_eq!(squares.len(), 64);
    /// ```
    pub fn squares(&self) -> BitboardSquareIterator
    {
        BitboardSquareIterator::new(&self)
    }

    /// Returns true if a given bit is set to true,
    /// false otherwise. 
    ///
    /// Panics if `index >= 64`
    ///
    /// # Arguments
    ///
    /// * `index` - The *index*-th bit to test.
    ///
    /// # Examples
    ///
    /// ```
    /// [TODO:write some example code]
    /// ```
    pub fn is_bit_set(&self, index: u8) -> bool
    {
        // An AND mask with all bits turned off except
        // the desired bit, which will be set to whatever it is in self.
        // If the end result is all 0s, the desired bit was 0.
        // If the end result is not 0, the desired bit was 1.
        (*self & Self::new(2_u64.pow(index as u32))).0 != 0
    }

    /// Sets a specific bit in the bitboard.
    ///
    ///
    /// # Arguments
    ///
    /// * `index` - the index of the bit to set.
    /// * `to` - true if the bit should be set to true, false if the bit should be set to false.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut bitboard = Bitboard::new(0);
    /// // Sets the 0th bit on
    /// let new_bitboard = bitboard.set_bit(0, true);
    /// assert_eq!(new_bitboard, Bitboard::new(1));
    /// // Turns the 0th bit back off again
    /// let new_bitboard = new_bitboard.set_bit(0, false);
    /// assert_eq!(new_bitboard, Bitboard::new(0));
    /// ```
    pub fn set_bit(&self, index: u8, to: bool) -> Self
    {
        let mut new = self.clone();
        match to
        {
            true => new |= Self::new(2_u64.pow(index as u32)),
            false => new &= !Self::new(2_u64.pow(index as u32)),
        }
        new
    }
}

impl From<Square> for Bitboard
{
    /// Converts a [Square] into a [Bitboard] where all bits are set to 0
    /// except the bit representing the given square.
    ///
    /// # Arguments
    ///
    /// * `value` - The square to enable the mask for.
    ///
    /// # Panics
    /// 
    /// Panics if [Square] is out of bounds:
    /// i.e if rank or file is `>= 8`
    ///
    /// # Examples
    ///
    /// ```
    /// let bitboard: Bitboard = Square::new(0,0).into();
    /// // Bitboard representation: 0b0000...001
    /// assert_eq!(Bitboard::new(1), bitboard);
    ///
    /// let bitboard = Bitboard::from(Square::new(7,7));
    /// assert_eq!(Bitboard::new(0x8000_0000_0000_0000), bitboard);
    /// ```
    fn from(value: Square) -> Self {
        // Converts an index (i.e "the 12th bit")
        // to the number whose index is set to 1 (i.e the 12th bit is 1, all others are 0)
        Bitboard::new(2_u64.pow(Bitboard::coords_to_index_unchecked(value).into()))
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    #[should_panic]
    fn test_exceed_rank_fails()
    {
        Bitboard::coords_to_index_unchecked(Square::new(8,0));
    }

    #[test]
    #[should_panic]
    fn test_exceed_file_fails()
    {
        Bitboard::coords_to_index_unchecked(Square::new(0,8));
    }

    #[test]
    #[should_panic]
    fn test_exceed_index_fails()
    {
        Bitboard::index_to_coords_unchecked(64);
    }

    #[test]
    fn test_exceed_rank_returns_err()
    {
        assert!(Bitboard::coords_to_index(Square::new(8,0)).is_err());
    }

    #[test]
    fn test_exceed_file_returns_err()
    {
        assert!(Bitboard::coords_to_index(Square::new(0,8)).is_err());
    }

    #[test]
    fn test_exceed_index_returns_err()
    {
        assert!(Bitboard::index_to_coords(64).is_err());
    }

    #[test]
    fn test_normal_coords_ok()
    {
        let result = Bitboard::coords_to_index(Square::new(0,0));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_normal_coords_2()
    {
        let result = Bitboard::coords_to_index(Square::new(0,5));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5);
    }

    #[test]
    fn test_normal_coords_3()
    {
        let result = Bitboard::coords_to_index(Square::new(1,1));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 9);
    }

    #[test]
    fn test_normal_index()
    {
        let result = Bitboard::index_to_coords(0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Square::new(0, 0));
    }

    #[test]
    fn test_normal_index_2()
    {
        let result = Bitboard::index_to_coords(3);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Square::new(0, 3));
    }

    #[test]
    fn test_normal_index_3()
    {
        let result = Bitboard::index_to_coords(10);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Square::new(1, 2));
    }

    #[test]
    fn test_rank_mask()
    {
        let outputs = [0b11111111,
                       0b1111111100000000,
                       0b111111110000000000000000,
                       0b11111111000000000000000000000000,
                       0b1111111100000000000000000000000000000000,
                       0b111111110000000000000000000000000000000000000000,
                       0b11111111000000000000000000000000000000000000000000000000,
                       0b1111111100000000000000000000000000000000000000000000000000000000
        ];

        for rank in 0..8
        {
            assert_eq!(Bitboard::rank_mask(rank).0, outputs[rank as usize]);
        }
    }

    #[test]
    fn test_file_mask()
    {
        let outputs = [0x0101010101010101,
                       0x0202020202020202,
                       0x0404040404040404,
                       0x0808080808080808,
                       0x1010101010101010,
                       0x2020202020202020,
                       0x4040404040404040,
                       0x8080808080808080
        ];

        for file in 0..8
        {
            assert_eq!(Bitboard::file_mask(file).0, outputs[file as usize]);
        }
    }

    #[test]
    fn test_rank_mask_bitor_all_ranks()
    {
        let mut bitboard: Bitboard = Bitboard::new(0);

        for rank in 0..8
        {
            bitboard |= Bitboard::rank_mask(rank);
        }

        assert_eq!(bitboard.0, u64::MAX);
    }

    #[test]
    fn test_file_mask_bitor_all_ranks()
    {
        let mut bitboard = Bitboard::new(0);

        for file in 0..8
        {
            bitboard |= Bitboard::file_mask(file);
        }

        assert_eq!(bitboard.0, u64::MAX);
    }

    #[test]
    fn test_rank_mask_range_bitor_all_ranks()
    {
        assert_eq!(Bitboard::rank_mask_iter(0..8).0, u64::MAX);
    }

    #[test]
    fn test_file_mask_range_bitor_all_ranks()
    {
        assert_eq!(Bitboard::file_mask_iter(0..8).0, u64::MAX);
    }

    #[test]
    fn test_is_bit_set_all_bits_0()
    {
        let bitboard = Bitboard::new(0);
        for i in 0..64
        {
            assert!(!bitboard.is_bit_set(i));
        }
    }

    #[test]
    fn test_is_bit_set_1()
    {
        let bitboard = Bitboard::new(1);
        assert!(bitboard.is_bit_set(0));
        for i in 1..64
        {
            assert!(!bitboard.is_bit_set(i));
        }
    }

    #[test]
    fn test_is_bit_set_all_bits_1()
    {
        let bitboard = Bitboard::new(u64::MAX);
        for i in 0..64
        {
            assert!(bitboard.is_bit_set(i));
        }
    }

    #[test]
    #[should_panic]
    fn test_is_bit_set_panics_out_of_range()
    {
        let bitboard = Bitboard::new(0);
        bitboard.is_bit_set(64);
    }
}

