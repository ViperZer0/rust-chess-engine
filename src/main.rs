#![warn(missing_docs)]
//! Documentation goes here

use std::str::FromStr;

use rust_chess_engine::{self, parse::MoveCommand};

fn main() {
    let next_move = MoveCommand::from_str("Rxb5").unwrap();
    println!("{:?}", next_move);
}
