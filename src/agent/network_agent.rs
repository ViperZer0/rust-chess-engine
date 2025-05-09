//! This module defines the [LocalNetworkAgent] and [RemoteNetworkAgent], 
//! which allow two players to play over a network. Each player locally has a [LocalNetworkAgent]
//! and the other player is treated as a [RemoteNetworkAgent], waiting for moves to come in.

use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use crate::{board::Move, game::GameState};

use super::{Agent, LocalAgent};

/// Opens up a [TcpListener] and blocks and waits for a connection.
///
/// Once a client connects to the listener, this function returns a tuple containing
/// a [LocalNetworkAgent] and a [RemoteNetworkAgent] in that order.
///
/// Both of the network agents operate on the opened [TcpStream].
///
/// # Arguments
///
/// * `addr` - The address to bind and listen for connections on. Accepts any [ToSocketAddrs]
///
/// # Panics
///
/// This function panics if any network operation fails.
///
/// # Examples
///
/// ```no_run
/// # use rust_chess_engine::game::Game;
/// # use rust_chess_engine::agent::host;
/// // Blocks until a client connects
/// let (agent_1, agent_2) = host("127.0.0.1:8080");
/// // Start the game with the host and client as white and black respectively.
/// let mut game = Game::new(agent_1, agent_2);
/// game.run();
/// ```
pub fn host<A: ToSocketAddrs>(addr: A) -> (LocalNetworkAgent, RemoteNetworkAgent)
{
    let listener = TcpListener::bind(addr).unwrap();
    match listener.accept()
    {
        Ok((socket, _)) => (LocalNetworkAgent::new(socket.try_clone().expect("Cloning stream failed")), RemoteNetworkAgent::new(socket.try_clone().expect("Cloning stream failed!"))),
        Err(_) => panic!("Couldn't accept connection :c"),
    }
}

/// This function is the counterpart to [host]. This function connects to a waiting/listening
/// [TcpListener] and opens up a new [TcpStream]. When the connection is accepted,
/// this returns a [RemoteNetworkAgent] and a [LocalNetworkAgent] respectively.
///
/// Note that the order of the tuple is flipped in comparison to [host]. This is intentional.
///
/// If a game on one client is started with 
///
/// ```no_run
/// # use rust_chess_engine::game::Game;
/// # use rust_chess_engine::agent::host;
/// let (local_agent, remote_agent) = host("127.0.0.1:8080");
/// let mut game = Game::new(local_agent, remote_agent);
/// ```
///
/// The game on the other client should be started with
/// ```no_run
/// # use rust_chess_engine::game::Game;
/// # use rust_chess_engine::agent::connect;
/// let (remote_agent, local_agent) = connect("127.0.0.1:8080");
/// let mut game = Game::new(remote_agent, local_agent);
/// ```
///
/// Here in the second example `remote_agent` corresponds to the first example's
/// `local_agent`, and vice versa. White is the first player, local in example 1
/// and remote in example 2, and Black is the second player, remote in example 1 
/// and local in example 2.
///
/// # Arguments
///
/// * `addr` - The address to connect to. This can be any [ToSocketAddrs]
///
/// # Panics
///
/// Panics if something goes wrong with the connection or cloning the [TcpStream]
///
/// # Examples
///
/// ```no_run
///
/// # use rust_chess_engine::game::Game;
/// # use rust_chess_engine::agent::connect;
/// let (agent_1, agent_2) = connect("127.0.0.1:8080");
/// let mut game = Game::new(agent_1, agent_2);
/// game.run();
/// ```
pub fn connect<A: ToSocketAddrs>(addr: A) -> (RemoteNetworkAgent, LocalNetworkAgent)
{
    if let Ok(stream) = TcpStream::connect(addr)
    {
        (RemoteNetworkAgent::new(stream.try_clone().expect("Cloning stream failed")), LocalNetworkAgent::new(stream.try_clone().expect("Cloning stream failed")))
    }
    else
    {
        panic!("Unable to connect :c");
    }
}

/// A [LocalNetworkAgent] is an [Agent](super::Agent) that functions exactly
/// like a [LocalAgent](super::LocalAgent) but also sends the [Move](crate::board::Move)s made 
/// to the paired [RemoteNetworkAgent].
pub struct LocalNetworkAgent
{
    // We wrap an inner agent so that we can use the local agent code to do things
    inner_agent: LocalAgent,
    stream: TcpStream,
}

impl LocalNetworkAgent
{
    fn new(stream: TcpStream) -> Self
    {
        Self
        {
            inner_agent: LocalAgent,
            stream,
        }
    }
}

impl Agent for LocalNetworkAgent
{
    fn agent_move_request(&mut self, game_state: &GameState) -> Move {
        loop 
        {
            let r#move = self.inner_agent.agent_move_request(game_state);
            // This may be a mistake having the local agent validate its own move before sending it
            // instead of just improving the board error check code...
            //
            // But there's no reason we can't just validate the move here.
            let new_board_result = game_state.current_board().attempt_move(&r#move);
            match new_board_result
            {
                Err(error) => {
                    println!("Error making move! {}", error);
                    continue;
                },
                Ok(_) =>
                {
                    // Return the move we made and also send it to the RemoteNetworkAgent.
                    postcard::to_io(&r#move, &self.stream).expect("Unable to write move to stream!");
                    return r#move;
                }
            }
        }
    }
}

/// A [RemoteNetworkAgent] is an [Agent](super::Agent) that listens for [Move](crate::board::Move)s
/// made from a paired [LocalNetworkAgent], keeping two instances of a [Game](crate::game::Game) in
/// sync on two different machines by copying the moves made.
pub struct RemoteNetworkAgent
{
    stream: TcpStream,
}

impl RemoteNetworkAgent
{
    fn new(stream: TcpStream) -> Self
    {
        Self
        {
            stream
        }
    }
}

impl Agent for RemoteNetworkAgent
{
    fn agent_move_request(&mut self, game_state: &GameState) -> Move {
        println!("{}", game_state.current_board());
        println!("Waiting for player's move...");
        let mut buffer: Vec<u8> = Vec::new();
        let r#move: Move = postcard::from_io((&self.stream, &mut buffer)).expect("Couldn't read move from stream!").0;
        r#move
    }
}
