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
pub enum MoveCommand {
    /// A "normal" chess move, which is basically any move that isn't a castle.
    NormalMove(MoveData),
    /// A kingside castle (notated as O-O)
    KingsideCastle,
    /// A queenside castle (notated as O-O-O)
    QueensideCastle,
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
#[derive(Debug)]
pub struct MoveData
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
            other => Ok(Self::NormalMove(MoveData::from_str(other)?)),
        }
    }
}

impl FromStr for MoveData
{
    type Err = NotationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let re = RegexBuilder::new(r"(?<piece>[nbrqk]?)(?<discriminant>[a-z]*?[0-9]*?)(?<capture>x?)(?<destination>[a-z][0-9])")
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
        let capture = captures.name("capture").unwrap().is_empty();
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
