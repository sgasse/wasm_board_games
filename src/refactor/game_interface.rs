use super::board::Cell;
use super::t3_game::{T3GameState, T3Move};
use super::tree_evaluator::TreeEvaluator;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub enum ExpandResult {
    Done,
    NotDone,
}

#[wasm_bindgen]
pub struct T3GameInterface {
    tree_eval: TreeEvaluator<T3GameState>,
    last_move_idx: usize,
    expand_new_idx: Vec<usize>,
    cur_expanded_depth: usize,
    max_expanded_depth: usize,
}

#[wasm_bindgen]
impl T3GameInterface {
    pub fn new() -> Self {
        console::log_1(&"Initialized a new T3GameInterface".into());
        Self {
            tree_eval: TreeEvaluator::new(T3GameState::default()),
            last_move_idx: 0,
            expand_new_idx: vec![0],
            cur_expanded_depth: 0,
            max_expanded_depth: 9,
        }
    }

    pub fn expand_one_level(&mut self) -> ExpandResult {
        match self.cur_expanded_depth {
            x if x < self.max_expanded_depth => {
                self.expand_new_idx = self
                    .tree_eval
                    .expand_and_get_children_idx(&self.expand_new_idx);
                self.cur_expanded_depth += 1;
                console::log_1(&format!("Expanded level {}", self.cur_expanded_depth).into());
                return ExpandResult::NotDone;
            }
            _ => {
                console::log_1(&"Expansion done".into());
                return ExpandResult::Done;
            }
        }
    }

    pub fn track_move(&mut self, game_move: T3Move) -> bool {
        console::log_2(&"Tracking move".into(), &game_move.into());
        true
    }

    pub fn get_best_move(&mut self) -> T3Move {
        // Evaluate value of all direct child states
        self.tree_eval.evaluate_states(self.last_move_idx);

        let (best_idx, best_avg_value) = self.identify_best_move();
        let best_move = self
            .tree_eval
            .game_states()
            .get(best_idx)
            .expect("Best state")
            .last_move();

        console::log_1(
            &format!(
                "Identified best move {:?} with avg_value {}",
                &best_move, best_avg_value
            )
            .into(),
        );

        // Update tracking values in game interface
        self.last_move_idx = best_idx;
        self.cur_expanded_depth -= 1;

        best_move
    }

    pub fn reset(&mut self) {
        console::log_1(&"Resetting game interface".into());
        self.tree_eval = TreeEvaluator::new(T3GameState::default());
        self.last_move_idx = 0;
        self.expand_new_idx = vec![0];
        self.cur_expanded_depth = 0;
        self.max_expanded_depth = 9;
    }

    fn identify_best_move(&self) -> (usize, i32) {
        // Select child state with highest value for `side`
        let last_state = self
            .tree_eval
            .game_states()
            .get(self.last_move_idx)
            .expect("Last state");

        let direct_children_idx = self
            .tree_eval
            .children()
            .get(self.last_move_idx)
            .expect("Direct children");

        let avg_values = direct_children_idx
            .iter()
            .filter_map(|&child_idx| self.tree_eval.avg_values().get(child_idx));

        let (best_idx, best_avg_value): (usize, i32) =
            direct_children_idx.iter().zip(avg_values).fold(
                (0, 0),
                |(best_idx, best_avg_value), (&child_idx, &child_avg_value)| {
                    match last_state.side() {
                        Cell::O => {
                            // If the last turn was O, the next is X and we want
                            // maximum values
                            if child_avg_value > best_avg_value {
                                return (child_idx, child_avg_value);
                            }
                        }
                        Cell::X => {
                            // If the last turn was X, the next is O and we want
                            // minimum values
                            if child_avg_value < best_avg_value {
                                return (child_idx, child_avg_value);
                            }
                        }
                        _ => (),
                    }

                    // Default case
                    return (best_idx, best_avg_value);
                },
            );

        (best_idx, best_avg_value)
    }
}
