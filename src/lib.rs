#![warn(missing_docs)]
#![allow(dead_code)]

//! More documentation here???
pub mod parse;
pub mod board;
pub mod bitboard;
pub mod agent;
pub mod game;
mod hashmap_diff;

/// Defines the size of unsigned integers used to store and calculate square coordinates.
/// a 8x8 grid of squares can be described using only u8s, but in theory if we were to use larger
/// boards for some reason we might want a bigger int size, so we can use this to change said size.
pub type UInt = u8;
