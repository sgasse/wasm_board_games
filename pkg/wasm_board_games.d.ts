declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	*/
	export enum ExpandResult {
	  Done = 0,
	  NotDone = 1,
	}
	/**
	*/
	export enum Cell {
	  Empty = 0,
	  X = 1,
	  O = 2,
	}
	/**
	*/
	export class Board {
	  free(): void;
	/**
	* @param {number} height
	* @param {number} width
	* @returns {Board}
	*/
	  static new(height: number, width: number): Board;
	/**
	* @returns {number}
	*/
	  width(): number;
	/**
	* @returns {number}
	*/
	  height(): number;
	/**
	* @returns {number}
	*/
	  cells_ptr(): number;
	/**
	* @param {number} row
	* @param {number} col
	* @param {number} mark
	* @returns {boolean}
	*/
	  set_cell(row: number, col: number, mark: number): boolean;
	/**
	* @param {number} row
	* @param {number} col
	* @returns {number}
	*/
	  get_index(row: number, col: number): number;
	/**
	* @param {number} row
	* @param {number} col
	* @returns {boolean}
	*/
	  in_bounds(row: number, col: number): boolean;
	/**
	* @param {number} idx
	* @returns {Coords}
	*/
	  get_coords(idx: number): Coords;
	/**
	*/
	  reset(): void;
	/**
	* Determine the winner on the lines through `self.last_move`.
	*
	* This assumes that there is no winning pattern on any other line which
	* does not go through `self.last_move`. This is a reasonable assumption
	* if every game state is evaluated directly, thus a previously completed
	* pattern on another line would have been detected before.
	* @param {Coords} last_move
	* @param {number} num_winner
	* @returns {number}
	*/
	  line_winner(last_move: Coords, num_winner: number): number;
	/**
	* @param {number} col
	* @returns {Coords}
	*/
	  first_empty_in_column(col: number): Coords;
	}
	/**
	*/
	export class BoardMove {
	  free(): void;
	/**
	* @param {number} row
	* @param {number} col
	* @param {number} side
	* @returns {BoardMove}
	*/
	  static new(row: number, col: number, side: number): BoardMove;
	/**
	* @param {any} js_value
	* @returns {BoardMove}
	*/
	  static from_js_value(js_value: any): BoardMove;
	/**
	* @returns {any}
	*/
	  to_js_value(): any;
	/**
	*/
	  coords: Coords;
	/**
	*/
	  side: number;
	}
	/**
	*/
	export class Coords {
	  free(): void;
	/**
	* @param {number} row
	* @param {number} col
	* @returns {Coords}
	*/
	  static new(row: number, col: number): Coords;
	/**
	*/
	  col: number;
	/**
	*/
	  row: number;
	}
	/**
	*/
	export class FiarGameInterface {
	  free(): void;
	/**
	* @returns {FiarGameInterface}
	*/
	  static new(): FiarGameInterface;
	/**
	* @returns {number}
	*/
	  expand_one_level(): number;
	/**
	* @param {BoardMove} game_move
	* @returns {boolean}
	*/
	  track_move(game_move: BoardMove): boolean;
	/**
	* @returns {BoardMove}
	*/
	  get_best_move(): BoardMove;
	/**
	*/
	  reset(): void;
	}
	/**
	*/
	export class FiarGameState {
	  free(): void;
	/**
	* @param {Board} board
	* @param {BoardMove} last_move
	* @returns {FiarGameState}
	*/
	  static new(board: Board, last_move: BoardMove): FiarGameState;
	/**
	* @returns {number}
	*/
	  side(): number;
	/**
	* @returns {BoardMove}
	*/
	  last_move(): BoardMove;
	}
	/**
	*/
	export class T3GameInterface {
	  free(): void;
	/**
	* @returns {T3GameInterface}
	*/
	  static new(): T3GameInterface;
	/**
	* @returns {number}
	*/
	  expand_one_level(): number;
	/**
	* @param {BoardMove} game_move
	* @returns {boolean}
	*/
	  track_move(game_move: BoardMove): boolean;
	/**
	* @returns {BoardMove}
	*/
	  get_best_move(): BoardMove;
	/**
	*/
	  reset(): void;
	}
	/**
	*/
	export class T3GameState {
	  free(): void;
	/**
	* @param {Board} board
	* @param {BoardMove} last_move
	* @returns {T3GameState}
	*/
	  static new(board: Board, last_move: BoardMove): T3GameState;
	/**
	* @returns {number}
	*/
	  side(): number;
	/**
	* @returns {BoardMove}
	*/
	  last_move(): BoardMove;
	}
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly t3gameinterface_new: () => number;
  readonly t3gameinterface_expand_one_level: (a: number) => number;
  readonly t3gameinterface_track_move: (a: number, b: number) => number;
  readonly t3gameinterface_get_best_move: (a: number) => number;
  readonly t3gameinterface_reset: (a: number) => void;
  readonly __wbg_fiargameinterface_free: (a: number) => void;
  readonly fiargameinterface_new: () => number;
  readonly fiargameinterface_expand_one_level: (a: number) => number;
  readonly fiargameinterface_track_move: (a: number, b: number) => number;
  readonly fiargameinterface_get_best_move: (a: number) => number;
  readonly fiargameinterface_reset: (a: number) => void;
  readonly __wbg_t3gameinterface_free: (a: number) => void;
  readonly __wbg_board_free: (a: number) => void;
  readonly board_new: (a: number, b: number) => number;
  readonly board_width: (a: number) => number;
  readonly board_height: (a: number) => number;
  readonly board_cells_ptr: (a: number) => number;
  readonly board_set_cell: (a: number, b: number, c: number, d: number) => number;
  readonly board_get_index: (a: number, b: number, c: number) => number;
  readonly board_in_bounds: (a: number, b: number, c: number) => number;
  readonly board_get_coords: (a: number, b: number) => number;
  readonly board_reset: (a: number) => void;
  readonly board_line_winner: (a: number, b: number, c: number) => number;
  readonly board_first_empty_in_column: (a: number, b: number) => number;
  readonly __wbg_coords_free: (a: number) => void;
  readonly __wbg_get_coords_row: (a: number) => number;
  readonly __wbg_set_coords_row: (a: number, b: number) => void;
  readonly __wbg_get_coords_col: (a: number) => number;
  readonly __wbg_set_coords_col: (a: number, b: number) => void;
  readonly __wbg_boardmove_free: (a: number) => void;
  readonly __wbg_get_boardmove_coords: (a: number) => number;
  readonly __wbg_set_boardmove_coords: (a: number, b: number) => void;
  readonly __wbg_get_boardmove_side: (a: number) => number;
  readonly __wbg_set_boardmove_side: (a: number, b: number) => void;
  readonly boardmove_new: (a: number, b: number, c: number) => number;
  readonly boardmove_from_js_value: (a: number) => number;
  readonly boardmove_to_js_value: (a: number) => number;
  readonly coords_new: (a: number, b: number) => number;
  readonly __wbg_fiargamestate_free: (a: number) => void;
  readonly fiargamestate_new: (a: number, b: number) => number;
  readonly fiargamestate_side: (a: number) => number;
  readonly fiargamestate_last_move: (a: number) => number;
  readonly __wbg_t3gamestate_free: (a: number) => void;
  readonly t3gamestate_new: (a: number, b: number) => number;
  readonly t3gamestate_side: (a: number) => number;
  readonly t3gamestate_last_move: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
