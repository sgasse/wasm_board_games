use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Cell {
    Empty,
    X,
    O,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Coords {
    pub row: u32,
    pub col: u32,
}

#[wasm_bindgen]
impl Coords {
    pub fn new(row: u32, col: u32) -> Self {
        Self { row, col }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeltaCoords {
    pub row: i32,
    pub col: i32,
}

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
    pub fn new(width: u32, height: u32) -> Self {
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
}

#[cfg(test)]

mod test {

    use super::{Board, Coords};

    #[test]
    fn test_get_coords() {
        let board = Board::new(4, 3);

        for (idx, coords) in vec![
            (0, Coords { row: 0, col: 0 }),
            (1, Coords { row: 0, col: 1 }),
            (6, Coords { row: 1, col: 2 }),
        ] {
            assert_eq!(board.get_coords(idx), coords);
        }
    }
}
