//! This module implements [RandomAgent], an [Agent]
//! that makes random moves. Pretty useless except for debugging.

use crate::board::{Board, Move};

use super::Agent;

pub struct RandomAgent;

impl Agent for RandomAgent
{
    fn agent_move_request(&mut self, board: &Board) -> Move {
        let moving_color = board.active_color();
        let moves = board.generate_moves_for_side(moving_color);
        let move_index: usize = rand::random_range(0..moves.len());
        return moves[move_index]
    }
}
