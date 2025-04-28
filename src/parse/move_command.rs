//! Specifies the [MoveCommand] type.

use std::str::FromStr;
use regex::RegexBuilder;

use crate::board::{Line, PieceType, Square};

use super::NotationParseError;

/// Represents a parsed move from algebraic notation
///
/// This type guarantees that the contained move is sound (meaning the given
/// notation was gramatically correct) but does not check if the move is valid (does not break
/// movement rules, rules about escaping check, etc.)
/// 
/// This type covers the three main move types in chess:
/// - A kingside castle (O-O)
/// - A queenside castle (O-O-O)
/// - Basically everything else.
#[derive(Debug)]
pub enum MoveCommand
{
    /// A "normal" chess move, which is basically any move that isn't a castle.
    NormalMove(MoveCommandData),
    /// A kingside castle (notated as O-O)
    KingsideCastle,
    /// A queenside castle (notated as O-O-O)
    QueensideCastle,
}

impl MoveCommand
{
    /// Returns true when this move is *not* a castle.
    /// So basically other move, including captures.
    pub fn is_normal_move(&self) -> bool
    {
        match self
        {
            MoveCommand::NormalMove(_) => true,
            MoveCommand::KingsideCastle => false,
            MoveCommand::QueensideCastle => false,
        }
    }

    /// Returns true if this move is a castling move.
    pub fn is_castle(&self) -> bool
    {
        match self
        {
            MoveCommand::NormalMove(_) => false,
            MoveCommand::KingsideCastle => true,
            MoveCommand::QueensideCastle => true,
        }
    }

    /// Returns true if this move is a kingside castle,
    /// false if it's a queenside castle or a "normal" move.
    pub fn is_kingside_castle(&self) -> bool
    {
        match self
        {
            MoveCommand::NormalMove(_) => false,
            MoveCommand::QueensideCastle => false,
            MoveCommand::KingsideCastle => true,
        }
    }

    /// Returns true if this move is a queenside castle,
    /// false if it's a kingside castle or a "normal" move.
    pub fn is_queenside_castle(&self) -> bool
    {
        match self
        {
            MoveCommand::NormalMove(_) => false,
            MoveCommand::KingsideCastle => false,
            MoveCommand::QueensideCastle => true,
        }
    }

    /// Returns [Some] containing a [MoveCommandData] if this move is a normal move.
    /// If this move is a castle, it has no associated [MoveCommandData] so returns [None].
    pub fn get_move_data(&self) -> Option<MoveCommandData>
    {
        match self
        {
            MoveCommand::NormalMove(x) => Some(*x),
            MoveCommand::KingsideCastle => None,
            MoveCommand::QueensideCastle => None,
        }
    }

}

/// Represents the information included with what we might consider to be a 
/// "normal" move that follows standard algebraic notation.
/// This includes moves and captures from all pawns and minor pieces,
/// with the only real exceptions being to castle (which is notated as O-O or O-O-O).
/// 
/// This data is basically 1-to-1 with the information included in a notated move.
///
/// For example, the move Bxa3 would be split up as follows:
/// - The piece type is a bishop
/// - Capture is true
/// - Target square is a3.
///
/// A MoveCommand can also handle a discriminant, i.e Nca3 would mean:
/// - The piece type is a knight
/// - Between the two knights, the one on the c file is the one being moved (discriminant)
/// - Capture is false
/// - Target square is a3.
///
/// There are some standard notation elements that are ignored: namely it is common
/// to indicate whether a move placed a king in check by suffixing the move with a "+".
/// This doesn't actually add any new information, however, so is ignored in our parser
/// as it might make it harder for beginners to input moves.
#[derive(Debug, Copy, Clone)]
pub struct MoveCommandData
{
    /// What piece type is being moved. Defaults to pawn if no letter was specified. 
    piece_type: PieceType,
    /// An optional filter that distinguishes between multiple pieces, if more than one 
    /// is allowed to move to the same location. 
    discriminant: Option<Line>,
    /// Whether or not this move is a capture.
    capture: bool,
    /// The destination square.
    target_square: Square,
}

impl FromStr for MoveCommand
{
    type Err = NotationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
        {
            "o-o" | "O-O" => Ok(Self::KingsideCastle),
            "o-o-o" | "O-O-O" => Ok(Self::QueensideCastle),
            other => Ok(Self::NormalMove(MoveCommandData::from_str(other)?)),
        }
    }
}

impl FromStr for MoveCommandData
{
    type Err = NotationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let re = RegexBuilder::new(r"(?<piece>[nbrqk]?)(?<discriminant>[a-h]?[0-9]?)?(?<capture>x?)(?<destination>[a-h][0-9])")
            .case_insensitive(true)
            .build()
            .expect("Invalid regex");

        let captures = re.captures(s);
        if captures.is_none()
        {
            return Err(NotationParseError::InvalidFormat(s.to_string()));
        }

        let captures = captures.unwrap();
        // This should work even if the capture group is an empty string???
        let piece_type = captures.name("piece").unwrap().as_str();
        let discriminant = captures.name("discriminant").unwrap().as_str();
        let capture = !captures.name("capture").unwrap().is_empty();
        let destination = captures.name("destination").unwrap().as_str();

        let piece_type = PieceType::from_str(piece_type)?;
        let discriminant: Option<Line> = match discriminant.is_empty()
        {
            true => None,
            false => Some(Line::from_str(discriminant)?),
        };
        let destination = Square::from_str(destination)?;

