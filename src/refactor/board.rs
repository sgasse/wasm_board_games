use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    X,
    O,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Coord {
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

    pub fn get_coords(&self, idx: usize) -> (u32, u32) {
        let idx = idx as u32;
        (idx % self.width, idx / self.height)
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
}
