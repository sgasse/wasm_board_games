use super::common::Coord;

#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    X,
    O,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    cells: Vec<Cell>,
    width: i32,
    height: i32,
}

impl Board {
    pub fn new(width: i32, height: i32) -> Board {
        Board {
            cells: vec![Cell::Empty; (width * height) as usize],
            width,
            height,
        }
    }

    pub fn in_bounds(&self, row: i32, col: i32) -> bool {
        if row >= 0 && row < self.height && col >= 0 && col < self.width {
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.cells = vec![Cell::Empty; self.cells.len()];
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn get_index(&self, row: i32, col: i32) -> usize {
        let idx = row * self.width + col;
        idx as usize
    }

    pub fn get_cell(&self, row: i32, col: i32) -> Result<Cell, &'static str> {
        if !self.in_bounds(row, col) {
            return Err("Out of bounds in `get_cell`");
        }

        Ok(self.cells[self.get_index(row, col)].clone())
    }

    pub fn get_first_empty_row_in_col(&self, col: i32) -> Result<i32, &'static str> {
        for row in (0..self.height()).rev() {
            if let Ok(cell) = self.get_cell(row, col) {
                match cell {
                    Cell::Empty => {
                        return Ok(row);
                    }
                    _ => (),
                }
            }
        }

        Err("No empty cell in this column.")
    }

    pub fn set_cell(&mut self, row: i32, col: i32, mark: Cell) -> Result<(), &'static str> {
        if !self.in_bounds(row, col) {
            return Err("Out of bounds in `set_cell`");
        }

        if self.cells[ext_get_index(row, col, self.width)] != Cell::Empty {
            return Err("Cell to set is not empty");
        }

        self.cells[ext_get_index(row, col, self.width)] = mark;

        Ok(())
    }

    /// Set a specific state for the board.
    ///
    /// # Arguments
    ///
    /// * `state` - The state to set as vector of `Cell`.
    ///
    /// Returns a unit-type if the state was set and an error if the state size does not match
    /// the width and height of the board.
    #[allow(dead_code)]
    pub fn set_state(&mut self, state: Vec<Cell>) -> Result<(), String> {
        if state.len() != (self.width * self.height) as usize {
            return Err(String::from("State has the wrong size"));
        }

        self.cells = state;
        Ok(())
    }

    /// Determine if any player has `num_winner` tokens in any line on the board by exhaustively
    /// evaluating all lines.
    ///
    /// # Arguments
    ///
    /// * `num_winner` - The number of tokens required to win.
    ///
    /// Returns the winner of the first line found if there are multiple, starting with rows, upward
    /// pointing diagonals, columns and downward pointing diagonals.
    pub fn line_count_winner(&self, num_winner: i32) -> Result<Cell, &'static str> {
        for r in 0..self.height() {
            // Check rows
            match count_equal(
                &self,
                &Coord { row: r, col: 0 },
                &Coord { row: 0, col: 1 },
                num_winner,
            ) {
                Ok(Cell::Empty) => (),
                Ok(w) => return Ok(w),
                Err(e) => return Err(e),
            }

            // Check diagonal up
            match count_equal(
                &self,
                &Coord { row: r, col: 0 },
                &Coord { row: -1, col: 1 },
                num_winner,
            ) {
                Ok(Cell::Empty) => (),
                Ok(w) => return Ok(w),
                Err(e) => return Err(e),
            }
        }

        for c in 0..self.width() {
            // Check cols
            match count_equal(
                &self,
                &Coord { row: 0, col: c },
                &Coord { row: 1, col: 0 },
                num_winner,
            ) {
                Ok(Cell::Empty) => (),
                Ok(w) => return Ok(w),
                Err(e) => return Err(e),
            }

            // Check diagonal down
            match count_equal(
                &self,
                &Coord { row: 0, col: c },
                &Coord { row: 1, col: 1 },
                num_winner,
            ) {
                Ok(Cell::Empty) => (),
                Ok(w) => return Ok(w),
                Err(e) => return Err(e),
            }
        }

        Ok(Cell::Empty)
    }
}