        Ok(Self
        {
            piece_type,
            discriminant,
            capture,
            target_square: destination,
        })
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_kingside_castle_notation_works()
    {
        assert!(MoveCommand::from_str("o-o").unwrap().is_kingside_castle());
        assert!(MoveCommand::from_str("O-O").unwrap().is_kingside_castle());
        assert!(!MoveCommand::from_str("o-o").unwrap().is_queenside_castle());
        assert!(!MoveCommand::from_str("O-O").unwrap().is_queenside_castle());
        assert!(!MoveCommand::from_str("o-o").unwrap().is_normal_move());
        assert!(!MoveCommand::from_str("O-O").unwrap().is_normal_move());
    }

    #[test]
    fn test_queenside_castle_notation_works()
    {
        assert!(MoveCommand::from_str("o-o-o").unwrap().is_queenside_castle());
        assert!(MoveCommand::from_str("O-O-O").unwrap().is_queenside_castle());
        assert!(!MoveCommand::from_str("o-o-o").unwrap().is_kingside_castle());
        assert!(!MoveCommand::from_str("O-O-O").unwrap().is_kingside_castle());
        assert!(!MoveCommand::from_str("o-o-o").unwrap().is_normal_move());
        assert!(!MoveCommand::from_str("O-O-O").unwrap().is_normal_move());
    }

    #[test]
    fn test_pawn_move_command()
    {
        let move_command = MoveCommand::from_str("e4").unwrap();
        match move_command
        {
            MoveCommand::NormalMove(move_data) => {
                assert_eq!(PieceType::Pawn, move_data.piece_type);
                assert!(move_data.discriminant.is_none());
                assert!(!move_data.capture);
                assert_eq!(Square::new(3, 4), move_data.target_square);
            }
            MoveCommand::KingsideCastle => assert!(false, "Expected NormalMove, got KingsideCastle"),
            MoveCommand::QueensideCastle => assert!(false, "Expected NormalMove, got QueensideCastle"),
        }
    }

    #[test]
    fn test_piece_move_command()
    {
        let move_command = MoveCommand::from_str("Bh8").unwrap();
        assert!(move_command.is_normal_move());
        let move_data = move_command.get_move_data().unwrap();
        assert_eq!(PieceType::Bishop, move_data.piece_type);
        assert!(move_data.discriminant.is_none());
        assert!(!move_data.capture);
        assert_eq!(Square::new(7, 7), move_data.target_square);
    }

    #[test]
    fn test_capture_with_piece()
    {
        let move_command = MoveCommand::from_str("Rxa1").unwrap();
        assert!(move_command.is_normal_move());
        let move_data = move_command.get_move_data().unwrap();
        assert_eq!(PieceType::Rook, move_data.piece_type);
        assert!(move_data.discriminant.is_none());
        assert!(move_data.capture);
        assert_eq!(Square::new(0, 0), move_data.target_square);
    }

    #[test]
    fn test_capture_with_pawn()
    {
        let move_command = MoveCommand::from_str("exd5").unwrap();
        assert!(move_command.is_normal_move());
        let move_data = move_command.get_move_data().unwrap();
        assert_eq!(PieceType::Pawn, move_data.piece_type);
        assert!(move_data.discriminant.is_some());
        assert!(move_data.capture);
        assert_eq!(Line::File(4), move_data.discriminant.unwrap());
    }

    #[test]
    fn test_discriminant_file()
    {
        let move_command = MoveCommand::from_str("Nge2").unwrap();
        assert!(move_command.is_normal_move());
        let move_data = move_command.get_move_data().unwrap();
        assert_eq!(PieceType::Knight, move_data.piece_type);
        assert!(move_data.discriminant.is_some());
        assert!(!move_data.capture);
        assert_eq!(Line::File(6), move_data.discriminant.unwrap());
    }

    #[test]
    fn test_discriminant_rank()
    {
        let move_command = MoveCommand::from_str("N1e2").unwrap();
        assert!(move_command.is_normal_move());
        let move_data = move_command.get_move_data().unwrap();
        assert_eq!(PieceType::Knight, move_data.piece_type);
        assert!(move_data.discriminant.is_some());
        assert!(!move_data.capture);
        assert_eq!(Line::Rank(0), move_data.discriminant.unwrap());
    }

    #[test]
    fn test_discriminant_both()
    {
        let move_command = MoveCommand::from_str("Qh4e1").unwrap();
        assert!(move_command.is_normal_move());
        let move_data = move_command.get_move_data().unwrap();
        assert_eq!(PieceType::Queen, move_data.piece_type);
        assert!(move_data.discriminant.is_some());
        assert!(!move_data.capture);
        assert_eq!(Line::RankAndFile(3, 7), move_data.discriminant.unwrap());
    }

    #[test]
    fn test_longest_notation()
    {
        let move_command = MoveCommand::from_str("Qh4xe1").unwrap();
        assert!(move_command.is_normal_move());
        let move_data = move_command.get_move_data().unwrap();
        assert_eq!(PieceType::Queen, move_data.piece_type);
        assert!(move_data.discriminant.is_some());
        assert!(move_data.capture);
        assert_eq!(Line::RankAndFile(3, 7), move_data.discriminant.unwrap());
    }
}

