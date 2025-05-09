//! This module implements [MinmaxAgent], a CPU/AI/chess engine [Agent] that
//! attempts to predict what the best move to make is.

use crate::{board::{Board, Evaluation, EvaluationWeights, Move, PlayerColor}, game::GameState};

use super::Agent;

pub struct MinmaxAgent
{
    evaluation_weights: EvaluationWeights,
    evaluation_depth: usize,
}

impl Agent for MinmaxAgent {
    fn agent_move_request(&mut self, game_state: &GameState) -> Move {
        let agent_color = game_state.current_board().active_color();
        let moves = game_state.current_board().generate_moves_for_side(agent_color);
        let possible_move_len = moves.len();
        let mut current_iter = 0;
        assert_ne!(possible_move_len, 0);
        let mut moves_iter = moves.iter();
        // Initialize first/default move
        println!("Thinking...");
        let mut best_move = moves_iter.next().unwrap();
        let mut best_score = self.evaluate_next_move(game_state.current_board(), &best_move);
        current_iter += 1;
        println!("Move score: {:?}", best_score);
        println!("Thinking... {:.1}% done", (current_iter as f64 / possible_move_len as f64) * 100.0);
        // Now we try to improve on our score.
        for r#move in moves_iter
        {
            let move_score = self.evaluate_next_move(game_state.current_board(), &r#move);
            println!("Move score: {:?}", best_score);
            if is_new_score_better_than_old_score(agent_color, best_score, move_score)
            {
                // Replace old score with new score
                best_score = move_score;
                // Replace old best move with new best move
                best_move = r#move;
            }
            current_iter += 1;
            println!("Thinking... {:.1}% done", (current_iter as f64 / possible_move_len as f64) * 100.0);
        }
        *best_move
    }
}

impl MinmaxAgent
{
    pub fn new(evaluation_depth: usize) -> Self
    {
        MinmaxAgent
        {
            evaluation_weights: EvaluationWeights::default(),
            evaluation_depth,
        }
    }

    fn evaluate_next_move(&self, current_board: &Board, next_move: &Move) -> Evaluation
    {
        current_board.attempt_move(next_move)
            .expect("Somehow we gave the board an illegla move in the Minmax Agent")
            .evaluate(&self.evaluation_weights, self.evaluation_depth)
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

