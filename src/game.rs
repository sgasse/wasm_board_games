use super::board::{Board, Cell};
use web_sys::console;

pub struct GameInterface {
    board: Board,
    num_winner: i32,
    next_turn: Cell,
    active: bool,
    cell_size: f64,
}

/// Interface between the frontend and the backend
impl GameInterface {
    pub fn new_ttt() -> GameInterface {
        let board = Board::new(3, 3);

        GameInterface {
            board,
            num_winner: 3i32,
            next_turn: Cell::X,
            active: false,
            cell_size: 100.0,
        }
    }

    pub fn new_fiar() -> GameInterface {
        let board = Board::new(7, 6);

        GameInterface {
            board,
            num_winner: 4i32,
            next_turn: Cell::X,
            active: false,
            cell_size: 100.0,
        }
    }

    pub fn set_cell(&mut self, row: i32, col: i32) -> bool {
        match self.active {
            true => {
                match self.board.set_cell(row, col, self.next_turn.clone()) {
                    Ok(()) => {
                        // Switch turn
                        self.next_turn = match &self.next_turn {
                            Cell::X => Cell::O,
                            Cell::O => Cell::X,
                            _ => Cell::X,
                        };

                        true
                    }
                    Err(_) => false,
                }
            }
            false => false,
        }
    }

    pub fn reset(&mut self) {
        self.board.reset();
        self.next_turn = Cell::X;
        self.active = true;
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn set_cell_size(&mut self, cell_size: f64) {
        self.cell_size = cell_size;
    }

    pub fn cell_size(&self) -> f64 {
        self.cell_size
    }

    pub fn winner(&mut self) -> Cell {
        match self.board.line_count_winner(self.num_winner) {
            Ok(winner) => match winner {
                Cell::Empty => winner,
                player => {
                    self.active = false;
                    player
                }
            },
            Err(msg) => {
                console::log_1(&msg.into());
                Cell::Empty
            }
        }
    }
}
