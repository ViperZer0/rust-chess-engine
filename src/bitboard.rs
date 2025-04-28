//! Bitboards!!!
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
//! structure like a [Vec] or [std::collections::HashMap]. It can also make certain calculations 
//! easier.
//!
//! Downsides are that some of the operations done on bitboards are incredibly opaque, difficult
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
//! Note that while most of this library uses [crate::UInt] to choose whether or not to use u8s
//! or another integer type for coordinates, this library mandates u8s because of the hard-coded,
//! fixed size of bitboards.
//!
//! Generally, providing a rank, file or index out of bounds (>= 8, >=8, and >= 64 respectively)
//! will result in a panic.
//!
//! Hopefully there will be try_* operations that return a result.

use std::fmt::Display;

use thiserror::Error;

use crate::board::Square;


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
    fn check_input_coords(rank: u8, file: u8) -> Result<(), Self>
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

    fn check_input_index(index: u8) -> Result<(), Self>
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

/// See the module-level documentation for more information.
#[derive(Debug, Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard
{
    /// Converts a 2D set of coordinates to a 1D index in the bitboard "array".
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
    pub fn coords_to_index(square: Square) -> u8
    {
        Self::try_coords_to_index(square).expect("The rank or file arguments exceeds the maximum of 7.")
    }

    /// Converts a 1D "index" into the bitboard array to the 2D coordinates that represents the
    /// same square.
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
    pub fn index_to_coords(index: u8) -> Square
    {
        Self::try_index_to_coords(index).expect("The index provided exceeds the maximum of 63.")
    }

    /// Like [Self::coords_to_index] but fallible, returns an [Err] if the input args 
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
    pub fn try_coords_to_index(square: Square) -> Result<u8, OutOfBoundsError>
    {
        _ = OutOfBoundsError::check_input_coords(square.rank, square.file)?;
        Ok(8*square.rank + square.file)
    }

    /// Like [Self::index_to_coords] but fallible, returns an [Err] if the input
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
    pub fn try_index_to_coords(index: u8) -> Result<Square, OutOfBoundsError>
    {
        _ = OutOfBoundsError::check_input_index(index)?;
        // this is equivalent to
        // (index / 8, index % 8)
        Ok(Square::new(index >> 3, index & 7))
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
        Bitboard::coords_to_index(Square::new(8,0));
    }

    #[test]
    #[should_panic]
    fn test_exceed_file_fails()
    {
        Bitboard::coords_to_index(Square::new(0,8));
    }

    #[test]
    #[should_panic]
    fn test_exceed_index_fails()
    {
        Bitboard::index_to_coords(64);
    }

    #[test]
    fn test_exceed_rank_returns_err()
    {
        assert!(Bitboard::try_coords_to_index(Square::new(8,0)).is_err());
    }

    #[test]
    fn test_exceed_file_returns_err()
    {
        assert!(Bitboard::try_coords_to_index(Square::new(0,8)).is_err());
    }

    #[test]
    fn test_exceed_index_returns_err()
    {
        assert!(Bitboard::try_index_to_coords(64).is_err());
    }

    #[test]
    fn test_normal_coords_ok()
    {
        let result = Bitboard::try_coords_to_index(Square::new(0,0));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_normal_coords_2()
    {
        let result = Bitboard::try_coords_to_index(Square::new(0,5));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5);
    }

    #[test]
    fn test_normal_coords_3()
    {
        let result = Bitboard::try_coords_to_index(Square::new(1,1));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 9);
    }

    #[test]
    fn test_normal_index()
    {
        let result = Bitboard::try_index_to_coords(0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Square::new(0, 0));
    }

    fn test_normal_index_2()
    {
        let result = Bitboard::try_index_to_coords(3);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Square::new(0, 3));
    }

    fn test_normal_index_3()
    {
        let result = Bitboard::try_index_to_coords(10);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Square::new(1, 2));
    }
}

