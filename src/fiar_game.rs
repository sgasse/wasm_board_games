use crate::{Board, BoardMove, Cell, Coords, GameState, X_WIN_VALUE};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct FiarGameState {
    board: Board,
    last_move: BoardMove,
}

#[wasm_bindgen]
impl FiarGameState {
    pub fn new(board: Board, last_move: BoardMove) -> Self {
        Self { board, last_move }
    }

    pub fn side(&self) -> Cell {
        self.last_move.side
    }

    pub fn last_move(&self) -> BoardMove {
        self.last_move
    }
}

impl Default for FiarGameState {
    fn default() -> Self {
        Self {
            board: Board::new(6, 7),
            last_move: BoardMove {
                coords: Coords { row: 0, col: 0 },
                // We usually start with X, so the "last" was O
                side: Cell::O,
            },
        }
    }
}

impl GameState for FiarGameState {
    fn expand(&self) -> Vec<FiarGameState> {
        let next_side = match self.last_move.side {
            Cell::X => Cell::O,
            Cell::O => Cell::X,
            Cell::Empty => panic!("Last move cannot be empty!"),
        };

        // We cannot have more children states as columns
        let mut next_states: Vec<FiarGameState> = Vec::with_capacity(self.board.width() as usize);

        'column_loop: for col in 0..self.board.width() {
            // We start from the bottom going up looking for free fields per
            // column.
            for row in (0..self.board.height()).rev() {
                if let Ok(Cell::Empty) = self.board.get_cell(row, col) {
                    let mut new_board = self.board.clone();
                    new_board.set_cell(row, col, next_side.clone());

                    next_states.push(FiarGameState {
                        board: new_board,
                        last_move: BoardMove {
                            coords: Coords { row, col },
                            side: next_side.clone(),
                        },
                    });

                    // After finding the first free cell, we are done with this
                    // column.
                    continue 'column_loop;
                }
            }
        }

        next_states
    }

    fn position_value(&self) -> i32 {
        match self.board.line_winner(&self.last_move.coords, 4) {
            Cell::X => return X_WIN_VALUE,
            Cell::O => return -X_WIN_VALUE,
            Cell::Empty => return 0,
        }
    }

    fn side(&self) -> Cell {
        self.last_move.side
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_fiargamestate_expand() -> Result<(), ()> {
        let b1 = Board::new(6, 7);

        let game_state = FiarGameState {
            board: b1,
            last_move: BoardMove {
                coords: Coords { row: 0, col: 0 },
                side: Cell::O,
            },
        };

        let expanded_states = game_state.expand();
        assert_eq!(expanded_states.len(), 7);

        for state in expanded_states {
            assert_eq!(state.last_move().coords.row, state.board.height() - 1);
            for row in 0..state.board.height() {
                for col in 0..state.board.width() {
                    match row {
                        x if x == (state.board.height() - 1) => {
                            let cell = state.board.get_cell(row, col)?;
                            assert!(cell == Cell::Empty || cell == Cell::X);
                        }
                        _ => {
                            assert_eq!(state.board.get_cell(row, col), Ok(Cell::Empty))
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
