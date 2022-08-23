mod board;
pub use board::Board;

mod common;
pub use common::{BoardMove, Cell, Coords, DeltaCoords};

mod fiar_game;
pub use fiar_game::FiarGameState;

mod game_interface;
pub use game_interface::{ExpandResult, FiarGameInterface, T3GameInterface};

mod t3_game;
pub use t3_game::T3GameState;

mod tree_evaluator;
pub use tree_evaluator::TreeEvaluator;

pub const X_WIN_VALUE: i32 = 1000000;

pub trait GameState {
    fn expand(&self) -> Vec<Self>
    where
        Self: Sized;
    fn position_value(&self) -> i32;
}
