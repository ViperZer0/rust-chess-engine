//! This sub module implements position evaluation and analysis for [Board]s.

use std::cmp::Ordering;
use derive_more::From;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use crate::board::{BoardResult, Move, PieceType, PlayerColor, Square};
use super::Board;

/// How highly to evaluate certain aspects of the position.
pub struct EvaluationWeights
{
    overall_material_weight: f64,
    queen_material_weight: f64,
    rook_material_weight: f64,
    bishop_material_weight: f64,
    knight_material_weight: f64,
    pawn_material_weight: f64,

    center_control_weight: f64,
    mobility_weight: f64,
}

impl Default for EvaluationWeights
{
    fn default() -> Self
    {
        Self
        {
            overall_material_weight: 1.0,
            queen_material_weight: 9.0,
            rook_material_weight: 5.0,
            bishop_material_weight: 3.0,
            knight_material_weight: 3.0,
            pawn_material_weight: 1.0,
            center_control_weight: 0.4,
            mobility_weight: 0.4,
        }
    }
}


/// The evaluated score of a given position.
#[derive(From, Copy, Clone, Debug)]
pub enum Evaluation
{
    /// This position is over, white has won.
    WhiteWin,
    /// The position is guaranteed to be a win for white in X turns with perfect play
    #[from(skip)]
    WhiteCheckmateIn(usize),
    /// The position is over with a stalemate or draw for both players. 
    Draw,
    /// The position is guaranteed to be a win for black in X turns with perfect play.
    #[from(skip)]
    BlackCheckmateIn(usize),
    /// The position is over, black has won.
    BlackWin,
    /// The position is undecided, but heuristics evalute the score to be positive (better for
    /// white) or negative (better for black)
    Score(f64)
}

impl From<isize> for Evaluation
{
    /// We can convert an `isize` into an `Evaluation`.
    ///
    /// We consider a positive integer to mean "White will checkmate Black in X moves".
    /// We consider a negative integer to mean "Black will checkmate White in X moves".
    ///
    /// This is why we implement [From] for [isize] but not [usize] (Even though we store the
    /// number as a `usize` internally)
    ///
    /// # Arguments
    ///
    /// * `value` - The number of moves until White checkmates Black if positive, or vice versa if
    /// negative.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_chess_engine::board::Evaluation;
    /// let evaluation = Evaluation::from(2_isize);
    /// assert_eq!(Evaluation::WhiteCheckmateIn(2_usize), evaluation);
    /// let evaluation: Evaluation = (-10_isize).into();
    /// assert_eq!(Evaluation::BlackCheckmateIn(10_usize), evaluation);
    /// let evaluation: Evaluation = 0_isize.into();
    /// assert_eq!(Evaluation::Draw, evaluation);
    /// ```
    fn from(value: isize) -> Self {
        match value
        {
            value if value > 0 => Evaluation::WhiteCheckmateIn(value as usize),
            value if value < 0 => Evaluation::BlackCheckmateIn(-value as usize),
            _ => Evaluation::Draw,
        }
    }
}

impl PartialEq for Evaluation
{
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).is_some_and(|ordering| ordering.is_eq())
    }
}

impl Eq for Evaluation {}

impl PartialOrd for Evaluation
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Evaluation
{
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other)
        {
            (Self::WhiteWin, Self::WhiteWin) => Ordering::Equal,
            // We want a LOWER checkmating score, that's a GREATER value for white,
            // so it's backwards.
            (Self::WhiteCheckmateIn(x), Self::WhiteCheckmateIn(y)) => x.cmp(y),
            (Self::Draw, Self::Draw) => Ordering::Equal,
            (Self::BlackCheckmateIn(x), Self::BlackCheckmateIn(y)) => y.cmp(x),
            (Self::BlackWin, Self::BlackWin) => Ordering::Equal,
            // Here we DO want a higher score for white.
            (Self::Score(x), Self::Score(y)) => x.partial_cmp(y).expect("Unable to compare floats"),
            // White win is greater than any other evaluation
            (Self::WhiteWin, _) => Ordering::Greater,
            // Ditto here
            (_, Self::WhiteWin) => Ordering::Less,
            // White checkmate is greater than remaining branches.
            (Self::WhiteCheckmateIn(_), _) => Ordering::Greater,
            (_, Self::WhiteCheckmateIn(_)) => Ordering::Less,
            // Draw can be better than or worse than the evaluation.
            // We treat Draw as a score of 0.
            (Self::Score(x), Self::Draw) => x.partial_cmp(&0_f64).expect("Unable to compare floats"),
            (Self::Draw, Self::Score(x)) => 0_f64.partial_cmp(x).expect("Unable to compare floats"),
            (Self::Draw, _) => Ordering::Greater,
            (_, Self::Draw) => Ordering::Less,
            (Self::Score(_), _) => Ordering::Greater,
            (_, Self::Score(_)) => Ordering::Less,
            (Self::BlackCheckmateIn(_), Self::BlackWin) => Ordering::Greater,
            (Self::BlackWin, Self::BlackCheckmateIn(_)) => Ordering::Less,
        }
    }
}

