use super::board::Cell;
use super::common::{Coord, Message};
use super::game::GameInterface;
use std::cell::{RefCell, RefMut};
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, HtmlElement, MessageEvent, MouseEvent, Worker};

const GRID_COLOR: &str = "#4d4d4d";
const EMPTY_COLOR: &str = "#000000";
const X_COLOR: &str = "#49b5bf";
const O_COLOR: &str = "#c9c900";

pub fn set_canvas_size(game_if: Rc<RefCell<GameInterface>>) {
    {
        // Setting the cell size requires a mutable borrow - moving into a block to allow for
        // immutable borrows of GameInterface later
        let mut game_if_ = (*game_if).borrow_mut();
        let window = window().unwrap();
        let board = game_if_.board();
        let inner_width = window.inner_width().unwrap().as_f64().unwrap() as u32;
        let inner_height = window.inner_height().unwrap().as_f64().unwrap() as u32;

        // Use 80% of the width or 66% of the height
        let cell_size = 4 * std::cmp::min(
            (inner_width / 5) / board.width() as u32,
            (inner_height / 6) / board.height() as u32,
        );
        game_if_.set_cell_size(cell_size as f64);
    }

    let game_if_ = (*game_if).borrow();
    let board = game_if_.board();
    let cell_size = game_if_.cell_size() as u32;

    let width = board.width() as u32 * (cell_size + 2) + 2;
    let height = board.height() as u32 * (cell_size + 2) + 2;

    let canvas = get_canvas("board").unwrap();
    canvas.set_width(width);
    canvas.set_height(height);
}

pub fn setup_ttt_canvas_click(
    game_if: Rc<RefCell<GameInterface>>,
    worker_handle: Rc<RefCell<Worker>>,
) {
    let callback = Closure::wrap(Box::new(move |event: MouseEvent| {
        let mut game_if_ = (*game_if).borrow_mut();
        let cell_size = game_if_.cell_size();
        let coord =
            canvas_pos_to_coord(event.offset_x() as f64, event.offset_y() as f64, cell_size);

        let msg = Message::SetMove(coord.clone());

        if let true = game_if_.set_cell(coord.row, coord.col) {
            let worker = &*worker_handle.borrow_mut();
            let msg = JsValue::from_serde(&msg).unwrap();
            match worker.post_message(&msg.into()) {
                Ok(()) => {
                    draw_grid(&game_if_).unwrap();
                    draw_cells(&game_if_).unwrap();
                    maybe_set_winner(game_if_);
                }
                Err(_) => console::log_1(&"Error sending a message to the worker".into()),
            }
        }

        log_debug_pos(event.offset_x(), event.offset_y(), cell_size);
    }) as Box<dyn FnMut(_)>);

    set_onclick_callback("board", &callback);

    // Leaks memory
    callback.forget();
}

pub fn setup_fiar_canvas_click(
    game_if: Rc<RefCell<GameInterface>>,
    worker_handle: Rc<RefCell<Worker>>,
) {
    let callback = Closure::wrap(Box::new(move |event: MouseEvent| {
        let mut game_if_ = (*game_if).borrow_mut();
        let cell_size = game_if_.cell_size();
        let mut coord =
            canvas_pos_to_coord(event.offset_x() as f64, event.offset_y() as f64, cell_size);

        {
            match game_if_.board().get_first_empty_row_in_col(coord.col) {
                Ok(row) => {
                    coord.row = row;

                    let msg = Message::SetMove(coord.clone());

                    if let true = game_if_.set_cell(coord.row, coord.col) {
                        let worker = &*worker_handle.borrow_mut();
                        let msg = JsValue::from_serde(&msg).unwrap();
                        match worker.post_message(&msg.into()) {
                            Ok(()) => {
                                draw_grid(&game_if_).unwrap();
                                draw_cells(&game_if_).unwrap();
                                maybe_set_winner(game_if_);
                            }
                            Err(_) => {
                                console::log_1(&"Error sending a message to the worker".into())
                            }
                        }
                    }
                }
                Err(_) => (),
            }
        }

        log_debug_pos(event.offset_x(), event.offset_y(), cell_size);
    }) as Box<dyn FnMut(_)>);

    set_onclick_callback("board", &callback);

    // Leaks memory
    callback.forget();
}

