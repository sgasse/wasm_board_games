importScripts('./pkg/wasm_board_games.js')

const { T3GameInterface } = wasm_bindgen

async function run_worker() {
  await wasm_bindgen('./pkg/wasm_board_games_bg.wasm')
  console.log('In worker')

  const game_if = T3GameInterface.new()
  self.onmessage = async (event) => {
    console.log('Got message', event.data)
  }
}

run_worker()
