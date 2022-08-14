// Make WASM structs available
const { Board, Cell } = wasm_bindgen

const BOARD_GAP_SIZE = 4
const BOARD_PADDING = 4
const CELL_SIZE = 80

// There global variables will be set after loading WASM
var gBoard = null
var gCells = null
var nextTurn = Cell.X
var gWorker = null

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
  setField(idx)
  drawBoardFields()
}

function setField(idx) {
  const coords = gBoard.get_coords(idx)
  gBoard.set_cell(coords.row, coords.col, nextTurn)
  nextTurn = nextTurn == Cell.X ? Cell.O : Cell.X
  console.log(`Set field row ${coords.row} col ${coords.col}`)
  gWorker.postMessage(coords)
}

function drawBoardFields() {
  // console.log(gCells)
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
    nextTurn = Cell.X
    drawBoardFields()
  }
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

  const worker = new Worker('./ttt_worker.js')
  gWorker = worker
}

run_wasm()
