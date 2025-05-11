//! This module implements [MinmaxAgent], a CPU/AI/chess engine [Agent] that
//! attempts to predict what the best move to make is.

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{board::{Evaluation, EvaluationWeights, Move, PlayerColor}, game::GameState};

use super::Agent;

/// An [Agent] that uses the minimax algorithm
/// to determine the best move to play.
pub struct MinmaxAgent
{
    evaluation_weights: EvaluationWeights,
    evaluation_depth: usize,
}

impl Agent for MinmaxAgent {
    fn agent_move_request(&mut self, game_state: &GameState) -> Move {
        let agent_color = game_state.current_board().active_color();
        let moves = game_state.current_board().generate_moves_for_side(agent_color);
        let best_move = 
        moves.par_iter().map(|r#move| (r#move, self.evaluate_next_move(game_state, r#move)))
        .reduce_with(|a, b|
            match is_new_score_better_than_old_score(agent_color, a.1, b.1)
            {
                true => b,
                false => a,
            }
        ).expect("No moves generated!");
        println!("Best move score: {:?}", best_move.1);
        *(best_move.0)
    }
}

impl MinmaxAgent
{
    /// Creates a new [MinmaxAgent]
    ///
    /// # Arguments
    ///
    /// * `num_candidate_moves` - How many moves to investigate further after a first analysis.
    /// * `evaluation_depth` - How many moves in the future to evaluate
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::agent::MinmaxAgent;
    /// let minmax_agent = MinmaxAgent::new(2);
    /// ```
    pub fn new(evaluation_depth: usize) -> Self
    {
        MinmaxAgent
        {
            evaluation_weights: EvaluationWeights::default(),
            evaluation_depth,
        }
    }

    fn evaluate_next_move(&self, current_game_state: &GameState, next_move: &Move) -> Evaluation
    {
        let next_move = current_game_state.update(next_move)
            .expect("Somehow we gave the board an illegal move in the Minmax Agent");

        // If we want to we can add to our evaluation with stuff like
        // "Are we moving a piece twice in a row" or whatever. Hopefully.
        return next_move.current_board().evaluate(&self.evaluation_weights, self.evaluation_depth);
    }
}

fn is_new_score_better_than_old_score(player_color: PlayerColor, old_score: Evaluation, new_score: Evaluation) -> bool
{
    match player_color
    {
        PlayerColor::White => old_score < new_score,
        PlayerColor::Black => old_score > new_score,
    }
}

