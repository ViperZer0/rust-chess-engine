use rust_chess_engine::{agent::{connect, host, Agent}, game::Game};

fn main()
{
    println!("1) Host\n2) Join");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let (agent_1, agent_2): (Box<dyn Agent>, Box<dyn Agent>) = match input.trim()
    {
        "1" => {
            println!("Waiting for connection on 127.0.0.1:8080");
            let (agent_1, agent_2) = host("127.0.0.1:8080");
            (Box::new(agent_1), Box::new(agent_2))
        },
        "2" => {
            println!("Trying to connect to 127.0.0.1:8080");
            let (agent_1, agent_2) = connect("127.0.0.1:8080");
            (Box::new(agent_1), Box::new(agent_2))
        }
        _ => {
            panic!("Didn't recognize that option!");
        }
    };

    let mut game = Game::new(agent_1, agent_2);
    game.run();
}
