//! This module defines the [LocalNetworkAgent] and [RemoteNetworkAgent], 
//! which allow two players to play over a network. Each player locally has a [LocalNetworkAgent]
//! and the other player is treated as a [RemoteNetworkAgent], waiting for moves to come in.

use std::{io::Write, net::TcpStream};

use crate::board::{Board, Move};

use super::{Agent, LocalAgent};

/// Creates two paired [Agent]s. 
/// 
/// `A1` and `A2` will be either a [LocalNetworkAgent] and a [RemoteNetworkAgent]
/// or a [RemoteNetworkAgent] and a [LocalNetworkAgent] respectively, depending on which client is
/// playing as White and which one is playing as Black.
pub fn create_connected_agents<A1, A2>() -> (A1, A2)
where A1: Agent, A2: Agent
{
    todo!()
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
                    self.stream.write(r#move);
                    return r#move;
                }
            }
        }
    }
}

/// A [RemoteNetworkAgent] is an [Agent](super::Agent) that listens for [Move](crate::board::Move)s
/// made from a paired [LocalNetworkAgent], keeping two instances of a [Game](crate::game::Game) in
/// sync on two different machines by copying the moves made.
pub struct RemoteNetworkAgent;
