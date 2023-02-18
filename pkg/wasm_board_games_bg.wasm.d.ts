/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function t3gameinterface_new(): number;
export function t3gameinterface_expand_one_level(a: number): number;
export function t3gameinterface_track_move(a: number, b: number): number;
export function t3gameinterface_get_best_move(a: number): number;
export function t3gameinterface_reset(a: number): void;
export function __wbg_fiargameinterface_free(a: number): void;
export function fiargameinterface_new(): number;
export function fiargameinterface_expand_one_level(a: number): number;
export function fiargameinterface_track_move(a: number, b: number): number;
export function fiargameinterface_get_best_move(a: number): number;
export function fiargameinterface_reset(a: number): void;
export function __wbg_t3gameinterface_free(a: number): void;
export function __wbg_board_free(a: number): void;
export function board_new(a: number, b: number): number;
export function board_width(a: number): number;
export function board_height(a: number): number;
export function board_cells_ptr(a: number): number;
export function board_set_cell(a: number, b: number, c: number, d: number): number;
export function board_get_index(a: number, b: number, c: number): number;
export function board_in_bounds(a: number, b: number, c: number): number;
export function board_get_coords(a: number, b: number): number;
export function board_reset(a: number): void;
export function board_line_winner(a: number, b: number, c: number): number;
export function board_first_empty_in_column(a: number, b: number): number;
export function __wbg_coords_free(a: number): void;
export function __wbg_get_coords_row(a: number): number;
export function __wbg_set_coords_row(a: number, b: number): void;
export function __wbg_get_coords_col(a: number): number;
export function __wbg_set_coords_col(a: number, b: number): void;
export function __wbg_boardmove_free(a: number): void;
export function __wbg_get_boardmove_coords(a: number): number;
export function __wbg_set_boardmove_coords(a: number, b: number): void;
export function __wbg_get_boardmove_side(a: number): number;
export function __wbg_set_boardmove_side(a: number, b: number): void;
export function boardmove_new(a: number, b: number, c: number): number;
export function boardmove_from_js_value(a: number): number;
export function boardmove_to_js_value(a: number): number;
export function coords_new(a: number, b: number): number;
export function __wbg_fiargamestate_free(a: number): void;
export function fiargamestate_new(a: number, b: number): number;
export function fiargamestate_side(a: number): number;
export function fiargamestate_last_move(a: number): number;
export function __wbg_t3gamestate_free(a: number): void;
export function t3gamestate_new(a: number, b: number): number;
export function t3gamestate_side(a: number): number;
export function t3gamestate_last_move(a: number): number;
export function __wbindgen_malloc(a: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number): number;