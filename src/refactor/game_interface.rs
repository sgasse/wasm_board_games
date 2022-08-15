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
}

#[wasm_bindgen]
impl T3GameInterface {
    pub fn new() -> Self {
        console::log_1(&"Initialized a new T3GameInterface".into());
        Self {
            tree_eval: TreeEvaluator::new(T3GameState::default()),
        }
    }

    pub fn expand_one_level(&mut self) -> ExpandResult {
        return ExpandResult::Done;
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
    }
}
