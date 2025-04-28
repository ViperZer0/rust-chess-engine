use std::{collections::HashMap, fmt::Display};
use std::str::FromStr;

use thiserror::Error;

use super::{Piece, PieceType, PlayerColor, Square};

/// A specified arrangement of pieces.
///
/// This is equivalent to [FEN](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)
/// thus records the following information:
/// - Piece placement
/// - Active color (whose turn it is)
/// - Whether or not black or white have castled yet
/// - Whether or not a square can be en passant captured
/// - Halfmoves (used to track fifty-move rule)
///     - Number of moves since last capture or pawn advance.
/// - Fullmoves
#[derive(Debug, PartialEq)]
pub struct BoardConfiguration
{
    // I'm not sure if a vec or a hashmap is better here.
    pieces: HashMap<Square, Piece>,
    active_color: PlayerColor,
    castling_availability: CastlingAvailability,
    en_passant_target_square: Option<Square>,
    halfmove_clock: u8,
    fullmove_number: u8,
}

/// This struct records information on who can castle and where.
/// 
/// In chess, castling is only available to a player if:
/// - The king has not moved from its starting square
/// - The rook on the side being castled towards has not moved.
/// 
/// FEN notation does not include information on the previous state of the board,
/// and so has a dedicated field to indicate whether or not the players
/// are allowed to castle or not.
///
/// Note that this doesn't include information on *temporary* scenarios in which castling are
/// prevented. If castling would put the king in check, the option is still available to the king
/// later.
#[derive(Debug, PartialEq, Clone)]
pub struct CastlingAvailability
{
    white_castle_kingside: bool,
    white_castle_queenside: bool,
    black_castle_kingside: bool,
    black_castle_queenside: bool,
}

impl CastlingAvailability
{
    /// Returns a new [CastlingAvailability].
    ///
    /// CastlingAvailability contains four fields which answer the following questions:
    /// - Can the white player castle kingside?
    /// - Can the white player castle queenside?
    /// - Can the black player castle kingside?
    /// - Can the black player castle queenside?
    ///
    /// # Arguments
    ///
    /// * `white_castle_kingside` - Can the white player castle kingside?
    /// * `white_castle_queenside` - Can the white player castle queenside?
    /// * `black_castle_kingside` - Can the black player castle kingside?
    /// * `black_castle_queenside` - Can the black player castle queenside?
    ///
    /// # Examples
    ///
    /// ```
    /// // CastlingAvailability state when the game starts.
    /// let castling_availability = CastlingAvailability::new(true, true, true, true);
    /// // CastlingAvailability state after both kings have moved.
    /// let castling_availability = CastlingAvailability::new(false, false, false, false);
    /// // CastlingAvailability when white's king has moved but black's has not.
    /// let castling_availability = CastlingAvailability::new(false, false, true, true);
    /// 
    /// ```
    pub fn new(white_castle_kingside: bool,
               white_castle_queenside: bool,
               black_castle_kingside: bool,
               black_castle_queenside: bool) -> Self
    {
        CastlingAvailability
        {
            white_castle_kingside,
            white_castle_queenside,
            black_castle_kingside,
            black_castle_queenside
        }
    }
}

impl BoardConfiguration
{
    /// Gets the pieces of the board as a hashmap.
    pub fn get_pieces(&self) -> &HashMap<Square, Piece>
    {
        return &self.pieces;
    }

