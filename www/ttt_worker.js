importScripts('./pkg/wasm_board_games.js')

const { T3GameInterface, T3Move, ExpandResult } = wasm_bindgen

async function run_worker() {
  await wasm_bindgen('./pkg/wasm_board_games_bg.wasm')
  console.log('In worker')

  const game_if = T3GameInterface.new()

  const do_expand = async () => {
    const expandResult = game_if.expand_one_level()
    if (expandResult != ExpandResult.Done) {
      setTimeout(do_expand, 10)
    }
  }

  self.onmessage = async (event) => {
    console.log('Got message', event.data)
    const kind = event.data.kind
    if (kind == 'track_move') {
      const lastMove = T3Move.from_js_value(event.data.lastMove)
      game_if.track_move(lastMove)
    } else if (kind == 'reset') {
      game_if.reset()
    } else if (kind == 'get_best_move') {
      const bestMove = game_if.get_best_move()
      this.postMessage({ kind: 'best_move', bestMove: bestMove.to_js_value() })
    }
  }

  setTimeout(do_expand, 10)
}

run_worker()
