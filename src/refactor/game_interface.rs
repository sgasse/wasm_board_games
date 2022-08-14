use super::board::{Board, Cell};
use super::t3_game::{T3GameState, T3Move};
use super::tree_evaluator::TreeEvaluator;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub struct T3GameInterface {
    tree_eval: TreeEvaluator<T3GameState>,
}

#[wasm_bindgen]
impl T3GameInterface {
    pub fn new() -> Self {
        let init_state = T3GameState::new(
            Board::new(3, 3),
            T3Move {
                row: 0,
                col: 0,
                side: Cell::X,
            },
        );
        console::log_1(&"Initialized a new T3GameInterface".into());
        Self {
            tree_eval: TreeEvaluator::new(init_state),
        }
    }
}

// Send best move
// Track move
// Reset