    /// Creates a new instance of the [BoardConfiguration].
    ///
    /// Note that, as with [BoardConfiguration] in general, all fields here are initialized as they
    /// are provided, with no logic being done to check if the configuration given is valid.
    ///
    /// You can totally make a board with no kings, and that's on you.
    ///
    /// This method is extremely verbose and requires every argument to be specified.
    /// If you want to use unspecified default arguments, consider creating a [BoardConfiguration]
    /// from a [BoardConfigurationBuilder] instead.
    ///
    /// # Arguments
    ///
    /// * `pieces` - The hashmap of pieces. The keys are the squares which the pieces occupy, and
    /// the values are the respective pieces.
    /// * `active_color` - Whose turn it is.
    /// * `castling_availability` - Which players can castle, and where.
    /// * `en_passant_target_square` - Whether or not there was en passant last turn and where.
    /// * `halfmove_clock` - The nuumber of single turns ([ply](https://en.wikipedia.org/wiki/Ply_(game_theory))) since the last capture or pawn advance.
    /// * `fullmove_number` - The number of full moves. Starts at 1, increments after Black's move.
    ///
    /// # Examples
    ///
    /// ```
    /// // This is more or less the bare minimum arguments to create a board configuration.
    /// // As mentioned above, you may want to consider BoardConfigurationBuilder instead.
    /// let board_config = BoardConfiguration::new(HashMap::new(), PlayerColor::White,
    /// CastlingAvailability::default(), None, 0, 1);
    /// ```
    pub fn new(pieces: HashMap<Square, Piece>, active_color: PlayerColor, castling_availability: CastlingAvailability, en_passant_target_square: Option<Square>, halfmove_clock: u8, fullmove_number: u8) -> Self
    {
        Self
        {
            pieces,
            active_color,
            castling_availability,
            en_passant_target_square,
            halfmove_clock,
            fullmove_number,
        }
    }
}

impl Display for BoardConfiguration
{
    // This will print out the FEN notation of a board configuration.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Default for BoardConfiguration
{
    /// Creates a [BoardConfiguration] with the "standard" chess configuration:
    /// i.e 16 pieces on each side, 8 pawns, 2 rooks, knights, bishops, one queen and king.
    /// 
    ///
    /// # Examples
    ///
    /// ```
    /// let board_config = BoardConfiguration::default();
    /// ```
    fn default() -> Self {
        Self::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").expect("The provided default board configuration was invalid.")
    }
}

/// Represents an error returned by [BoardConfiguration::from_str].
#[derive(Debug, Error)]
pub enum InvalidFENError
{
    /// This error variant is returned when the FEN record is missing a required section.
    #[error("Missing field {0} from FEN notated string {1}")]
    MissingFENSection(u8, String),
    /// This error variant is returned when the first field (piece placement data) of a FEN has too few ranks specified.
    #[error("The FEN notated string {0} had too few specified ranks!")]
    TooFewRanks(String),
    /// This error variant is returned when the first field (piece placement data) of a FEN has too many ranks specified.
    #[error("The FEN notated string {1} had too many specified ranks! (Remainder: {0})")]
    TooManyRanks(String, String),
    /// This error variant is returned when the first field (piece placement data) of a FEN has too few files specified.
    #[error("There were too few files ({1} < 7) on rank {0} in the FEN record {2}")]
    TooFewFiles(u8, u8, String),
    /// This error variant is returned when the first field (piece placement data) of a FEN has too many files specified.
    #[error("There were too many files ({1} >= 8) on rank {0} in the FEN record {2}")]
    TooManyFiles(u8, u8, String),
    /// This error variant is returned when the first field (piece placement data) of a FEN has an invalid character.
    #[error("An invalid character for a piece type was provided: {0} in {1}")]
    InvalidPieceCharacter(String, String),
    /// This error variant is returned when the second field (active color) of a FEN has an invalid character.
    #[error("{0} was not a valid character for the active color in the string {1}")]
    InvalidTurnCharacter(String, String),
    /// This error variant is returned when the third field (castling availability) of a FEN has an invalid character.
    #[error("Invalid castling character {0} in FEN string {1}")]
    InvalidCastlingCharacter(String, String),
    /// This error variant is returned when the fourth field (en passant target square) of a FEN has an invalid character.
    #[error("Invalid en passant target square notation {0} in FEN string {1}")]
    InvalidEnPassantTargetSquare(String, String),
    /// This error variant is returned when the fifth field (halfmove clock) of a FEN has an invalid character, or isn't a number.
    #[error("Halfmove clock field was not a number: {0}")]
    InvalidHalfMoveClock(String),
    /// This error variant is returned when the sixth field (fullmove number) of a FEN has an invalid character, or isn't a number.
    #[error("Fullmove number field was not a number: {0}")]
    InvalidFullMoveNumber(String)
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
    /// Any invalid FEN record will return an error. A FEN record can be invalid for many reasons,
    /// including:
    /// - Missing one of the required fields
    /// - Having an invalid piece type character present
    /// - Having a badly formatted square coordinate
    /// - And other such formatting errors.
    /// 
    /// The errors returned will be of type [InvalidFENError]
    ///
    /// # Examples
    ///
    /// ```
    /// [TODO:write some example code]
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Get an iterator over the parts of the FEN string.
        let mut iter = s.split_ascii_whitespace();

