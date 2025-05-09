use rust_chess_engine::{agent::{LocalAgent, MinmaxAgent}, game::Game};

fn main()
{
    let agent_white = LocalAgent;
    let agent_black = MinmaxAgent::new(2);

    let mut game = Game::new(agent_white, agent_black);
    game.run();
}
