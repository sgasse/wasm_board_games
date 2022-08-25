importScripts('./pkg/wasm_board_games.js')

const {
  FiarGameInterface,
  T3GameInterface,
  BoardMove,
  ExpandResult,
} = wasm_bindgen

async function run_worker(gameName) {
  await wasm_bindgen('./pkg/wasm_board_games_bg.wasm')
  console.log('In worker')

  var gameIf = null

  if (gameName == 't3') {
    gameIf = T3GameInterface.new()
  } else if (gameName == 'fiar') {
    gameIf = FiarGameInterface.new()
  } else {
    throw `Unknown gameName ${gameName}`
  }

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
      // Schedule the expansion of the next level with a timeout so queued,
      // received messages have a chance to be processed before.
      setTimeout(expandGraph, 1)
    }
  }

  // Interrupt the self-rescheduling expansion calls for `task`
  // and resume the expansion afterwards
  function runBetweenExpansion(task) {
    pauseExpansion = true
    task()
    pauseExpansion = false
    setTimeout(expandGraph, 1)
  }

  // Handle incoming messages
  self.onmessage = async (event) => {
    console.log('Got message', event.data)
    const kind = event.data.kind
    if (kind == 'track_move') {
      runBetweenExpansion(() => {
        const lastMove = BoardMove.from_js_value(event.data.lastMove)
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
  setTimeout(expandGraph, 1)
}