        let pieces = iter.next().ok_or_else(|| InvalidFENError::MissingFENSection(0, s.to_string()))?;
        let turn = iter.next().ok_or_else(|| InvalidFENError::MissingFENSection(1, s.to_string()))?;
        let castling = iter.next().ok_or_else(|| InvalidFENError::MissingFENSection(2, s.to_string()))?;
        let en_passant = iter.next().ok_or_else(|| InvalidFENError::MissingFENSection(3, s.to_string()))?;
        let half_move_clock = iter.next().ok_or_else(|| InvalidFENError::MissingFENSection(4, s.to_string()))?;
        let full_move_number = iter.next().ok_or_else(|| InvalidFENError::MissingFENSection(5, s.to_string()))?;

        let piece_map = parse_pieces(pieces)?;
        let turn = match turn
        {
            "w" => PlayerColor::White,
            "b" => PlayerColor::Black,
            _ => return Err(InvalidFENError::InvalidTurnCharacter(turn.to_string(), s.to_string())),
        };

        let castling: CastlingAvailability = castling.parse()?;
        let en_passant: Option<Square> = match en_passant
        {
            "-" => None,
            x => Some(x.parse().map_err(|_| InvalidFENError::InvalidEnPassantTargetSquare(x.to_string(), s.to_string()))?)
        };

        let half_move_clock: u8 = half_move_clock.parse().map_err(|_| InvalidFENError::InvalidHalfMoveClock(half_move_clock.to_string()))?;
        let full_move_number: u8 = full_move_number.parse().map_err(|_| InvalidFENError::InvalidFullMoveNumber(full_move_number.to_string()))?;

        Ok(Self
        {
            pieces: piece_map,
            active_color: turn,
            castling_availability: castling,
            en_passant_target_square: en_passant,
            halfmove_clock: half_move_clock,
            fullmove_number: full_move_number,
        })
    }
}


/// Parses the section of the FEN responsible for describing the piece positions on the board
/// and returns a HashMap<Square, Piece>, where the key is the square on which a piece rests,
/// and the value is the piece itself.
///
/// # Arguments
///
/// * `s` - The input FEN notated string. This should be only the first field of a full FEN notated
/// string. / is expected as a delimiter, kqrbnp/KQRBNP are expected as valid piece types,
/// and numbers are expected for empty squares. See [FEN](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation) for more information.
///
/// # Errors
///
/// An [InvalidFENError] is returned if the FEN notation is badly formatted for any reason. Parsing
/// the pieces can fail if there are too few or too many ranks, or if an unexpected character was
/// found in the string.
///
/// # Examples
///
/// ```
/// [TODO:write some example code]
/// ```
fn parse_pieces(s: &str) -> Result<HashMap<Square, Piece>, InvalidFENError>
{
    let mut map = HashMap::new();
    let mut iter = s.split("/");
    // I don't know if it's a good idea or not to hard code this like this?
    for rank in (0..8).rev()
    {
        let mut current_file = 0;
        let current_rank_string = iter.next().ok_or_else(|| InvalidFENError::TooFewRanks(s.to_string()))?;
        for char in current_rank_string.chars()
        {
            if current_file >= 8
            {
                return Err(InvalidFENError::TooManyFiles(rank, current_file, s.to_string()));
            }

            // If the character is a number, we insert that many empty squares before parsing
            // the next piece.
            if char.is_ascii_digit()
            {
                let num_blank_spaces: u8 = char.to_digit(10).expect("to_digit() failed after asserting that char.is_ascii_digit()").try_into().expect("Somehow converting a single digit u32 to a u8 failed???");
                current_file += num_blank_spaces;
                continue;
            }

            // Otherwise we assume it is a piece type.
            // Upper case pieces are white, lowercase is black
            let piece_color = match char.is_uppercase()
            {
                true => PlayerColor::White,
                false => PlayerColor::Black,
            };

            let piece_type: PieceType = char.to_string().parse().map_err(|_| InvalidFENError::InvalidPieceCharacter(char.to_string(), s.to_string()))?;

            // Create the new piece and add it to the board.
            let square = Square::new(rank, current_file);
            let piece = Piece::new(piece_color, piece_type);
            map.insert(square, piece);
            // Move to next square
            current_file += 1;
        }

        // If we didn't reach the end of the board, return an error.
        if current_file < 8
        {
            return Err(InvalidFENError::TooFewFiles(rank, current_file, s.to_string()));
        }
    }

    // If we still have stuff left over, that means we had too many ranks.
    let next = iter.next();
    if next.is_some()
    {
        return Err(InvalidFENError::TooManyRanks(next.unwrap().to_string(), s.to_string()))
    }

    Ok(map)
}

