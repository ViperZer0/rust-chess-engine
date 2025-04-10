//! Functions and utilities related to parsing algebraically notated chess commands.
//!
//! # A Brief Guide to Algebraic Notation
//! There exists a standard notation for how chess moves are described. 
//! A series of these notated "commands" can be used to faithfully recreate a game of chess,
//! and is often used by chess players to input their moves.
//!
//! A chess move consists of the following sections:
//! - The piece type being moved, represented a single letter (defaults to a pawn if unspecified).
//! - A *discriminant*, which is used when two or more pieces could make a valid move with this
//! command. The discriminant is either the distinguishing rank or file of the two pieces as a
//! single letter or number.
//! - Whether or not this move was a capture, represented by the letter "x". This can sometimes be
//! a colon or a different symbol, but here we always expect a standard ASCII x.
//! - The destination square, described as a letter-number pair representing the file and rank
//! respectively of the final square.
//!
//! A normal move looks something like this:
//!
//! ```none
//! Rbxd3
//! ||| |
//! |||  ---> The destination square. d3 represents the square with rank 2 (starting from an index of 0) and file 3. (Required)
//! || -----> This move is a capture. Required only when capturing.
//! | ------> The discriminant. If both rooks can move to d3, we move the one on the b file. This is required only when two identical pieces can move to the same square.
//! --------> The piece type. This is one of R (rook), N (knight), B (bishop), Q (queen), K (king). If the piece type isn't specified, we default to a pawn move.
//! ```
//!
//! If you want to dive deeper, there are many excellent resources on how algebraic notation works,
//! including the [Wikipedia page](https://en.wikipedia.org/wiki/Algebraic_notation_(chess)).
//!
//! ## Exceptions
//! There are lots of specific moves that don't follow the rules above.
//! - Castling is indicated by O-O (kingside) or O-O-O (queenside).
//! - Sometimes checks and checkmate are given special symbols. These do not actually add
//! additional information in determining the move made but are standard in chess notation anyways.
//! They're not used or expected here however.
//!
//! # Examples
//! You probably want to start by creating a [MoveCommand] from an algebraically notated string.
//! ```
//! # use rust_chess_engine::parse::MoveCommand;
//! # use std::str::FromStr;
//! let next_move = MoveCommand::from_str("Rxb5").unwrap();
//! println!("{:?}", next_move);
//! /* This will print the following:
//!  * NormalMove(MoveData { piece_type: Rook, discriminant: None, capture: false, target_square: Square { rank: 4, file: 1 } })
//! */
//! ```

mod move_command;
mod coordinates;
mod error;
mod parse_piece_type;
mod parse_line;
mod parse_square;

// Re-exports
pub use move_command::MoveCommand;
pub use move_command::MoveData;
pub use error::NotationParseError;
pub use coordinates::alphabetic_file_to_numeric;
pub use coordinates::algebraic_to_square;
