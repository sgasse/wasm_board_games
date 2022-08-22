mod board;
mod game_interface;
mod t3_game;
mod tree_evaluator;

pub use {
    board::{Board, Cell},
    game_interface::{ExpandResult, T3GameInterface},
    t3_game::T3Move,
};

pub const X_WIN_VALUE: i32 = 1000000;

pub trait GameState {
    fn expand(&self) -> Vec<Self>
    where
        Self: Sized;
    fn position_value(&self) -> i32;
}
