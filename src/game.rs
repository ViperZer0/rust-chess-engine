//! This module implements the main game loop

use getset::Getters;

use crate::{agent::Agent, board::{Board, Move, MoveError}};

/// A game of chess!!!
pub struct Game<A1, A2>
where A1: Agent, A2: Agent
{
    game_state: GameState,
    agent_white: A1,
    agent_black: A2,
}

/// Holds the current board.
/// Will also probably hold things like the board history 
/// and the last made move so specific agents can use them when needed.
#[derive(Getters)]
#[getset(get="pub")]
pub struct GameState
{
    /// Gets the current [Board].
    current_board: Board,
}

impl GameState
{
    /// Makes a [Move] on a [Board] 
    /// and updates the state of the game accordingly.
    ///
    /// # Arguments
    ///
    /// * `r#move` - The move to make
    ///
    /// # Errors
    ///
    /// Returns a [MoveError] if [Board::attempt_move] failed.
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn update(&self, r#move: &Move) -> Result<Self, MoveError>
    {
        Ok(Self
        {
            current_board: self.current_board.attempt_move(r#move)?,
        })
    }
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
            game_state: GameState
            {
                current_board: Board::new_default_starting_board(),
            },
            agent_white,
            agent_black,
        }
    }

    /// Runs the full game until the game is over
    pub fn run(&mut self) 
    {
        while self.game_state.current_board().game_result().is_in_progress()
        {
            self.next_round()
        }

        // Once the game is over we do something idk
        println!("Game is over!");
        println!("Result: {:?}", self.game_state.current_board().game_result());
    }

    /// Progresses the game by one "round", i.e
    /// one move by white and one move by black.
    ///
    /// # Examples
    ///
    pub fn next_round(&mut self)
    {
        if self.game_state.current_board().game_result().is_in_progress()
        {
            self.game_state = Self::agent_turn(&self.game_state, &mut self.agent_white);
        }
        if self.game_state.current_board().game_result().is_in_progress()
        {
            self.game_state = Self::agent_turn(&self.game_state, &mut self.agent_black);
        }
    }

    /// Progresses the game by one "turn",
    /// i.e one move by either white or black.
    ///
    /// # Arguments
    ///
    /// * `agent` - The agent taking their turn.
    ///
    fn agent_turn<A: Agent>(game_state: &GameState, agent: &mut A) -> GameState
    {
        loop
        {
            let move_request = agent.agent_move_request(game_state);
            let new_game_state = game_state.update(&move_request);
            match new_game_state
            {
                Err(error) => {
                    println!("Error making move! {}", error);
                    continue;
                },
                Ok(new_game_state) =>
                {
                    return new_game_state;
                }
            }
        }
    }
}
