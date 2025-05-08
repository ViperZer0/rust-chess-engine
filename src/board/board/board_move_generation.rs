//! This submodule contains functions to generate possible moves. This is used both for computer
//! agents for evaluation and to check for end-game positions, since checkmate or stalemate
//! both occur when there are no possible moves.

use crate::{bitboard::Bitboard, board::{r#move::MoveData, Move, PieceType, PlayerColor, Square}};

use super::Board;

impl Board
{
    /// Generates a bitboard for all possible moves for a piece on a square.
    /// If there is no piece at that square, this function returns [None].
    fn generate_possible_moves_for_piece(&self, player_color: PlayerColor, square: Square) -> Option<Bitboard>
    {
        let piece = self.piece_at(&square)?;
        Some(match piece.piece_type()
        {
            PieceType::Pawn => self.pawn_moves(player_color, square) | self.pawn_attacks(player_color, square),
            PieceType::Knight => self.knight_moves(player_color, square),
            PieceType::Bishop => self.bishop_moves(player_color, square),
            PieceType::Rook => self.rook_moves(player_color, square),
            PieceType::Queen => self.queen_moves(player_color, square),
            PieceType::King => self.king_moves(player_color, square),
        })
    }

    /// Returns a [Vec] containing all legal moves for a specific piece/square.
    ///
    /// Legal moves are moves that are both possible (following piece movement rules) but also
    /// don't violate the rules of chess, namely leaving your king in check.
    ///
    /// # Arguments
    ///
    /// * `player_color` - The player who is doing the moving
    /// * `starting_square` - The square that the piece is on. If there is no piece on the starting
    /// square, the resulting [Vec] will be empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{Board, PlayerColor, Square};
    /// let board = Board::new_default_starting_board();
    /// let pawn_moves = board.generate_legal_moves_for_piece(PlayerColor::White, Square::new(1, 0));
    /// assert_eq!(pawn_moves.len(), 2);
    /// ```
    pub fn generate_legal_moves_for_piece(&self, player_color: PlayerColor, starting_square: Square) -> Vec<Move>
    {
        self.generate_possible_moves_for_piece(player_color, starting_square)
            .into_iter()
            .flat_map(|bb| bb.squares().collect::<Vec<_>>())
            .map(|square| 
                Move::NormalMove(
                    MoveData::new(
                        starting_square, square, 
                        // We are basically assuming that since the bitboard
                        // filtered out moves where we collide with our own pieces,
                        // if there exists any piece at the target square,
                        // that must be a capture, otherwise it's just a move.
                        self.piece_at(&square).is_some()
                    )
                )
            )
            .filter(|m| self.check_move(m))
            .collect()
    }

    /// Returns a [Vec] containing all possible, *legal* moves a player can make
    /// with any of their pieces.
    ///
    /// # Arguments
    ///
    /// * `player_color` - The side moving.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::{Board, PlayerColor};
    /// let board = Board::new_default_starting_board();
    /// let all_moves = board.generate_moves_for_side(PlayerColor::White);
    /// // White starts with 20 possible moves, 16 pawn moves and 4 knight moves.
    /// assert_eq!(all_moves.len(), 20);
    /// ```
    pub fn generate_moves_for_side(&self, player_color: PlayerColor) -> Vec<Move>
    {
        let mut moves = Vec::new();
        for square in self.query().color(player_color).result().squares()
        {
            moves.extend(self.generate_legal_moves_for_piece(player_color, square))
        }

        moves
    }
}
