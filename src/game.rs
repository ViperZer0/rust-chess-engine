//! This module implements the main game loop

use crate::{agent::Agent, board::{Board, Move}};

/// A game of chess!!!
pub struct Game<A1, A2>
where A1: Agent, A2: Agent
{
    current_board: Board,
    agent_white: A1,
    agent_black: A2,
}

impl<A1, A2> Game<A1, A2>
where A1: Agent, A2: Agent
{
    /// Creates a new [Game] with the given [Agent]s.
    ///
    /// # Arguments
    ///
    /// * `agent_white` - The white player [Agent]
    /// * `agent_black` - The black player [Agent]
    ///
    /// # Examples
    ///
    pub fn new(agent_white: A1, agent_black: A2) -> Self
    {
        Self
        {
            current_board: Board::new_default_starting_board(),
            agent_white,
            agent_black,
        }
    }

    /// Runs the full game until the game is over
    pub fn run(&mut self) 
    {
        while self.current_board.game_result().is_in_progress()
        {
            self.next_round()
        }

        // Once the game is over we do something idk
        println!("Game is over!");
        println!("Result: {:?}", self.current_board.game_result());
    }

    /// Progresses the game by one "round", i.e
    /// one move by white and one move by black.
    ///
    /// # Examples
    ///
    fn next_round(&mut self)
    {
        if self.current_board.game_result().is_in_progress()
        {
            self.current_board = Self::agent_turn(&self.current_board, &mut self.agent_white);
        }
        if self.current_board.game_result().is_in_progress()
        {
            self.current_board = Self::agent_turn(&self.current_board, &mut self.agent_black);
        }
    }

    /// Progresses the game by one "turn",
    /// i.e one move by either white or black.
    ///
    /// # Arguments
    ///
    /// * `agent` - The agent taking their turn.
    ///
    fn agent_turn<A: Agent>(board: &Board, agent: &mut A) -> Board
    {
        loop
        {
            let move_request = agent.agent_move_request(board);
            let new_board_result = board.attempt_move(&move_request);
            match new_board_result
            {
                Err(error) => {
                    println!("Error making move! {}", error);
                    continue;
                },
                Ok(new_board) =>
                {
                    return new_board;
                }
            }
        }
    }
}