/// Return which player if any has `num_winner` tokens in a row starting from `row` and `col` and
/// counting in the direction of `d_row` and `d_col`.
///
/// # Arguments
///
/// * `board` - A borrow to a board to evaluate.
/// * `pos` - A borrow to the `Coord` of the position to start at.
/// * `d_pos` - A borrow to the `Coord` of delta values for iterating from the start position.
/// * `num_winner` - The number of tokens required to win.
///
/// Returns the token of the player that won and `Cell::Empty` if nobody won.
fn count_equal(
    board: &Board,
    pos: &Coord,
    d_pos: &Coord,
    num_winner: i32,
) -> Result<Cell, &'static str> {
    let mut count = 0;
    let mut marker = Cell::Empty;

    let Coord {
        row: mut cur_row,
        col: mut cur_col,
    } = pos;

    while board.in_bounds(cur_row, cur_col) {
        let cur_marker = board.get_cell(cur_row, cur_col)?;
        if cur_marker == marker {
            count += 1;
        } else {
            marker = cur_marker;
            count = 1;
        }

        if count >= num_winner {
            return Ok(marker);
        }

        cur_row = cur_row + d_pos.row;
        cur_col = cur_col + d_pos.col;
    }

    Ok(Cell::Empty)
}

pub fn ext_get_index(row: i32, col: i32, width: i32) -> usize {
    let idx = row * width + col;
    idx as usize
}

#[cfg(test)]
mod test {

    use super::{Board, Cell};

    #[test]
    fn test_new_board() {
        let b = Board::new(3, 3);
        assert_eq!(b.width, 3);
        assert_eq!(b.height, 3);
        assert_eq!(b.cells.len(), 9);

        let large_b = Board::new(7, 10);
        assert_eq!(large_b.width, 7);
        assert_eq!(large_b.height, 10);
        assert_eq!(large_b.cells.len(), 70);
    }

    #[test]
    fn test_set_cell() {
        let mut b = Board::new(3, 3);
        for cell in b.cells.iter() {
            assert_eq!(*cell, Cell::Empty);
        }

        let mut res = b.set_cell(1, 1, Cell::X);
        assert_eq!(res, Ok(()));
        assert_eq!(b.cells[b.get_index(1, 1)], Cell::X);

        res = b.set_cell(1, 1, Cell::O);
        assert_eq!(res, Err("Cell to set is not empty"));
        assert_eq!(b.cells[b.get_index(1, 1)], Cell::X);

        res = b.set_cell(2, 1, Cell::O);
        assert_eq!(res, Ok(()));
        assert_eq!(b.cells[b.get_index(2, 1)], Cell::O);
    }

    #[test]
    fn test_set_state() {
        let mut b = Board::new(3, 3);
        for cell in b.cells.iter() {
            assert_eq!(*cell, Cell::Empty);
        }

        let new_state = vec![
            Cell::X,
            Cell::O,
            Cell::Empty,
            Cell::X,
            Cell::Empty,
            Cell::O,
            Cell::X,
            Cell::O,
            Cell::Empty,
        ];
        let res = b.set_state(new_state.clone());
        assert_eq!(res, Ok(()));

        for (i, cell) in b.cells.iter().enumerate() {
            assert_eq!(*cell, new_state[i]);
        }
    }

    #[test]
    fn test_line_count_winner() -> Result<(), &'static str> {
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

        let res = b1.line_count_winner(3)?;
        assert_eq!(res, Cell::X);

        let mut b2 = Board::new(3, 3);
        // XOX
        // OOO
        //  X
        let _ = b2.set_state(vec![
            Cell::X,
            Cell::O,
            Cell::X,
            Cell::O,
            Cell::O,
            Cell::O,
            Cell::Empty,
            Cell::X,
            Cell::Empty,
        ]);

        let res = b2.line_count_winner(3)?;
        assert_eq!(res, Cell::O);

        let mut b3 = Board::new(3, 3);
        // XOX
        // OXO
        //  X
        let _ = b3.set_state(vec![
            Cell::X,
            Cell::O,
            Cell::X,
            Cell::O,
            Cell::X,
            Cell::O,
            Cell::Empty,
            Cell::X,
            Cell::Empty,
        ]);

        let res = b3.line_count_winner(3)?;
        assert_eq!(res, Cell::Empty);

        let mut b4 = Board::new(5, 4);
        //   OXO
        // OOXOX
        // OXOOX
        // XXXOX
        let _ = b4.set_state(vec![
            Cell::Empty,
            Cell::Empty,
            Cell::O,
            Cell::X,
            Cell::O,
            Cell::O,
            Cell::O,
            Cell::X,
            Cell::O,
            Cell::X,
            Cell::O,
            Cell::X,
            Cell::O,
            Cell::O,
            Cell::X,
            Cell::X,
            Cell::X,
            Cell::X,
            Cell::O,
            Cell::X,
        ]);

        let res = b4.line_count_winner(4)?;
        assert_eq!(res, Cell::X);

        Ok(())
    }
}
