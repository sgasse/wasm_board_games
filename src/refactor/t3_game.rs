use super::board::{Board, Cell, Coords};
use super::GameState;
use super::X_WIN_VALUE;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct T3Move {
    pub coords: Coords,
    pub side: Cell,
}

#[wasm_bindgen]
impl T3Move {
    pub fn new(row: u32, col: u32, side: Cell) -> Self {
        Self {
            coords: Coords { row, col },
            side,
        }
    }

    pub fn from_js_value(js_value: JsValue) -> Self {
        js_value.into_serde().unwrap()
    }

    pub fn to_js_value(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct T3GameState {
    board: Board,
    last_move: T3Move,
}

#[wasm_bindgen]
impl T3GameState {
    pub fn new(board: Board, last_move: T3Move) -> Self {
        Self { board, last_move }
    }

    pub fn default() -> Self {
        Self {
            board: Board::new(3, 3),
            last_move: T3Move {
                coords: Coords { row: 0, col: 0 },
                // We usually start with X, so the "last" was O
                side: Cell::O,
            },
        }
    }

    pub fn side(&self) -> Cell {
        self.last_move.side
    }

    pub fn last_move(&self) -> T3Move {
        self.last_move
    }
}

impl GameState for T3GameState {
    fn expand(&self) -> Vec<T3GameState> {
        let next_side = match self.last_move.side {
            Cell::X => Cell::O,
            Cell::O => Cell::X,
            Cell::Empty => panic!("Last move cannot be empty!"),
        };

        let next_states: Vec<T3GameState> = self
            .board
            .cells()
            .iter()
            .enumerate()
            .filter_map(|(idx, cell)| {
                if let Cell::Empty = cell {
                    let mut new_board = self.board.clone();
                    let Coords { row, col } = new_board.get_coords(idx);
                    new_board.set_cell(row, col, next_side.clone());

                    return Some(T3GameState {
                        board: new_board,
                        last_move: T3Move {
                            coords: Coords { row, col },
                            side: next_side.clone(),
                        },
                    });
                }

                None
            })
            .collect();
        next_states
    }

    fn position_value(&self) -> i32 {
        match self.board.line_winner(&self.last_move.coords, 3) {
            Cell::X => return X_WIN_VALUE,
            Cell::O => return -X_WIN_VALUE,
            Cell::Empty => return 0,
        }
    }
}

#[cfg(test)]
mod test {

    use super::Cell;
    use super::T3Move;
    use super::{Board, T3GameState};
    use crate::refactor::{board::Coords, GameState};

    #[test]
    fn test_t3gamestate_expand() {
        let mut b1 = Board::new(3, 3);
        // X X
        // O O
        // X O
        let _ = b1.set_state(vec![
            Cell::X,
            Cell::Empty,
            Cell::X,
            Cell::O,
            Cell::Empty,
            Cell::O,
            Cell::X,
            Cell::Empty,
            Cell::O,
        ]);

        let game_state = T3GameState {
            board: b1,
            last_move: T3Move {
                coords: Coords { row: 2, col: 2 },
                side: Cell::O,
            },
        };

        let expanded_states = game_state.expand();
        assert_eq!(expanded_states.len(), 3);
    }
}