impl FromStr for CastlingAvailability
{
    type Err = InvalidFENError;

    /// Parses the part of the
    /// [FEN](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation) responsible
    /// for determining castling availability. The short of it is:
    ///
    /// - K means White can castle kingside
    /// - Q means White can castle queenside
    /// - k means Black can castle kingside
    /// - q means Black can castle kingside
    /// - "-" means no one can castle.
    ///
    /// So on a new board where everybody can castle, the input string to this method would be
    /// KQkq.
    ///
    /// If Black has castled kingside, and White has moved their kingside rook (so they can only
    /// castle queenside), the string would be Q.
    ///
    /// If no one can castle, this field is just given as "-".
    ///
    /// Techincally this method internally just discards "-", so KQkq- is still valid and will be
    /// treated the same as KQkq, but this is not guaranteed to work now or in the future, so avoid
    /// creating FEN records with atypical syntax.
    ///
    /// This method is primarily intended to be called from [BoardConfiguration::from_str]. See
    /// that method for parsing the entire FEN.
    ///
    /// # Arguments
    ///
    /// * `s` - The input string, see above for the specific notation to use.
    ///
    /// # Errors
    ///
    /// This method returns an [InvalidFENError::InvalidCastlingCharacter] if a character in
    /// the input string is not one of {K, Q, k, q, -}.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// // Default castling availability
    /// let castling_availability = CastlingAvailability::from_str("KQkq");
    /// // White has castled, Black has not.
    /// let castling_availability = CastlingAvailability::from_str("kq");
    /// // No one can castle.
    /// let castling_availability = CastlingAvailability::from_str("-");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut white_castle_kingside: bool = false;
        let mut white_castle_queenside: bool = false;
        let mut black_castle_kingside: bool = false;
        let mut black_castle_queenside: bool = false;

        for char in s.chars()
        {
            match char
            {
                'K' => white_castle_kingside = true,
                'Q' => white_castle_queenside = true,
                'k' => black_castle_kingside = true,
                'q' => black_castle_queenside = true,
                // We can basically ignore this character, it just means nobody can castle.
                '-' => (),
                _ => return Err(InvalidFENError::InvalidCastlingCharacter(char.to_string(), s.to_string())),
            };
        }

        Ok(CastlingAvailability::new(white_castle_kingside, white_castle_queenside, black_castle_kingside, black_castle_queenside))
    }
}

