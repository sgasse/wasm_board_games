pub mod board;
pub mod t3_game;

pub trait GameState {
    fn expand<T: GameState>(&self) -> Vec<T>;
    fn position_value(&self) -> i32;
}
