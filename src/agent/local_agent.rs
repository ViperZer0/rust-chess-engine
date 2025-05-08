//! This module defines a [LocalAgent], a type of [Agent] that represents
//! a local player typing moves into their keyboard.

use std::io;
use std::str::FromStr;

use crate::{board::{Board, Move}, parse::MoveCommand};

use super::Agent;

pub struct LocalAgent;

impl Agent for LocalAgent
{
    fn agent_move_request(&mut self, board: &Board) -> Move {
        loop 
        {
            println!("{}", board);
            println!("Please enter your next move: ");
            let mut input = String::new();
            let result = io::stdin().read_line(&mut input);
            if result.is_err()
            {
                println!("Error: {}", result.unwrap_err());
                continue;
            }
            let move_command = MoveCommand::from_str(&input);
            if let Err(error) = move_command
            {
                println!("Badly formatted move! {}", error);
                continue;
            }
            let move_command = move_command.unwrap();
            let r#move = board.get_move(&move_command);
            if let Err(error) = r#move
            {
                println!("Impossible move: {}", error);
                continue;
            }
            return r#move.unwrap()
        }
    }
}
