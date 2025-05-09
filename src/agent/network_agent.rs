//! This module defines the [LocalNetworkAgent] and [RemoteNetworkAgent], 
//! which allow two players to play over a network. Each player locally has a [LocalNetworkAgent]
//! and the other player is treated as a [RemoteNetworkAgent], waiting for moves to come in.

use std::{io::{Read, Write}, net::{TcpListener, TcpStream, ToSocketAddrs}};

use crate::board::{Board, Move};

use super::{Agent, LocalAgent};

pub fn host<A: ToSocketAddrs>(addr: A) -> (LocalNetworkAgent, RemoteNetworkAgent)
{
    let listener = TcpListener::bind(addr).unwrap();
    match listener.accept()
    {
        Ok((socket, _)) => (LocalNetworkAgent::new(socket.try_clone().expect("Cloning stream failed")), RemoteNetworkAgent::new(socket.try_clone().expect("Cloning stream failed!"))),
        Err(_) => panic!("Couldn't accept connection :c"),
    }
}

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
    fn agent_move_request(&mut self, board: &Board) -> Move {
        loop 
        {
            let r#move = self.inner_agent.agent_move_request(board);
            // This may be a mistake having the local agent validate its own move before sending it
            // instead of just improving the board error check code...
            //
            // But there's no reason we can't just validate the move here.
            let new_board_result = board.attempt_move(&r#move);
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
    fn agent_move_request(&mut self, board: &Board) -> Move {
        println!("{}", board);
        println!("Waiting for player's move...");
        let mut buffer: Vec<u8> = Vec::new();
        let r#move: Move = postcard::from_io((&self.stream, &mut buffer)).expect("Couldn't read move from stream!").0;
        r#move
    }
}
