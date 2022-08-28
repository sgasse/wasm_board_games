use crate::{Cell, Coords, DeltaCoords};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    cells: Vec<Cell>,
    width: u32,
    height: u32,
}

impl Board {
    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn get_cell(&self, row: u32, col: u32) -> Result<Cell, ()> {
        match self.in_bounds(row, col) {
            true => Ok(self.cells.get(self.get_index(row, col)).unwrap().clone()),
            false => Err(()),
        }
    }

    pub fn set_state(&mut self, state: Vec<Cell>) -> Result<(), ()> {
        if state.len() != (self.width * self.height) as usize {
            return Err(());
        }

        self.cells = state;
        Ok(())
    }
}

#[wasm_bindgen]
impl Board {
    pub fn new(height: u32, width: u32) -> Self {
        Board {
            cells: vec![Cell::Empty; (width * height) as usize],
            width,
            height,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells_ptr(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn set_cell(&mut self, row: u32, col: u32, mark: Cell) -> bool {
        if !self.in_bounds(row, col) {
            return false;
        }

        if self.cells[self.get_index(row, col)] != Cell::Empty {
            return false;
        }

        let cell_idx = self.get_index(row, col);
        self.cells[cell_idx] = mark;
        true
    }

    pub fn get_index(&self, row: u32, col: u32) -> usize {
        let idx = row * self.width + col;
        idx as usize
    }

    pub fn in_bounds(&self, row: u32, col: u32) -> bool {
        if row < self.height && col < self.width {
            true
        } else {
            false
        }
    }

    pub fn get_coords(&self, idx: usize) -> Coords {
        let idx = idx as u32;
        Coords {
            row: idx / self.width,
            col: idx % self.width,
        }
    }

    pub fn reset(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = Cell::Empty;
        }
    }

    /// Determine the winner on the lines through `self.last_move`.
    ///
    /// This assumes that there is no winning pattern on any other line which
    /// does not go through `self.last_move`. This is a reasonable assumption
    /// if every game state is evaluated directly, thus a previously completed
    /// pattern on another line would have been detected before.
    pub fn line_winner(&self, last_move: &Coords, num_winner: i32) -> Cell {
        // To determine the potential winner, we check the horizontal, vertial,
        // diagonal-down and diagonal-up lines through `self.last_move`.

        // Find the start point by substracting the minimum distance from
        // both the row and the column.
        // For point a (1, 2), the start of the diagonal down is s (0, 1)
        // | |s| | |
        // | | |a| |
        // | | | | |
        let diag_down_min_dist = u32::min(last_move.row, last_move.col);
        let diag_down_start = Coords {
            row: last_move.row - diag_down_min_dist,
            col: last_move.col - diag_down_min_dist,
        };

        // Find the start point by substracting the minimum distance from the
        // column and *adding* the minimum distance to the row. For the
        // row-part, we take the distance to the height into account since it is
        // the diagonal up.
        // For point a (1, 2), the start of the diagonal up is s (2, 1).
        // | | | | |
        // | | |a| |
        // | |s| | |
        let diag_up_min_dist = u32::min(self.height - 1 - last_move.row, last_move.col);
        let diag_up_start = Coords {
            row: last_move.row + diag_up_min_dist,
            col: last_move.col - diag_up_min_dist,
        };

        let pos_d_pos_pairs = vec![
            (
                // Horizontal
                Coords {
                    row: last_move.row,
                    col: 0,
                },
                DeltaCoords { row: 0, col: 1 },
            ),
            (
                // Vertical
                Coords {
                    row: 0,
                    col: last_move.col,
                },
                DeltaCoords { row: 1, col: 0 },
            ),
            (
                // Diagonal down
                diag_down_start,
                DeltaCoords { row: 1, col: 1 },
            ),
            (
                // Diagonal up
                diag_up_start,
                DeltaCoords { row: -1, col: 1 },
            ),
        ];

        for (pos, d_pos) in pos_d_pos_pairs {
            let line_winner = side_with_min_equal(&self, &pos, &d_pos, num_winner);
            match line_winner {
                Cell::Empty => continue,
                side => return side,
            };
        }

        Cell::Empty
    }

    pub fn first_empty_in_column(&self, col: u32) -> Coords {
        for row in (0..self.height()).rev() {
            if let Ok(Cell::Empty) = self.get_cell(row, col) {
                return Coords { row, col };
            }
        }

        Coords { row: 0, col }
    }
}

fn side_with_min_equal(board: &Board, pos: &Coords, d_pos: &DeltaCoords, num_winner: i32) -> Cell {
    let mut count = 0;
    let mut marker = Cell::Empty;

    let Coords {
        row: mut cur_row,
        col: mut cur_col,
    } = pos;

    while board.in_bounds(cur_row, cur_col) {
        let cur_marker = board.get_cell(cur_row, cur_col).unwrap();
        if cur_marker == marker {
            count += 1;
        } else {
            marker = cur_marker;
            count = 1;
        }

        if (count >= num_winner) && (marker != Cell::Empty) {
            return marker;
        }

        cur_row = (cur_row as i32 + d_pos.row) as u32;
        cur_col = (cur_col as i32 + d_pos.col) as u32;
    }

    Cell::Empty
}

#[cfg(test)]

mod test {

    use super::{Board, Cell, Coords};

    #[test]
    fn test_get_coords() {
        let board = Board::new(3, 4);

        for (idx, coords) in vec![
            (0, Coords { row: 0, col: 0 }),
            (1, Coords { row: 0, col: 1 }),
            (6, Coords { row: 1, col: 2 }),
        ] {
            assert_eq!(board.get_coords(idx), coords);
        }
    }

    #[test]
    fn test_line_winner() {
        let mut b1 = Board::new(3, 3);
        // XO
        // OXO
        //  XX
        let _ = b1.set_state(vec![
            Cell::X,
            Cell::O,
            Cell::Empty,
            Cell::O,
            Cell::X,
            Cell::O,
            Cell::Empty,
            Cell::X,
            Cell::X,
        ]);

        let last_move_coords = Coords { row: 1, col: 1 };

        assert_eq!(b1.line_winner(&last_move_coords, 3), Cell::X);
    }
}