pub fn maybe_set_winner(mut game_if: RefMut<GameInterface>) {
    match game_if.winner() {
        Cell::Empty => (),
        Cell::X => {
            let msg = "Blue won!";
            console::log_1(&msg.into());
            set_text_field("notification", msg);
        }
        Cell::O => {
            let msg = "Yellow won!";
            console::log_1(&msg.into());
            set_text_field("notification", msg);
        }
    }
}

pub fn set_text_field(elem_name: &str, msg: &str) {
    let document = window().unwrap().document().unwrap();
    document
        .get_element_by_id(elem_name)
        .expect(&format!("#{} should exist", elem_name))
        .dyn_ref::<HtmlElement>()
        .expect(&format!("#{} should be a HtmlElement", elem_name))
        .set_inner_text(msg);
}

pub fn setup_worker_on_msg_callback(
    game_if: Rc<RefCell<GameInterface>>,
    worker_handle: Rc<RefCell<Worker>>,
) {
    let callback = Closure::wrap(Box::new(move |event: MessageEvent| {
        console::log_1(&"Frontend: Received response from worker".into());

        match event.data().into_serde::<Message>() {
            Ok(msg) => {
                let mut game_if_ = (*game_if).borrow_mut();

                if let Message::SetMove(move_coords) = msg {
                    match game_if_.set_cell(move_coords.row, move_coords.col) {
                        true => {
                            draw_grid(&game_if_).unwrap();
                            draw_cells(&game_if_).unwrap();

                            maybe_set_winner(game_if_)
                        }
                        false => console::log_1(
                            &"Frontend: Could not set coordinates of received move".into(),
                        ),
                    }
                }
            }
            Err(_) => console::log_1(&"Frontend: Could not deserialize received message".into()),
        };
    }) as Box<dyn FnMut(_)>);

    let worker_handle = &*worker_handle.borrow();
    worker_handle.set_onmessage(Some(callback.as_ref().unchecked_ref()));

    // Leaks memory
    callback.forget();
}

pub fn get_reset_closure(
    game_if: Rc<RefCell<GameInterface>>,
    worker_handle: Rc<RefCell<Worker>>,
    init_notification: &'static str,
) -> Closure<dyn FnMut()> {
    Closure::wrap(Box::new(move || {
        {
            let game_if_ = &mut *game_if.borrow_mut();
            game_if_.reset();
            set_text_field("notification", init_notification);
            draw_grid(game_if_).unwrap();
            draw_cells(game_if_).unwrap();
            {
                let msg = Message::Reset;
                let worker = &*worker_handle.borrow_mut();
                let msg = JsValue::from_serde(&msg).unwrap();
                if let Err(_) = worker.post_message(&msg.into()) {
                    console::log_1(&"Error sending reset message to worker".into());
                }
            }
        }

        console::log_1(&"Reset".into());
    }) as Box<dyn FnMut()>)
}

pub fn setup_delayed_reset(
    game_if: Rc<RefCell<GameInterface>>,
    worker_handle: Rc<RefCell<Worker>>,
    init_notification: &'static str,
) {
    let callback = get_reset_closure(
        Rc::clone(&game_if),
        Rc::clone(&worker_handle),
        init_notification,
    );

    window()
        .expect("Should have a window in this context")
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            500,
        )
        .expect("Should be able to setup callback with timeout");

    callback.forget();
}

pub fn setup_reset_button(
    game_if: Rc<RefCell<GameInterface>>,
    worker_handle: Rc<RefCell<Worker>>,
    init_notification: &'static str,
) {
    let callback = get_reset_closure(game_if, worker_handle, init_notification);

    set_onclick_callback("resetButton", &callback);

    // Leaks memory
    callback.forget();
}

pub fn setup_ai_move_button(worker_handle: Rc<RefCell<Worker>>) {
    let callback = Closure::wrap(Box::new(move || {
        {
            let msg = Message::GetMove;
            let worker = &*worker_handle.borrow_mut();
            let msg = JsValue::from_serde(&msg).unwrap();
            if let Err(_) = worker.post_message(&msg.into()) {
                console::log_1(&"Error requesting move from worker".into());
            }
        }

        console::log_1(&"AI move".into());
    }) as Box<dyn FnMut()>);

    set_onclick_callback("aiMove", &callback);

    // Leaks memory
    callback.forget();
}

