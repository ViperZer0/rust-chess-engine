//! This example shows how to make a new [Agent] and use it in a [Game].

mod random_agent
{
    //! This module implements [RandomAgent], an [Agent]
    //! that makes random moves. Pretty useless except for debugging.

    use rust_chess_engine::game::GameState;

    use rust_chess_engine::board::Move;

    use rust_chess_engine::agent::Agent;

    /// A [RandomAgent] is an [Agent] that selects moves at random.
    /// Essentially the worst possible algorithm for a chess AI.

    pub struct RandomAgent;

    impl Agent for RandomAgent
    {
        fn agent_move_request(&mut self, game_state: &GameState) -> Move {
            let moving_color = game_state.current_board().active_color();
            let moves = game_state.current_board().generate_moves_for_side(moving_color);
            let move_index: usize = rand::random_range(0..moves.len());
            return moves[move_index]
        }
    }
}

use random_agent::RandomAgent;
use rust_chess_engine::{agent::LocalAgent, game::Game};

fn main()
{
    let agent_white = LocalAgent;
    let agent_black = RandomAgent;

    let mut game = Game::new(agent_white, agent_black);
    game.run();
}
