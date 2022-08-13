pub mod board;
pub mod t3_game;
pub mod tree_evaluator;

pub const X_WIN_VALUE: i32 = 1000000;

pub trait GameState {
    fn expand(&self) -> Vec<Self>
    where
        Self: Sized;
    fn position_value(&self) -> i32;
}
