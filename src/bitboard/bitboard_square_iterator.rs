//! This module contains the [BitboardSquareIterator], used to iterator over a bitboard
//! to get each activated square, given by its bit being set to 1.

use crate::board::Square;

use super::Bitboard;

// Bitboard with all the bits set to 0 except for the LSB.
const LSB_MASK: Bitboard = Bitboard(1);

pub struct BitboardSquareIterator<'a>
{
    // The underlying bitboard reference.
    // The iterator must last at least as long as the bitboard does,
    // since the iterator does not own the bitboard.
    bitboard: &'a Bitboard,
    // The current index we are checking.
    cur_index: u8
}

impl<'a> BitboardSquareIterator<'a>
{
    pub fn new(bitboard: &'a Bitboard) -> Self
    {
        BitboardSquareIterator
        {
            bitboard,
            cur_index: 0,
        }
    }
}

impl Iterator for BitboardSquareIterator<'_>
{
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        // Initial sanity check I suppose.
        if self.cur_index >= 64
        {
            return None;
        }
        // Check cur_index bitmask
        let mut bitmask = (*self.bitboard >> self.cur_index) & LSB_MASK;
        self.cur_index += 1;
        while bitmask.0 == 0
        {
            if self.cur_index >= 64
            {
                return None;
            }
            bitmask = (*self.bitboard >> self.cur_index) & LSB_MASK;
            self.cur_index += 1;
        }
        // Once we find a valid index, convert it to a square and return it.
        // We subtract one since the current index is 1 higher than the valid bitmask we found.
        Some(Bitboard::index_to_coords_unchecked(self.cur_index - 1))
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_no_valid_squares()
    {
        let bitboard = Bitboard::new(0);
        let squares: Vec<Square> = bitboard.squares().collect();
        assert!(squares.is_empty());
    }

    #[test]
    fn test_all_valid_squares()
    {
        let bitboard = Bitboard::new(u64::MAX);
        let squares: Vec<Square> = bitboard.squares().collect();
        assert_eq!(squares.len(), 64);
    }

    #[test]
    fn test_one_valid_square()
    {
        let bitboard = Bitboard::new(1);
        let squares: Vec<Square> = bitboard.squares().collect();
        assert_eq!(squares.len(), 1);
        assert_eq!(squares[0], Square::new(0, 0));
    }

    #[test]
    fn test_one_valid_square_2()
    {
        let bitboard = Bitboard::new(0x80_00_00_00_00_00_00_00);
        let squares: Vec<Square> = bitboard.squares().collect();
        assert_eq!(squares.len(), 1);
        assert_eq!(squares[0], Square::new(7, 7));
    }

    #[test]
    fn test_one_valid_square_3()
    {
        let bitboard = Bitboard::new(0x01_00_00_00_00_00_00_00);
        let squares: Vec<Square> = bitboard.squares().collect();
        assert_eq!(squares.len(), 1);
        assert_eq!(squares[0], Square::new(7, 0));
    }

    #[test]
    fn test_full_rank()
    {
        let bitboard = Bitboard::rank_mask(0);
        let squares: Vec<Square> = bitboard.squares().collect();
        let mut cur_file: u8 = 0;
        assert_eq!(squares.len(), 8);
        for square in squares.iter()
        {
            assert_eq!(*square, Square::new(0, cur_file));
            cur_file += 1;
        }
    }

    #[test]
    fn test_full_file()
    {
        let bitboard = Bitboard::file_mask(0);
        let squares: Vec<Square> = bitboard.squares().collect();
        let mut cur_rank: u8 = 0;
        assert_eq!(squares.len(), 8);
        for square in squares.iter()
        {
            assert_eq!(*square, Square::new(cur_rank, 0));
            cur_rank += 1;
        }
    }
}
