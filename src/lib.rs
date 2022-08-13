mod board;
mod common;
mod frontend;
mod game;
mod game_evaluator;
mod game_state;
mod refactor;
mod tree;
mod utils;
mod worker;

use frontend::{
    draw_grid, set_canvas_size, setup_ai_move_button, setup_delayed_reset, setup_fiar_canvas_click,
    setup_reset_button, setup_ttt_canvas_click, setup_worker_on_msg_callback,
};
use game::GameInterface;
use std::cell::RefCell;
use std::rc::Rc;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use worker::setup_worker;

pub use refactor::board::{Board, Cell};

// Called by our JS entry point to run TicTacTo
#[wasm_bindgen]
pub fn run_tictacto() -> Result<(), JsValue> {
    set_panic_hook();

    // Create worker and store handle
    let worker_handle = setup_worker("./ttt_worker.js")?;

    // Create shared GameInterface
    let shared_game_if = Rc::new(RefCell::new(GameInterface::new_ttt()));

    set_canvas_size(Rc::clone(&shared_game_if));

    {
        let game_if_ = Rc::clone(&shared_game_if);
        let game_if_ = &*game_if_.borrow();
        draw_grid(&game_if_).expect("Could not draw grid");
    }

    setup_ttt_canvas_click(Rc::clone(&shared_game_if), Rc::clone(&worker_handle));
    setup_reset_button(
        Rc::clone(&shared_game_if),
        Rc::clone(&worker_handle),
        "Tic Tac To",
    );
    setup_delayed_reset(
        Rc::clone(&shared_game_if),
        Rc::clone(&worker_handle),
        "Tic Tac To",
    );
    setup_ai_move_button(Rc::clone(&worker_handle));
    setup_worker_on_msg_callback(Rc::clone(&shared_game_if), Rc::clone(&worker_handle));

    Ok(())
}

// Called by our JS entry point to run Four in a row
#[wasm_bindgen]
pub fn run_four_in_a_row() -> Result<(), JsValue> {
    set_panic_hook();

    // Create worker and store handle
    let worker_handle = setup_worker("./fiar_worker.js")?;

    // Create shared GameInterface
    let shared_game_if = Rc::new(RefCell::new(GameInterface::new_fiar()));

    set_canvas_size(Rc::clone(&shared_game_if));

    {
        let game_if_ = Rc::clone(&shared_game_if);
        let game_if_ = &*game_if_.borrow();
        draw_grid(&game_if_).expect("Could not draw grid");
    }

    setup_fiar_canvas_click(Rc::clone(&shared_game_if), Rc::clone(&worker_handle));
    setup_reset_button(
        Rc::clone(&shared_game_if),
        Rc::clone(&worker_handle),
        "Four in a row",
    );
    setup_delayed_reset(
        Rc::clone(&shared_game_if),
        Rc::clone(&worker_handle),
        "Four in a row",
    );
    setup_ai_move_button(Rc::clone(&worker_handle));
    setup_worker_on_msg_callback(Rc::clone(&shared_game_if), Rc::clone(&worker_handle));

    Ok(())
}
