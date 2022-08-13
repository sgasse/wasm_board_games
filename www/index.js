console.log('Hello from script')

const { Board, Cell } = wasm_bindgen

function setupBoard(rows, cols) {
  const numFields = rows * cols
  createFields(numFields)

  const boardDiv = document.getElementById('board')
  boardDiv.style.width = `${cols * 50 + (cols - 1) * 1}px`
  boardDiv.style.width = `${rows * 50 + (rows - 1) * 1}px`
  boardDiv.style.backgroundColor = 'green'
}

function createFields(numFields) {
  for (var i = 0; i < numFields; i++) {
    var fieldDiv = document.createElement('div')
    fieldDiv.id = `field_${i}`
    fieldDiv.className = 'boardField'
    fieldDiv.onclick = clickField
    document.getElementById('board').appendChild(fieldDiv)
    console.log('Added element', i)
  }
}

function clickField(clickObj) {
  console.log('Clicked', clickObj.target)
  const idx = parseInt(clickObj.target.id.split('_')[1], 10)
  console.log(idx)
  getRowColFromIdx(idx)
}

function getRowColFromIdx(idx) {
  const col = idx % 3
  const row = Math.floor(idx / 3)
  console.log('Row', row, 'col', col)
}

var gBoard = null

async function run_wasm() {
  const rustWasm = await wasm_bindgen('./pkg/wasm_board_games_bg.wasm')
  console.log('WASM loaded')

  console.log(rustWasm.memory)

  var board = Board.new(3, 3)
  board.set_cell(1, 2, Cell.O)
  gBoard = board
  const cells = new Uint8Array(
    rustWasm.memory.buffer,
    board.cells(),
    board.width() * board.height(),
  )
  console.log(cells)
  setupBoard(3, 3)
}

run_wasm()
