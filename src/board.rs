//! Types and utilities related to storing board state.
//!
//! This module exposes the following types:
//! - [Board] represents a given board: i.e an arrangement of pieces and information of an ongoing
//! or completed game.
//! - [Square] represents a specific square or coordinates on the board.
//! - [PieceType] is an enum representing all the standard chess pieces
//! - [PlayerColor] is an enum representing White and Black colors.
//! - A [Piece] is a combination of a [PieceType] and a [PlayerColor].
//! - A [Move] is used to transition a [Board] from one state into a new state.
//! - A [BoardConfiguration] is used to set up a [Board].
//!
//! # Examples
//!
//! First, create a [Board]. There are a handful of methods to do this:
//! - [Board::new_default_starting_board()] creates a new board with the standard arrangement of
//! pieces.
//! - [Board::new_blank_board()] creates a new board with no pieces on it.
//! - [Board::new_board_with_configuration()] creates a new board with a custom configuration,
//! given by [BoardConfiguration]
//!
//! ```
//! // Default, "standard" board
//! let board = Board::new_default_starting_board();
//! ```
//!
//! If you want to create a custom board, you probably want to use [BoardConfiguration] or
//! [BoardConfigurationBuilder]
//!
//! ```
//! // This creates a board configuration for a "normal" starting board.
//! let board_configuration = BoardConfiguration::default()
//! let board = Board::new_default_starting_board();
//! // BoardConfiguration implements PartialEq, so these two versions
//! // should produce the same board configuration.
//! assert_eq!(board.get_board_configuration(), board_configuration);
//! ```
//!
//! Generally, using a [FEN](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation) record is probably easier to produce a starting board configuration 
//! than building it programatically.
//! ```
//! let board_configuration = BoardConfiguration::from_str("k7/2p2p2/2p2p2/8/1p4p1/2pppp2/8/K7 w - - 0 1");
//! ```
//!
//! If you want to build up a custom board programmatically, [BoardConfigurationBuilder] is
//! probably the easiest and most flexible way to construct it.
//!
//! ```
//! let mut board_configuration_builder = BoardConfigurationBuilder::default();
//! board_configuration_builder
//!     .add_piece(Piece::new(PlayerColor::White, PieceType::Pawn),Square::new(0,0))
//!     .set_active_color(PlayerColor::White);
//!
//! let board_configuration = board_configuration_builder.build();
//! ```

mod line;
mod square;
mod piece_type;
mod piece;
mod r#move;
mod board;
mod board_config;
mod player_color;
mod error;
mod board_result;

pub use line::Line;
pub use square::Square;
pub use piece_type::PieceType;
pub use r#move::Move;
pub use board::Board;
pub use player_color::PlayerColor;
pub use piece::Piece;
pub use board_config::{BoardConfiguration, CastlingAvailability, BoardConfigurationBuilder, InvalidFENError};
pub use board_result::{BoardResult, DrawReason};
pub use error::MoveError;

