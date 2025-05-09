//! This module defines the [Agent] trait as well as a collection
//! of agents.

use crate::{board::Move, game::GameState};

mod local_agent;
mod network_agent;
mod minmax_agent;

pub use local_agent::LocalAgent;
pub use network_agent::{host, connect, LocalNetworkAgent, RemoteNetworkAgent};
pub use minmax_agent::MinmaxAgent;

/// This trait defines an agent, which takes a `&mut self` and
/// an &[Board] and returns the [Move] that it has selected so the game can progress.
pub trait Agent
{
    /// This method requests for an agent to make a move. Once it has selected its move,
    /// the game will handle implementing the move and such.
    fn agent_move_request(&mut self, game_state: &GameState) -> Move;
}

// We implement T for Box<dyn T> to basically convert dynamic dispatch
// to static dispatch.
impl<A: Agent + ?Sized> Agent for Box<A>
{
    fn agent_move_request(&mut self, game_state: &GameState) -> Move {
        (**self).agent_move_request(game_state)
    }
}
