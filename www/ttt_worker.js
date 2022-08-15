importScripts('./pkg/wasm_board_games.js')

const { T3GameInterface, T3Move, ExpandResult } = wasm_bindgen

async function run_worker() {
  await wasm_bindgen('./pkg/wasm_board_games_bg.wasm')
  console.log('In worker')

  const gameIf = T3GameInterface.new()
  var pauseExpansion = false

  // Expand the graph if it is not completed (to the desired depth)
  // or there is a message to interrupt the expansion
  const expandGraph = async () => {
    const startTime = new Date()
    const expandResult = gameIf.expand_one_level()
    console.log(
      `Expansion took`,
      new Date().getTime() - startTime.getTime(),
      'ms',
    )
    if (expandResult != ExpandResult.Done && !pauseExpansion) {
      setTimeout(expandGraph, 10)
    }
  }

  // Interrupt the self-rescheduling expansion calls for `task`
  // and resume the expansion afterwards
  function runBetweenExpansion(task) {
    pauseExpansion = true
    task()
    pauseExpansion = false
    setTimeout(expandGraph, 10)
  }

  // Handle incoming messages
  self.onmessage = async (event) => {
    console.log('Got message', event.data)
    const kind = event.data.kind
    if (kind == 'track_move') {
      runBetweenExpansion(() => {
        const lastMove = T3Move.from_js_value(event.data.lastMove)
        gameIf.track_move(lastMove)
      })
    } else if (kind == 'reset') {
      runBetweenExpansion(() => {
        gameIf.reset()
      })
    } else if (kind == 'get_best_move') {
      runBetweenExpansion(() => {
        const bestMove = gameIf.get_best_move()
        this.postMessage({
          kind: 'best_move',
          bestMove: bestMove.to_js_value(),
        })
      })
    }
  }

  // Kick off initial expansion
  setTimeout(expandGraph, 10)
}

run_worker()
