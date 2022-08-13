pub mod board;
pub mod t3_game;

pub trait GameState {
    fn expand(&self) -> Vec<Self>
    where
        Self: Sized;
    fn position_value(&self) -> i32;
}
