#![warn(missing_docs)]
//! Documentation goes here


use rust_chess_engine::{agent::LocalAgent, game::Game};

fn main() {
    let agent_white = LocalAgent;
    let agent_black = LocalAgent;

    let mut game = Game::new(agent_white, agent_black);
    game.run();
}
