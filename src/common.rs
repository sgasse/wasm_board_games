use serde::{Deserialize, Serialize};

/// Coordinates on the board
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Coord {
    pub row: i32,
    pub col: i32,
}

/// Message send between the main thread and the worker
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Message {
    Reset,
    SetMove(Coord),
    GetMove,
}
