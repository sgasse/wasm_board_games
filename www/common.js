// Make WASM structs available
const { Board, Cell, BoardMove, Coords } = wasm_bindgen

const BOARD_GAP_SIZE = 4
const BOARD_PADDING = 4
const CELL_SIZE = 80

// There global variables will be set after loading WASM
var gBoard = null
var gCells = null
var lastMove = null
var gWorker = null
var gameActive = true

// Setup a board with the given number of rows and columns.
function setupBoard(rows, cols) {
  const numFields = rows * cols
  createFields(numFields)

  const boardDiv = document.getElementById('board')
  boardDiv.style.width = `${
    cols * CELL_SIZE + (cols - 1) * BOARD_GAP_SIZE + 1 * BOARD_PADDING
  }px`
  boardDiv.style.height = `${
    rows * CELL_SIZE + (rows - 1) * BOARD_GAP_SIZE + 1 * BOARD_PADDING
  }px`
  boardDiv.style.gap = `${BOARD_GAP_SIZE}px`
  boardDiv.style.padding = `${BOARD_PADDING}px`
}

// Create field HTML elements dynamically based on the number of fields.
function createFields(numFields) {
  for (var i = 0; i < numFields; i++) {
    const fieldDiv = document.createElement('div')
    fieldDiv.id = `field_${i}`
    fieldDiv.className = 'board-field rounded-corners'
    fieldDiv.style.width = `${CELL_SIZE}px`
    fieldDiv.style.height = `${CELL_SIZE}px`
    fieldDiv.onclick = clickField
    document.getElementById('board').appendChild(fieldDiv)
    console.log('Added element', i)
  }
}

// Receive click on field.
function clickField(clickObj) {
  const idx = parseInt(clickObj.target.id.split('_')[1], 10)
  setFieldWithIdx(idx)
  drawBoardFields()
  gWorker.postMessage({ kind: 'track_move', lastMove: lastMove.to_js_value() })
}

// Set field with index into all generated fields.
function setFieldWithIdx(idx) {
  const coords = gBoard.get_coords(idx)
  setFieldWithCoords(coords)
}

// Set field with row/column coordinates.
function setFieldWithCoords(coords) {
  if (gameActive) {
    lastMove.coords = coords
    lastMove.side = lastMove.side == Cell.X ? Cell.O : Cell.X

    gBoard.set_cell(lastMove.coords.row, lastMove.coords.col, lastMove.side)
    console.log(
      `Set field row ${lastMove.coords.row} col ${lastMove.coords.col}`,
    )

    const winner = gBoard.line_winner(lastMove.coords, 3)
    checkWinner(winner)
  }
}

// Check if any side won after the last move and update the game if so.
function checkWinner(winner) {
  if (winner == Cell.X || winner == Cell.O) {
    gameActive = false
    const winnerName = winner == Cell.X ? 'X' : 'O'
    document.getElementById('notification').innerText = `${winnerName} wins!`
  }
}

// Draw the HTML fields according to the board.
function drawBoardFields() {
  for (let i = 0; i < gBoard.width() * gBoard.height(); i++) {
    const cell = gCells[i]
    const boardField = document.getElementById(`field_${i}`)
    if (cell == Cell.Empty) {
      boardField.className = 'board-field rounded-corners'
      boardField.innerHTML = ''
    } else {
      boardField.className = 'board-field-set rounded-corners'
      if (cell == Cell.X) {
        boardField.innerHTML = '<span>X</span>'
      } else {
        boardField.innerHTML = '<span>O</span>'
      }
    }
  }
}

// Setup the callback functions of the board.
function setupButtons(resetText) {
  const resetButton = document.getElementById('reset-button')
  resetButton.onclick = () => {
    gBoard.reset()
    gWorker.postMessage({ kind: 'reset' })
    lastMove.coords.row = 0
    lastMove.coords.col = 0
    lastMove.side = Cell.O
    drawBoardFields()
    document.getElementById('notification').innerText = resetText
    gameActive = true
  }

  const aiMoveButton = document.getElementById('ai-move-button')
  aiMoveButton.onclick = () => {
    if (gameActive) {
      gWorker.postMessage({ kind: 'get_best_move' })
    }
  }
}

// Setup the worker precalculating possible next moves and their avg. values.
function setupWorker(workerFile) {
  const worker = new Worker(workerFile)

  // Set on-message callback of worker
  worker.onmessage = async (event) => {
    if (event.data.kind == 'best_move') {
      if (gameActive) {
        const bestMove = BoardMove.from_js_value(event.data.bestMove)
        setFieldWithCoords(bestMove.coords)
        drawBoardFields()
        console.log('Got best move', bestMove, 'from worker')
      }
    }
  }

  // Make worker available as global variable
  gWorker = worker
}

// Run main WASM entry point loading WASM code and triggering the setup.
async function run_wasm(row, col, workerFile, resetText) {
  const rustWasm = await wasm_bindgen('./pkg/wasm_board_games_bg.wasm')
  console.log('WASM loaded')
  console.log(rustWasm.memory)

  // Create new board as WASM object
  var board = Board.new(row, col)

  // Create array to directly access board cells in WASM memory from JS
  const cells = new Uint8Array(
    rustWasm.memory.buffer,
    board.cells_ptr(),
    board.width() * board.height(),
  )

  // Make board and cells available as global variables
  gBoard = board
  gCells = cells

  setupBoard(board.height(), board.width())
  drawBoardFields()
  setupButtons(resetText)

  lastMove = BoardMove.new(0, 0, Cell.O)
  setupWorker(workerFile)
}