impl Board
{
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
    pub fn evaluate(&self, evaluation_weights: &EvaluationWeights, depth: usize) -> Evaluation
    {
        // Start at negative and positive "infinity"
        let mut alpha = Evaluation::BlackWin;
        let mut beta = Evaluation::WhiteWin;
        self.evaluate_recursive(evaluation_weights, &mut alpha, &mut beta, depth)
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
    fn evaluate_recursive(&self, evaluation_weights: &EvaluationWeights, alpha: &mut Evaluation, beta: &mut Evaluation, depth: usize) -> Evaluation
    {
        // If this board is over, we don't need to use heuristics or recurse deeper.
        match self.game_result()
        {
            BoardResult::Win(PlayerColor::White) => Evaluation::WhiteWin,
            BoardResult::Win(PlayerColor::Black) => Evaluation::BlackWin,
            BoardResult::Draw(_) => Evaluation::Draw,
            BoardResult::InProgress =>
            {
                // Once we reach a depth of 0, just approximate the position and return the score
                if depth == 0
                {
                    let score: f64 = self.evaluate_approximate(evaluation_weights);
                    return score.into();
                }

                let possible_moves = self.generate_moves_for_side(self.active_color());
                match self.active_color()
                {
                    // White is trying to MAXIMIZE score.
                    PlayerColor::White =>
                    {
                        let mut best_value = Evaluation::BlackWin;
                        for r#move in possible_moves
                        {
                            best_value = Ord::max(best_value, self.make_move(&r#move).evaluate_recursive(evaluation_weights, alpha, beta, depth - 1));
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
                            best_value = Ord::min(best_value, self.make_move(&r#move).evaluate_recursive(evaluation_weights, alpha, beta, depth - 1));
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
    }

    /// Evaluate a position without actually traversing future positions.
    ///
    /// Instead we use EvaluationWeights to approximate how "good" the position is.
    fn evaluate_approximate(&self, evaluation_weights: &EvaluationWeights) -> f64
    {
        let material_score = self.evaluate_material_score(evaluation_weights);
        let center_control_score = self.evaluate_center_control(evaluation_weights);
        let mobility_score = self.evaluate_material_score(evaluation_weights);
        return material_score + center_control_score + mobility_score;
    }

    // Compares the amount of material each side has and returns the total weighted difference.
    fn evaluate_material_score(&self, evaluation_weights: &EvaluationWeights) -> f64
    {
        // Evalute differences in the number of queens
        let white_piece_count = self.query().color(PlayerColor::White).piece_type(PieceType::Queen).result().squares().count() as isize;
        let black_piece_count = self.query().color(PlayerColor::Black).piece_type(PieceType::Queen).result().squares().count() as isize;
        let queen_score = (white_piece_count - black_piece_count) as f64 * evaluation_weights.queen_material_weight;

        // Evaluate differences in the number of rooks
        let white_piece_count = self.query().color(PlayerColor::White).piece_type(PieceType::Rook).result().squares().count() as isize;
        let black_piece_count = self.query().color(PlayerColor::Black).piece_type(PieceType::Rook).result().squares().count() as isize;
        let rook_score = (white_piece_count - black_piece_count) as f64 * evaluation_weights.rook_material_weight;

        // Evaluate differences in the number of bishops
        let white_piece_count = self.query().color(PlayerColor::White).piece_type(PieceType::Bishop).result().squares().count() as isize;
        let black_piece_count = self.query().color(PlayerColor::Black).piece_type(PieceType::Bishop).result().squares().count() as isize;
        let bishop_score = (white_piece_count - black_piece_count) as f64 * evaluation_weights.bishop_material_weight;

        // Evaluate differences in the number of knights
        let white_piece_count = self.query().color(PlayerColor::White).piece_type(PieceType::Knight).result().squares().count() as isize;
        let black_piece_count = self.query().color(PlayerColor::Black).piece_type(PieceType::Knight).result().squares().count() as isize;
        let knight_score = (white_piece_count - black_piece_count) as f64 * evaluation_weights.knight_material_weight;

        // Evaluate differences in the number of pawns
        let white_piece_count = self.query().color(PlayerColor::White).piece_type(PieceType::Pawn).result().squares().count() as isize;
        let black_piece_count = self.query().color(PlayerColor::Black).piece_type(PieceType::Pawn).result().squares().count() as isize;
        let pawn_score = (white_piece_count - black_piece_count) as f64 * evaluation_weights.pawn_material_weight;

        return (queen_score + rook_score + bishop_score + knight_score + pawn_score) * evaluation_weights.overall_material_weight;
    }

    fn evaluate_center_control(&self, evaluation_weights: &EvaluationWeights) -> f64
    {
        let square_1_count = self.all_squares_that_can_capture_square_theoretical(PlayerColor::White, Square::new(3, 3)).len() as isize -
                             self.all_squares_that_can_capture_square_theoretical(PlayerColor::Black, Square::new(3, 3)).len() as isize;
        let square_2_count = self.all_squares_that_can_capture_square_theoretical(PlayerColor::White, Square::new(3, 4)).len() as isize -
                             self.all_squares_that_can_capture_square_theoretical(PlayerColor::Black, Square::new(3, 4)).len() as isize;
        let square_3_count = self.all_squares_that_can_capture_square_theoretical(PlayerColor::White, Square::new(4, 4)).len() as isize -
                             self.all_squares_that_can_capture_square_theoretical(PlayerColor::Black, Square::new(4, 4)).len() as isize;
        let square_4_count = self.all_squares_that_can_capture_square_theoretical(PlayerColor::White, Square::new(4, 3)).len() as isize -
                             self.all_squares_that_can_capture_square_theoretical(PlayerColor::Black, Square::new(4, 3)).len() as isize;

        return (square_1_count + square_2_count + square_3_count + square_4_count) as f64 * evaluation_weights.center_control_weight;
    }

    fn evaluate_mobility(&self, evaluation_weights: &EvaluationWeights) -> f64
    {
        let white_moves = self.generate_moves_for_side(PlayerColor::White).len() as isize;
        let black_moves = self.generate_moves_for_side(PlayerColor::Black).len() as isize;
        return (white_moves - black_moves) as f64 * evaluation_weights.mobility_weight;
    }
}

#[cfg(test)]
mod tests
{
    use std::str::FromStr;

    use crate::board::BoardConfiguration;

    use super::*;

    #[test]
    fn test_equal_evaluations_draw()
    {
        assert!(Evaluation::WhiteWin == Evaluation::WhiteWin);
        assert!(Evaluation::WhiteCheckmateIn(1) == Evaluation::WhiteCheckmateIn(1));
        assert!(Evaluation::Draw == Evaluation::Draw);
        assert!(Evaluation::BlackCheckmateIn(1) == Evaluation::BlackCheckmateIn(1));
        assert!(Evaluation::BlackWin == Evaluation::BlackWin);
        assert!(Evaluation::Score(0.0) == Evaluation::Score(0.0));
    }

    #[test]
    fn test_evaluation_white_win_greater_than_all_others()
    {
        assert!(Evaluation::WhiteWin > Evaluation::WhiteCheckmateIn(1));
        assert!(Evaluation::WhiteWin > Evaluation::Draw);
        assert!(Evaluation::WhiteWin > Evaluation::Score(999999_f64));
        assert!(Evaluation::WhiteWin > Evaluation::BlackCheckmateIn(1));
        assert!(Evaluation::WhiteWin > Evaluation::BlackWin);
    }

    #[test]
    fn test_evaluation_white_checkmate()
    {
        assert!(Evaluation::WhiteCheckmateIn(1) < Evaluation::WhiteWin);
        assert!(Evaluation::WhiteCheckmateIn(1) > Evaluation::Score(999999.0));
        assert!(Evaluation::WhiteCheckmateIn(1) > Evaluation::Draw);
        assert!(Evaluation::WhiteCheckmateIn(1) > Evaluation::BlackCheckmateIn(1));
        assert!(Evaluation::WhiteCheckmateIn(1) > Evaluation::BlackWin);
    }

    #[test]
    fn test_relative_scores()
    {
        // White wants to maximize their evaluation. A shorter checkmate for black is better
        // for white.
        assert!(Evaluation::BlackCheckmateIn(1) > Evaluation::BlackCheckmateIn(2));
        // On the flip side, black wants to minimize the evaluation, which means a shorter
        // checkmate for white is "more negative" than a longer checkmate
        assert!(Evaluation::WhiteCheckmateIn(1) < Evaluation::WhiteCheckmateIn(2));
        // White wants a HIGHER evaluation
        assert!(Evaluation::Score(100.0) > Evaluation::Score(-3.0));
        // Higher score is better than a draw for white
        assert!(Evaluation::Score(1.0) > Evaluation::Draw);
        assert!(Evaluation::Score(-1.0) < Evaluation::Draw);
    }

    #[test]
    fn evaluate_center_control()
    {
        let board = Board::new_board_with_configuration(&BoardConfiguration::from_str("r1bqkbnr/pppppppp/2n5/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1").unwrap());
        let evaluation_weights = EvaluationWeights {
            center_control_weight: 1.0,
            ..Default::default()
        };

        // It should be 1 white squares and 2 black squares
        assert_eq!(-1.0, board.evaluate_center_control(&evaluation_weights));
    }
}
