//! Board stuff!!!

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
pub use board_config::{BoardConfiguration, CastlingAvailability};
pub use board_result::{BoardResult, DrawReason};

