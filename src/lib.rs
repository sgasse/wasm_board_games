mod board;
mod common;
mod fiar_game;
mod game_interface;
mod t3_game;
mod tree_evaluator;

pub use {
    board::Board,
    common::{BoardMove, Cell, Coords, DeltaCoords},
    fiar_game::FiarGameState,
    game_interface::{ExpandResult, FiarGameInterface, T3GameInterface},
    t3_game::T3GameState,
    tree_evaluator::TreeEvaluator,
};

pub const X_WIN_VALUE: i32 = 1000000;

pub trait GameState {
    fn expand(&self) -> Vec<Self>
    where
        Self: Sized;
    fn position_value(&self) -> i32;
}