/// Helper builder for constructing a [BoardConfiguration], since they have a lot of information
/// included with them.
///
/// You may be interested in more easily constructing a [BoardConfiguration] directly using
/// something like [BoardConfiguration::from_str] with FEN or [BoardConfiguration::default].
///
/// This builder is more appropriate for building up heavily customized boards programatically.
#[derive(Default, Clone)]
pub struct BoardConfigurationBuilder
{
    pieces: Option<HashMap<Square, Piece>>,
    active_color: Option<PlayerColor>,
    castling_availability: Option<CastlingAvailability>,
    en_passant_target_square: Option<Square>,
    halfmove_clock: Option<u8>,
    fullmove_number: Option<u8>,
}

impl BoardConfigurationBuilder
{
    /// Sets the list of pieces included with the board.
    ///
    /// This will replace any previously added pieces, creating an entirely new map of pieces.
    /// The input is a [HashMap] of [Square] to [Piece], where each [Piece] is located on the given
    /// [Square] on the board.
    ///
    /// This method is optional, as is [Self::add_piece], and will default to a blank board if no
    /// pieces are specified.
    pub fn set_pieces(mut self, pieces: HashMap<Square, Piece>) -> Self
    {
        // Get existing hashmap or create a new one
        self.pieces = Some(pieces);
        self
    }

    /// Adds a new piece to the list of pieces on the board.
    ///
    /// This will add to the existing set of pieces, unlike [Self::set_pieces]. If a square
    /// currently has a piece on it, it will be replaced with the new provided piece.
    /// 
    /// This method is optional, as is [Self::set_pieces], and will default to a blank board if no
    /// pieces are specified.
    pub fn add_piece(mut self, piece: Piece, square: Square) -> Self
    {
        // Construct new hashmap if one doesn't exist yet.
        let mut hashmap = self.pieces.get_or_insert_with(|| HashMap::new());
        // Add the new piece
        hashmap.insert(square, piece);
        self
    }

    /// Sets the active player's turn.
    ///
    /// This method is optional, if a [BoardConfiguration] is built without this being set it will
    /// default to [PlayerColor::White].
    pub fn set_active_color(mut self, color: PlayerColor) -> Self
    {
        self.active_color = Some(color);
        self
    }

    /// Sets what directions are available for each player to castle in.
    ///
    /// If this method is not called, it will default to having both queenside and kingside castles
    /// available to both black and white.
    pub fn set_castling_availability(mut self, castling_availability: CastlingAvailability) -> Self
    {
        self.castling_availability = Some(castling_availability);
        self
    }

    /// Sets whether or not a previous move made it possible to capture en passant.
    ///
    /// If this method is not called, it will default to no en passant being available.
    pub fn set_en_passant_target_square(mut self, square: Square) -> Self
    {
        self.en_passant_target_square = Some(square);
        self
    }

    /// Sets how many moves (from either player) have passed since the last capture or pawn
    /// advance.
    ///
    /// If this method is not called, it will default to 0.
    pub fn set_halfmove_clock(mut self, halfmove_clock: u8) -> Self
    {
        self.halfmove_clock = Some(halfmove_clock);
        self
    }

    /// Sets how many total moves (1 for each 2 turns).
    ///
    /// Defaults to 1.
    pub fn set_fullmove_number(mut self, fullmove_number: u8) -> Self
    {
        self.fullmove_number = Some(fullmove_number);
        self
    }

    /// Builds a new [BoardConfiguration] given the provided parameters.
    ///
    /// This will consume the BoardConfigurationBuilder, since the [BoardConfiguration]
    /// will take ownership of the underlying HashMap storing the arrangement of pieces.
    pub fn build(self) -> BoardConfiguration
    {
        BoardConfiguration { 
            pieces: self.pieces.unwrap_or_else(|| HashMap::new()),
            active_color: self.active_color.unwrap_or(PlayerColor::White),
            castling_availability: self.castling_availability.unwrap_or_else(|| CastlingAvailability::new(true, true, true, true)),
            en_passant_target_square: self.en_passant_target_square,
            halfmove_clock: self.halfmove_clock.unwrap_or(0),
            fullmove_number: self.fullmove_number.unwrap_or(1)
        }
    }
}


#[cfg(test)]
mod tests
{
    use std::collections::HashMap;
    use crate::board::{Piece, PieceType, PlayerColor, Square};
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
