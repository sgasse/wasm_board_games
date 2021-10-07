use super::board::{Board, Cell};
use super::common::Coord;

const X_WIN_VALUE: f32 = 100.0;
const EPS: f32 = 1e-7;

pub trait GameState
where
    Self: Sized,
{
    fn new(board: Board, next_turn: Cell) -> Self;

    fn evaluate(&mut self) -> Result<(), &'static str>;

    fn expand(&self) -> Result<Vec<Self>, &'static str>;

    fn is_final(&self) -> bool;

    fn avg_value(&self) -> f32;

    fn set_avg_value(&mut self, avg_value: f32);

    fn position_value(&self) -> f32;

    fn last_move(&self) -> Coord;

    fn next_turn(&self) -> Cell;
}

/// State of a TicTacTo game
#[derive(Clone, Debug, PartialEq)]
pub struct TTTGameState {
    board: Board,
    next_turn: Cell,
    avg_value: f32,
    position_value: f32,
    is_final: bool,
    last_move: Coord,
}

impl GameState for TTTGameState {
    /// Return a new game state with a given board position and next player.
    ///
    /// # Arguments
    ///
    /// * `board` - A Board struct holding the playing field.
    /// * `next_turn` - A Cell enum indicating which player's turn is the next.
    fn new(board: Board, next_turn: Cell) -> TTTGameState {
        TTTGameState {
            board,
            next_turn,
            avg_value: 0.0,
            position_value: 0.0,
            is_final: false,
            last_move: Coord { row: 0, col: 0 },
        }
    }

    fn evaluate(&mut self) -> Result<(), &'static str> {
        match self.board.line_count_winner(3)? {
            Cell::Empty => {
                self.position_value = 0.0;
            }
            Cell::X => {
                self.position_value = X_WIN_VALUE;
                self.is_final = true;
            }
            Cell::O => {
                self.position_value = -X_WIN_VALUE;
                self.is_final = true;
            }
        }

        Ok(())
    }

    fn expand(&self) -> Result<Vec<TTTGameState>, &'static str> {
        let mut child_states = Vec::new();

        // Do not expand if final state
        if self.position_value.abs() < EPS {
            // Iterate over all fields and find possible moves
            for row in 0..self.board.height() {
                for col in 0..self.board.width() {
                    let cell = self.board.get_cell(row, col)?;

                    // Create a child state per possible move
                    if cell == Cell::Empty {
                        let mut new_state = self.clone();
                        new_state.board.set_cell(row, col, self.next_turn.clone())?;
                        new_state.next_turn = match self.next_turn {
                            Cell::X => Cell::O,
                            Cell::O => Cell::X,
                            _ => Cell::X,
                        };
                        new_state.last_move = Coord { row, col };

                        // Evaluate new state
                        new_state.evaluate()?;

                        child_states.push(new_state);
                    }
                }
            }
        }

        Ok(child_states)
    }

    fn is_final(&self) -> bool {
        self.is_final
    }

    fn avg_value(&self) -> f32 {
        self.avg_value
    }

    fn set_avg_value(&mut self, avg_value: f32) {
        self.avg_value = avg_value;
    }

    fn position_value(&self) -> f32 {
        self.position_value
    }

    fn last_move(&self) -> Coord {
        self.last_move.clone()
    }

    fn next_turn(&self) -> Cell {
        self.next_turn.clone()
    }
}

/// State of a Four-in-a-row game
#[derive(Clone, Debug, PartialEq)]
pub struct FiarGameState {
    board: Board,
    next_turn: Cell,
    avg_value: f32,
    position_value: f32,
    is_final: bool,
    last_move: Coord,
}

impl GameState for FiarGameState {
    /// Return a new game state with a given board position and next player.
    ///
    /// # Arguments
    ///
    /// * `board` - A Board struct holding the playing field.
    /// * `next_turn` - A Cell enum indicating which player's turn is the next.
    fn new(board: Board, next_turn: Cell) -> FiarGameState {
        FiarGameState {
            board,
            next_turn,
            avg_value: 0.0,
            position_value: 0.0,
            is_final: false,
            last_move: Coord { row: 0, col: 0 },
        }
    }

    fn evaluate(&mut self) -> Result<(), &'static str> {
        match self.board.line_count_winner(4)? {
            Cell::Empty => {
                self.position_value = 0.0;
            }
            Cell::X => {
                self.position_value = X_WIN_VALUE;
                self.is_final = true;
            }
            Cell::O => {
                self.position_value = -X_WIN_VALUE;
                self.is_final = true;
            }
        }

        Ok(())
    }

    fn expand(&self) -> Result<Vec<FiarGameState>, &'static str> {
        let mut child_states = Vec::new();

        // Do not expand if final state
        if self.position_value.abs() < EPS {
            // Iterate over all columns but only to the first free row per column
            'col_loop: for col in 0..self.board.width() {
                for row in (0..self.board.height()).rev() {
                    let cell = self.board.get_cell(row, col)?;

                    if cell == Cell::Empty {
                        let mut new_state = self.clone();
                        new_state.board.set_cell(row, col, self.next_turn.clone())?;
                        new_state.next_turn = match self.next_turn {
                            Cell::X => Cell::O,
                            Cell::O => Cell::X,
                            _ => Cell::X,
                        };
                        new_state.last_move = Coord { row, col };

                        // Evaluate new state
                        new_state.evaluate()?;

                        child_states.push(new_state);

                        // Skip empty fields above the first empty row per column
                        continue 'col_loop;
                    }
                }
            }
        }

        Ok(child_states)
    }

    fn is_final(&self) -> bool {
        self.is_final
    }

    fn avg_value(&self) -> f32 {
        self.avg_value
    }

    fn set_avg_value(&mut self, avg_value: f32) {
        self.avg_value = avg_value;
    }

    fn position_value(&self) -> f32 {
        self.position_value
    }

    fn last_move(&self) -> Coord {
        self.last_move.clone()
    }

    fn next_turn(&self) -> Cell {
        self.next_turn.clone()
    }
}

#[cfg(test)]
mod test {

    use super::{Board, Cell, FiarGameState, GameState, TTTGameState};

    #[test]
    fn test_expand_ttt_state() -> Result<(), &'static str> {
        let root_state = TTTGameState::new(Board::new(3, 3), Cell::X);
        let child_states = root_state.expand()?;

        assert_eq!(child_states.len(), 9);

        Ok(())
    }

    #[test]
    fn test_expand_fiar_state() -> Result<(), &'static str> {
        let root_state = FiarGameState::new(Board::new(7, 6), Cell::X);
        let child_states = root_state.expand()?;

        assert_eq!(child_states.len(), 7);

        Ok(())
    }
}
