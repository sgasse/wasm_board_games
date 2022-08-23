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

function clickField(clickObj) {
  const idx = parseInt(clickObj.target.id.split('_')[1], 10)
  setFieldWithIdx(idx)
  drawBoardFields()
  gWorker.postMessage({ kind: 'track_move', lastMove: lastMove.to_js_value() })
}

function setFieldWithIdx(idx) {
  const coords = gBoard.get_coords(idx)
  setFieldWithCoords(coords)
}

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

function checkWinner(winner) {
  if (winner == Cell.X || winner == Cell.O) {
    gameActive = false
    const winnerName = winner == Cell.X ? 'X' : 'O'
    document.getElementById('notification').innerText = `${winnerName} wins!`
  }
}

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

function setupButtons() {
  const resetButton = document.getElementById('reset-button')
  resetButton.onclick = () => {
    gBoard.reset()
    gWorker.postMessage({ kind: 'reset' })
    lastMove.coords.row = 0
    lastMove.coords.col = 0
    lastMove.side = Cell.O
    drawBoardFields()
    document.getElementById('notification').innerText = 'Tic Tac To'
    gameActive = true
  }

  const aiMoveButton = document.getElementById('ai-move-button')
  aiMoveButton.onclick = () => {
    if (gameActive) {
      gWorker.postMessage({ kind: 'get_best_move' })
    }
  }
}

function setupWorker() {
  const worker = new Worker('./ttt_worker.js')
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
  gWorker = worker
}

async function run_wasm() {
  const rustWasm = await wasm_bindgen('./pkg/wasm_board_games_bg.wasm')
  console.log('WASM loaded')
  console.log(rustWasm.memory)

  var board = Board.new(3, 3)
  const cells = new Uint8Array(
    rustWasm.memory.buffer,
    board.cells_ptr(),
    board.width() * board.height(),
  )

  gBoard = board
  gCells = cells

  setupBoard(3, 3)
  drawBoardFields()
  setupButtons()

  lastMove = BoardMove.new(0, 0, Cell.O)
  setupWorker()
}

run_wasm()
