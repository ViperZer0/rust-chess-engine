use std::str::FromStr;
use thiserror::Error;

use crate::board::BoardConfiguration;

/// Represents an error returned by [BoardConfiguration::from_str].
#[derive(Debug, Error)]
pub enum InvalidFENError
{
}

impl FromStr for BoardConfiguration
{
    type Err = InvalidFENError;

    /// Takes in a string of valid [FEN](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)
    /// and generates a correct BoardConfiguration for it.
    /// # Arguments
    ///
    /// * `s` - A valid FEN string.
    ///
    /// # Errors
    ///
    /// [TODO:describe error types and what triggers them]
    ///
    /// # Examples
    ///
    /// ```
    /// [TODO:write some example code]
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[cfg(test)]
mod tests
{
    use std::{collections::HashMap, hash::Hash};
    use crate::board::{Board, Piece, PieceType, PlayerColor, Square};
    use super::*;

    fn add_all_pieces_to_map(hashmap: &mut HashMap<Square, Piece>)
    {
        add_white_pieces_to_map(hashmap);
        add_black_pieces_to_map(hashmap);
    }

    fn add_white_pieces_to_map(hashmap: &mut HashMap<Square, Piece>)
    {
        use PieceType::*;
        use PlayerColor::White;
        hashmap.insert(Square::new(0, 0), Piece::new(White, Rook));
        hashmap.insert(Square::new(0, 1), Piece::new(White, Knight));
        hashmap.insert(Square::new(0, 2), Piece::new(White, Bishop));
        hashmap.insert(Square::new(0, 3), Piece::new(White, Queen));
        hashmap.insert(Square::new(0, 4), Piece::new(White, King));
        hashmap.insert(Square::new(0, 5), Piece::new(White, Bishop));
        hashmap.insert(Square::new(0, 6), Piece::new(White, Knight));
        hashmap.insert(Square::new(0, 7), Piece::new(White, Rook));
        hashmap.insert(Square::new(1, 0), Piece::new(White, Pawn));
        hashmap.insert(Square::new(1, 1), Piece::new(White, Pawn));
        hashmap.insert(Square::new(1, 2), Piece::new(White, Pawn));
        hashmap.insert(Square::new(1, 3), Piece::new(White, Pawn));
        hashmap.insert(Square::new(1, 4), Piece::new(White, Pawn));
        hashmap.insert(Square::new(1, 5), Piece::new(White, Pawn));
        hashmap.insert(Square::new(1, 6), Piece::new(White, Pawn));
        hashmap.insert(Square::new(1, 7), Piece::new(White, Pawn));
    }

    fn add_black_pieces_to_map(hashmap: &mut HashMap<Square, Piece>)
    {
        use PieceType::*;
        use PlayerColor::Black;
        hashmap.insert(Square::new(7, 0), Piece::new(Black, Rook));
        hashmap.insert(Square::new(7, 1), Piece::new(Black, Knight));
        hashmap.insert(Square::new(7, 2), Piece::new(Black, Bishop));
        hashmap.insert(Square::new(7, 3), Piece::new(Black, Queen));
        hashmap.insert(Square::new(7, 4), Piece::new(Black, King));
        hashmap.insert(Square::new(7, 5), Piece::new(Black, Bishop));
        hashmap.insert(Square::new(7, 6), Piece::new(Black, Knight));
        hashmap.insert(Square::new(7, 7), Piece::new(Black, Rook));
        hashmap.insert(Square::new(6, 0), Piece::new(Black, Pawn));
        hashmap.insert(Square::new(6, 1), Piece::new(Black, Pawn));
        hashmap.insert(Square::new(6, 2), Piece::new(Black, Pawn));
        hashmap.insert(Square::new(6, 3), Piece::new(Black, Pawn));
        hashmap.insert(Square::new(6, 4), Piece::new(Black, Pawn));
        hashmap.insert(Square::new(6, 5), Piece::new(Black, Pawn));
        hashmap.insert(Square::new(6, 6), Piece::new(Black, Pawn));
        hashmap.insert(Square::new(6, 7), Piece::new(Black, Pawn));
    }

    #[test]
    fn test_default_configuration()
    {
        let board_config = BoardConfiguration::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let mut hashmap: HashMap<Square, Piece> = HashMap::new();
        add_all_pieces_to_map(&mut hashmap);
        assert_eq!(*board_config.get_pieces(), hashmap);
    }

    #[test]
    fn test_empty_configuration()
    {
        let board_config = BoardConfiguration::from_str("8/8/8/8/8/8/8/8 w KQkq - 0 1").unwrap();
        let hashmap: HashMap<Square, Piece> = HashMap::new();
        assert_eq!(*board_config.get_pieces(), hashmap);
    }

    #[test]
    fn test_invalid_configuration()
    {
        let board_config = BoardConfiguration::from_str("");
        assert!(board_config.is_err());
        let board_config = BoardConfiguration::from_str("asdkjfhaskldjfh");
        assert!(board_config.is_err());
        let board_config = BoardConfiguration::from_str("8/8/8/7/8/8/8/8 w KQkq - 0 1");
        assert!(board_config.is_err());
        let board_config = BoardConfiguration::from_str("rnbqkbnF/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KWkq - 0 1");
        assert!(board_config.is_err());
    }
}
