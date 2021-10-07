use super::common::{Coord, Message};
use super::game_evaluator::{get_best_child_state, GameEvaluator};
use super::game_state::{FiarGameState, GameState, TTTGameState};
use js_sys;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{console, DedicatedWorkerGlobalScope, Worker};

pub fn setup_worker(worker_file: &str) -> Result<Rc<RefCell<Worker>>, &'static str> {
    let worker_handle = match Worker::new(worker_file) {
        Ok(worker) => {
            console::log_1(&"Created a new worker".into());
            worker
        }
        Err(_) => return Err("Could not create worker"),
    };

    Ok(Rc::new(RefCell::new(worker_handle)))
}

macro_rules! gen_worker_if_impl {
    ( $worker_if:ty, $width:expr, $height:expr, $max_depth:expr ) => {
        #[wasm_bindgen]
        impl $worker_if {
            pub fn new() -> Self {
                console::log_1(&"Created a new WorkerInterface".into());

                let evaluator = GameEvaluator::new($width, $height, $max_depth);

                Self {
                    evaluator,
                    should_stop: false,
                }
            }

            pub fn process_msg(&mut self, msg: JsValue) {
                // console::log_1(&"Received something".into());
                let msg: Message = msg.into_serde().unwrap();
                // console::log_2(&"Received row: ".into(), &msg_data.move_coords.row.into());
                // let log_msg = format!("Worker: Received command {:?}", msg.command);
                // console::log_1(&log_msg.into());

                self.should_stop = true;

                match msg {
                    Message::Reset => {
                        self.reset();
                        self.precompute_until_stop();
                    }
                    Message::SetMove(move_coords) => {
                        self.track_move(move_coords);
                        self.precompute_until_stop();
                    }
                    Message::GetMove => {
                        self.send_move();
                        self.precompute_until_stop();
                    }
                }
            }

            fn reset(&mut self) {
                self.should_stop = false;
                self.evaluator.reset();
                console::log_1(&"Worker: Reset done".into());
            }

            fn track_move(&mut self, last_move: Coord) {
                match self.evaluator.track_move(last_move) {
                    Ok(()) => {
                        console::log_1(&"Worker: Tracked moved".into());
                        self.evaluator.init_expansion($max_depth);
                        self.should_stop = false;
                    }
                    Err(_) => console::log_1(&"Worker: Could not track move".into()),
                }
            }

            fn send_move(&mut self) {
                // Evaluate tree
                if let Err(msg) = self
                    .evaluator
                    .evaluate_tree(self.evaluator.get_root_node(), 9)
                {
                    console::log_1(&msg.into());
                    return;
                }

                // Determine best move
                let best_child = match get_best_child_state(self.evaluator.get_root_node()) {
                    Ok(child) => child,
                    Err(_) => {
                        console::log_1(&"Worker: Error doing AI move".into());
                        return;
                    }
                };

                // Do best move
                self.evaluator.set_root_node(Rc::clone(&best_child));

                // Send best move
                let move_coords = (*best_child).borrow().data.last_move();
                let message = Message::SetMove(move_coords);
                let message = JsValue::from_serde(&message).unwrap();

                // Get global scope to post back to main thread
                let global_scope: JsValue = js_sys::global().into();
                let d_scope: DedicatedWorkerGlobalScope = global_scope.into();

                match d_scope.post_message(&message.into()) {
                    Ok(()) => console::log_1(&"Worker: Send AI move".into()),
                    Err(_) => {
                        console::log_1(&"Worker: Error sending AI move".into());
                    }
                }
            }

            fn precompute_until_stop(&mut self) {
                console::log_1(&"Worker: Starting expansion loop".into());
                while !self.should_stop {
                    match self.evaluator.expand_by(100) {
                        Ok(true) => {
                            console::log_1(&"Worker: Expansion done".into());
                            return;
                        }
                        Ok(false) => (),
                        Err(_) => {
                            console::log_1(&"Worker: Error in expansion".into());
                        }
                    }
                }

                console::log_1(&"Worker: Exiting pre-compute".into());
            }
        }
    };
}

#[wasm_bindgen]
pub struct TTTWorkerInterface {
    evaluator: GameEvaluator<TTTGameState>,
    should_stop: bool,
}
gen_worker_if_impl!(TTTWorkerInterface, 3, 3, 9);

#[wasm_bindgen]
pub struct FiarWorkerInterface {
    evaluator: GameEvaluator<FiarGameState>,
    should_stop: bool,
}
gen_worker_if_impl!(FiarWorkerInterface, 7, 6, 6);