pub fn set_onclick_callback<T: ?Sized>(element_name: &str, callback: &Closure<T>) {
    let document = web_sys::window().unwrap().document().unwrap();
    document
        .get_element_by_id(element_name)
        .expect("element should exist")
        .dyn_ref::<HtmlElement>()
        .expect("element should match type")
        .set_onclick(Some(callback.as_ref().unchecked_ref()));
}

fn log_debug_pos(x: i32, y: i32, cell_size: f64) {
    console::log_2(&"Offset X: ".into(), &x.into());
    console::log_2(&"Offset Y: ".into(), &y.into());
    let coord = canvas_pos_to_coord(x as f64, y as f64, cell_size);
    console::log_2(&"Row: ".into(), &coord.row.into());
    console::log_2(&"Col: ".into(), &coord.col.into());
}

fn canvas_pos_to_coord(x: f64, y: f64, cell_size: f64) -> Coord {
    let row = ((y - 1.0) / (cell_size + 1.0)).floor() as i32;
    let col = ((x - 1.0) / (cell_size + 1.0)).floor() as i32;
    Coord { row, col }
}

pub fn draw_grid(game_if: &GameInterface) -> Result<(), &'static str> {
    let width = game_if.board().width();
    let height = game_if.board().height();
    let cell_size = game_if.cell_size();

    let canvas = get_canvas("board")?;
    let ctx = get_2d_context(&canvas)?;

    // Clear context
    ctx.clear_rect(
        0 as f64,
        0 as f64,
        canvas.width() as f64,
        canvas.height() as f64,
    );

    ctx.set_line_width(2.0);
    ctx.begin_path();
    ctx.set_stroke_style(&wasm_bindgen::JsValue::from_str(GRID_COLOR));

    // Vertical lines
    for i in 0..=width {
        ctx.move_to(i as f64 * (cell_size + 2.0) + 2.0, 0.0);
        ctx.line_to(
            i as f64 * (cell_size + 2.0) + 2.0,
            (cell_size + 2.0) * height as f64 + 2.0,
        );
    }

    // Horizontal lines
    for j in 0..=height {
        ctx.move_to(0.0, j as f64 * (cell_size + 2.0) + 2.0);
        ctx.line_to(
            (cell_size + 2.0) * width as f64 + 2.0,
            j as f64 * (cell_size + 2.0) + 2.0,
        );
    }

    ctx.stroke();

    Ok(())
}

pub fn draw_cells(game_if: &GameInterface) -> Result<(), &str> {
    let board = game_if.board();
    let cell_size = game_if.cell_size();

    let ctx = get_2d_context(&get_canvas("board")?)?;

    ctx.begin_path();

    for row in 0..board.height() {
        for col in 0..board.width() {
            match board.get_cell(row, col).expect("Cell should exist") {
                Cell::Empty => ctx.set_fill_style(&wasm_bindgen::JsValue::from_str(EMPTY_COLOR)),
                Cell::X => ctx.set_fill_style(&wasm_bindgen::JsValue::from_str(X_COLOR)),
                Cell::O => ctx.set_fill_style(&wasm_bindgen::JsValue::from_str(O_COLOR)),
            }

            ctx.fill_rect(
                col as f64 * (cell_size + 2.0) + 2.0,
                row as f64 * (cell_size + 2.0) + 2.0,
                cell_size,
                cell_size,
            )
        }
    }

    Ok(())
}

fn get_canvas(canvas_name: &str) -> Result<web_sys::HtmlCanvasElement, &'static str> {
    let document = match web_sys::window() {
        Some(document) => match document.document() {
            Some(document) => document,
            None => return Err("Could not get document"),
        },
        None => return Err("Could not get document"),
    };

    let canvas = match document.get_element_by_id(canvas_name) {
        Some(canvas) => canvas,
        None => return Err("Could not get canvas"),
    };

    canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| "Could not cast canvas")
}

fn get_2d_context(
    canvas: &web_sys::HtmlCanvasElement,
) -> Result<web_sys::CanvasRenderingContext2d, &'static str> {
    match canvas.get_context("2d") {
        Ok(ctx) => match ctx {
            Some(ctx) => {
                return ctx
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .map_err(|_| "Could not cast context")
            }
            None => return Err("Could not get context"),
        },
        Err(_) => return Err("Could not get context"),
    }
}
