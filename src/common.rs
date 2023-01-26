use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Cell {
    Empty,
    X,
    O,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BoardMove {
    pub coords: Coords,
    pub side: Cell,
}

#[wasm_bindgen]
impl BoardMove {
    pub fn new(row: u32, col: u32, side: Cell) -> Self {
        Self {
            coords: Coords { row, col },
            side,
        }
    }

    pub fn from_js_value(js_value: JsValue) -> Self {
        serde_wasm_bindgen::from_value(js_value).unwrap()
    }

    pub fn to_js_value(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}
