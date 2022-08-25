use crate::{BoardMove, Cell, FiarGameState, T3GameState, TreeEvaluator, X_WIN_VALUE};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub enum ExpandResult {
    Done,
    NotDone,
}

macro_rules! gen_game_if_impl {
    ( $game_if:ty, $max_depth:expr ) => {
        #[wasm_bindgen]
        impl $game_if {
            pub fn new() -> Self {
                console::log_1(&"Initialized a new GameInterface".into());
                Self {
                    tree_eval: TreeEvaluator::new_with_default(),
                    last_move_idx: 0,
                    expand_new_idx: vec![0],
                    cur_expanded_depth: 0,
                    max_expanded_depth: $max_depth,
                }
            }

            pub fn expand_one_level(&mut self) -> ExpandResult {
                match self.cur_expanded_depth {
                    x if x < self.max_expanded_depth => {
                        self.expand_new_idx = self
                            .tree_eval
                            .expand_and_get_children_idx(&self.expand_new_idx);
                        self.cur_expanded_depth += 1;
                        console::log_1(
                            &format!("Expanded level {}", self.cur_expanded_depth).into(),
                        );
                        match self.cur_expanded_depth < self.max_expanded_depth {
                            true => return ExpandResult::NotDone,
                            false => {
                                console::log_1(&"Expansion done".into());
                                return ExpandResult::Done;
                            }
                        }
                    }
                    _ => {
                        console::log_1(&"Expansion done".into());
                        return ExpandResult::Done;
                    }
                }
            }

            pub fn track_move(&mut self, game_move: BoardMove) -> bool {
                match self.identify_move(&game_move) {
                    Some(idx) => {
                        console::log_2(&"Tracked move".into(), &game_move.into());
                        // Update tracking values in game interface
                        self.last_move_idx = idx;
                        self.cur_expanded_depth -= 1;
                        return true;
                    }
                    None => {
                        console::log_2(&"Could not track move".into(), &game_move.into());
                        return false;
                    }
                }
            }

            pub fn get_best_move(&mut self) -> BoardMove {
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
                self.tree_eval = TreeEvaluator::new_with_default();
                self.last_move_idx = 0;
                self.expand_new_idx = vec![0];
                self.cur_expanded_depth = 0;
                self.max_expanded_depth = $max_depth;
            }

            fn identify_move(&self, game_move: &BoardMove) -> Option<usize> {
                let direct_children = self
                    .tree_eval
                    .children()
                    .get(self.last_move_idx)
                    .expect("Direct children");

                let game_states = direct_children.iter().map(|&child_idx| {
                    self.tree_eval
                        .game_states()
                        .get(child_idx)
                        .expect("Child game state")
                });

                for (&child_idx, game_state) in direct_children.iter().zip(game_states) {
                    if *game_move == game_state.last_move() {
                        return Some(child_idx);
                    }
                }

                None
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

                let (best_idx, best_avg_value): (Option<usize>, i32) = match last_state.side() {
                    Cell::O => {
                        direct_children_idx.iter().zip(avg_values).fold(
                            (None, -X_WIN_VALUE),
                            |(best_idx, best_avg_value), (&child_idx, &child_avg_value)| {
                                // If the last turn was O, the next is X and we want
                                // maximum values
                                if child_avg_value > best_avg_value {
                                    return (Some(child_idx), child_avg_value);
                                }
                                (best_idx, best_avg_value)
                            },
                        )
                    }
                    Cell::X => {
                        direct_children_idx.iter().zip(avg_values).fold(
                            (None, X_WIN_VALUE),
                            |(best_idx, best_avg_value), (&child_idx, &child_avg_value)| {
                                // If the last turn was O, the next is X and we want
                                // maximum values
                                if child_avg_value < best_avg_value {
                                    return (Some(child_idx), child_avg_value);
                                }
                                (best_idx, best_avg_value)
                            },
                        )
                    }
                    Cell::Empty => (None, 0),
                };

                (
                    best_idx.expect("Should have found a best index"),
                    best_avg_value,
                )
            }
        }
    };
}

#[wasm_bindgen]
pub struct T3GameInterface {
    tree_eval: TreeEvaluator<T3GameState>,
    last_move_idx: usize,
    expand_new_idx: Vec<usize>,
    cur_expanded_depth: usize,
    max_expanded_depth: usize,
}

gen_game_if_impl!(T3GameInterface, 9);

#[wasm_bindgen]
pub struct FiarGameInterface {
    tree_eval: TreeEvaluator<FiarGameState>,
    last_move_idx: usize,
    expand_new_idx: Vec<usize>,
    cur_expanded_depth: usize,
    max_expanded_depth: usize,
}

gen_game_if_impl!(FiarGameInterface, 6);
