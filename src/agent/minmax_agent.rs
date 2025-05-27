//! This module implements [MinmaxAgent], a CPU/AI/chess engine [Agent] that
//! attempts to predict what the best move to make is.

use std::{collections::HashMap, sync::RwLock};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{board::{Board, Evaluation, EvaluationWeights, Move, PlayerColor}, game::GameState};

use super::Agent;

/// An [Agent] that uses the minimax algorithm
/// to determine the best move to play.
pub struct MinmaxAgent
{
    evaluation_weights: EvaluationWeights,
    evaluation_depth: usize,
    board_memory: RwLock<HashMap<Board, Evaluation>>
}

/// Contains information about how this board state was evaluated.
struct BoardEvaluationContext
{
    // How many boards past this board have been evaluated.
    depth: usize,
    // The evaluation outcome
    evaluation: Evaluation
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
            board_memory: RwLock::new(HashMap::new()),
        }
    }

    fn evaluate_next_move(&self, current_game_state: &GameState, next_move: &Move) -> Evaluation
    {
        let next_move = current_game_state.update(next_move)
            .expect("Somehow we gave the board an illegal move in the Minmax Agent");

        // If we want to we can add to our evaluation with stuff like
        // "Are we moving a piece twice in a row" or whatever. Hopefully.
        return Self::evaluate(next_move.current_board(), &self.evaluation_weights, self.evaluation_depth);
    }

    /// Evaluates a position. 
    ///
    /// # Arguments
    ///
    /// * `evaluation_weights` - How much to weight different factors of a position. This analysis
    /// only happens after `depth` is reached, otherwise the score is based on the maximum or
    /// minimum of all possible moves
    /// - `num_candidate_moves` - How many moves to evaluate in detail after an initial cheap analysis.
    /// * `depth` - How many moves into the future to calculate.
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn evaluate(board: &Board, evaluation_weights: &EvaluationWeights, depth: usize) -> Evaluation
    {
        // Start at negative and positive "infinity"
        let mut alpha = Evaluation::BlackWin;
        let mut beta = Evaluation::WhiteWin;
        Self::evaluate_recursive(board, evaluation_weights, &mut alpha, &mut beta, depth)
    }

    /// Recursively evaluate all possible moves up to `depth` moves in the future.
    /// 
    /// We use a minimax algorithm with alpha-beta pruning.
    ///
    /// # Arguments
    ///
    /// * `evaluation_weights` - The weights to use at the end of the evaluation when we use
    /// heuristics to evaluate how good a position is.
    /// * `alpha` - The minimum score that the maximizing player is assured of.
    /// * `beta` - The maximum score that the minimizing player is assured of.
    /// * `depth` - How many moves in the future to continue evaluating
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    fn evaluate_recursive(board: &Board, evaluation_weights: &EvaluationWeights, alpha: &mut Evaluation, beta: &mut Evaluation, depth: usize) -> Evaluation
    {
        // Once we reach a depth of 0, just approximate the position and return the score
        if depth == 0
        {
            return board.evaluate_approximate(evaluation_weights);
        }

        let possible_moves = board.generate_moves_for_side(board.active_color());
        match board.active_color()
        {
            // White is trying to MAXIMIZE score.
            PlayerColor::White =>
            {
                let mut best_value = Evaluation::BlackWin;
                for r#move in possible_moves
                {
                    best_value = Ord::max(best_value, Self::evaluate_recursive(&board.attempt_move(&r#move).expect("Expected move to be valid."), evaluation_weights, alpha, beta, depth - 1));
                    if best_value >= *beta
                    {
                        // No need to evaluate further, we already know this is more or
                        // less "too good to be true" because black is assured of a lower
                        // score.
                        //
                        // This is known as a beta cutoff.
                        break;
                    }
                    // We update our minimum score to the greater of these two values.
                    *alpha = Ord::max(*alpha, best_value);
                }
                return match best_value
                {
                    // We update our evaluation based on going "back" a step. So if White
                    // wins in the next board, on this board we return a score of
                    // WhiteCheckmateIn(1), so on and so forth.
                    Evaluation::WhiteWin => Evaluation::WhiteCheckmateIn(1),
                    Evaluation::WhiteCheckmateIn(x) => Evaluation::WhiteCheckmateIn(x+1),
                    Evaluation::BlackWin => Evaluation::BlackCheckmateIn(1),
                    Evaluation::BlackCheckmateIn(x) => Evaluation::BlackCheckmateIn(x+1),
                    Evaluation::Draw => Evaluation::Score(0.0),
                    Evaluation::Score(x) => Evaluation::Score(x),
                }
            }
            // Black is trying to MINIMIZE score.
            PlayerColor::Black =>
            {
                let mut best_value = Evaluation::WhiteWin;
                for r#move in possible_moves
                {
                    best_value = Ord::min(best_value, Self::evaluate_recursive(&board.attempt_move(&r#move).expect("Expected move to be valid"), evaluation_weights, alpha, beta, depth - 1));
                    if best_value <= *alpha
                    {
                        // No need to evaluate further, see above case for beta cutoff.
                        //
                        // This is an alpha cutoff.
                        break;
                    }
                    // The maximum score that black is assured of.
                    *beta = Ord::min(*beta, best_value);
                }
                return match best_value
                {
                    // We update our evaluation based on going "back" a step. So if White
                    // wins in the next board, on this board we return a score of
                    // WhiteCheckmateIn(1), so on and so forth.
                    Evaluation::WhiteWin => Evaluation::WhiteCheckmateIn(1),
                    Evaluation::WhiteCheckmateIn(x) => Evaluation::WhiteCheckmateIn(x+1),
                    Evaluation::BlackWin => Evaluation::BlackCheckmateIn(1),
                    Evaluation::BlackCheckmateIn(x) => Evaluation::BlackCheckmateIn(x+1),
                    Evaluation::Draw => Evaluation::Score(0.0),
                    Evaluation::Score(x) => Evaluation::Score(x),
                }
            }
        }
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

