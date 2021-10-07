
importScripts('./pkg/wasm_board_games.js');

const { TTTWorkerInterface } = wasm_bindgen;

async function run_worker() {
    await wasm_bindgen('./pkg/wasm_board_games_bg.wasm');
    console.log('In worker');

    var worker_if = TTTWorkerInterface.new();
    self.onmessage = async event => {
        var result = worker_if.process_msg(event.data);
    }

}

run_worker();