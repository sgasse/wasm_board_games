use super::board::{Board, Cell, Coord};
use super::GameState;

const X_WIN_VALUE: i32 = 1000000;

pub struct T3Move {
    row: u32,
    col: u32,
    side: Cell,
}

pub struct T3GameState {
    board: Board,
    last_move: T3Move,
}

impl T3GameState {
    /// Determine the winner on the lines through `self.last_move`.
    ///
    /// This assumes that there is no winning pattern on any other line which
    /// does not go through `self.last_move`. This is a reasonable assumption
    /// if every game state is evaluated directly, thus a previously completed
    /// pattern on another line would have been detected before.
    fn line_winner(&self) -> Cell {
        // To determine the potential winner, we check the horizontal, vertial,
        // diagonal-down and diagonal-up lines through `self.last_move`.

        // Find the start point by substracting the minimum distance from
        // both the row and the column.
        // For point a (1, 2), the start of the diagonal down is s (0, 1)
        // | |s| | |
        // | | |a| |
        // | | | | |
        let diag_down_min_dist = u32::min(self.last_move.row, self.last_move.col);
        let diag_down_start = Coord {
            row: (self.last_move.row - diag_down_min_dist) as i32,
            col: (self.last_move.col - diag_down_min_dist) as i32,
        };

        // Find the start point by substracting the minimum distance from the
        // column and *adding* the minimum distance to the row. For the
        // row-part, we take the distance to the height into account since it is
        // the diagonal up.
        // For point a (1, 2), the start of the diagonal up is s (2, 1).
        // | | | | |
        // | | |a| |
        // | |s| | |
        let diag_up_min_dist = u32::min(
            self.board.height() - 1 - self.last_move.row,
            self.last_move.col,
        );
        let diag_up_start = Coord {
            row: (self.last_move.row + diag_up_min_dist) as i32,
            col: (self.last_move.col - diag_up_min_dist) as i32,
        };

        let pos_d_pos_pairs = vec![
            (
                // Horizontal
                Coord {
                    row: self.last_move.row as i32,
                    col: 0,
                },
                Coord { row: 0, col: 1 },
            ),
            (
                // Vertical
                Coord {
                    row: 0,
                    col: self.last_move.col as i32,
                },
                Coord { row: 1, col: 0 },
            ),
            (
                // Diagonal down
                diag_down_start,
                Coord { row: 1, col: 1 },
            ),
            (
                // Diagonal up
                diag_up_start,
                Coord { row: -1, col: 1 },
            ),
        ];

        for (pos, d_pos) in pos_d_pos_pairs {
            let line_winner = side_with_min_equal(&self.board, &pos, &d_pos, 3);
            match line_winner {
                Cell::Empty => continue,
                side => return side,
            };
        }

        Cell::Empty
    }
}

impl GameState for T3GameState {
    fn expand(&self) -> Vec<T3GameState> {
        let next_side = match self.last_move.side {
            Cell::X => Cell::O,
            Cell::O => Cell::X,
            Cell::Empty => panic!("Last move cannot be empty!"),
        };

        let next_states: Vec<T3GameState> = self
            .board
            .cells()
            .iter()
            .enumerate()
            .filter_map(|(idx, cell)| {
                if let Cell::Empty = cell {
                    let mut new_board = self.board.clone();
                    let (row, col) = new_board.get_coords(idx);
                    new_board.set_cell(row, col, next_side.clone());

                    return Some(T3GameState {
                        board: new_board,
                        last_move: T3Move {
                            row,
                            col,
                            side: next_side.clone(),
                        },
                    });
                }

                None
            })
            .collect();
        next_states
    }

    fn position_value(&self) -> i32 {
        match self.line_winner() {
            Cell::X => return X_WIN_VALUE,
            Cell::O => return -X_WIN_VALUE,
            Cell::Empty => return 0,
        }
    }
}

fn side_with_min_equal(board: &Board, pos: &Coord, d_pos: &Coord, num_winner: i32) -> Cell {
    let mut count = 0;
    let mut marker = Cell::Empty;

    let Coord {
        row: mut cur_row,
        col: mut cur_col,
    } = pos;

    while board.in_bounds(cur_row as u32, cur_col as u32) {
        let cur_marker = board.get_cell(cur_row as u32, cur_col as u32).unwrap();
        if cur_marker == marker {
            count += 1;
        } else {
            marker = cur_marker;
            count = 1;
        }

        if (count >= num_winner) && (marker != Cell::Empty) {
            return marker;
        }

        cur_row = cur_row + d_pos.row;
        cur_col = cur_col + d_pos.col;
    }

    Cell::Empty
}

#[cfg(test)]
mod test {

    use crate::refactor::GameState;

    use super::Cell;
    use super::T3Move;
    use super::{Board, T3GameState};

    #[test]
    fn test_t3gamestate_expand() {
        let mut b1 = Board::new(3, 3);
        // X X
        // O O
        // X O
        let _ = b1.set_state(vec![
            Cell::X,
            Cell::Empty,
            Cell::X,
            Cell::O,
            Cell::Empty,
            Cell::O,
            Cell::X,
            Cell::Empty,
            Cell::O,
        ]);

        let game_state = T3GameState {
            board: b1,
            last_move: T3Move {
                row: 2,
                col: 2,
                side: Cell::O,
            },
        };

        let expanded_states = game_state.expand();
        assert_eq!(expanded_states.len(), 3);
    }

    #[test]
    fn test_t3gamestate_line_winner() {
        let mut b1 = Board::new(3, 3);
        // XO
        // OXO
        //  XX
        let _ = b1.set_state(vec![
            Cell::X,
            Cell::O,
            Cell::Empty,
            Cell::O,
            Cell::X,
            Cell::O,
            Cell::Empty,
            Cell::X,
            Cell::X,
        ]);

        let game_state = T3GameState {
            board: b1,
            last_move: T3Move {
                row: 1,
                col: 1,
                side: Cell::X,
            },
        };

        assert_eq!(game_state.line_winner(), Cell::X);
    }
}
