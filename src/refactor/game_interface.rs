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
        let best_move = T3Move {
            row: 0,
            col: 0,
            side: Cell::X,
        };
        console::log_2(&"Returning best move".into(), &best_move.into());
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
}
