//! This module defines the [Agent] trait as well as a collection
//! of agents.

use crate::board::{Board, Move};

mod local_agent;
mod random_agent;

pub use local_agent::LocalAgent;
pub use random_agent::RandomAgent;

/// This trait defines an agent, which takes a `&mut self` and
/// an &[Board] and returns the [Move] that it has selected so the game can progress.
pub trait Agent
{
    /// This method requests for an agent to make a move. Once it has selected its move,
    /// the game will handle implementing the move and such.
    fn agent_move_request(&mut self, board: &Board) -> Move;
}
