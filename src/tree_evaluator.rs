use super::{Cell, GameState, X_WIN_VALUE};
use std::collections::VecDeque;
use web_sys::console;

pub struct TreeEvaluator<T> {
    parent: Vec<usize>,
    children: Vec<Vec<usize>>,
    game_states: Vec<T>,
    worst_case_values: Vec<i32>,
}

impl<'a, T> TreeEvaluator<T>
where
    T: GameState + Default,
{
    pub fn new(init_state: T) -> TreeEvaluator<T> {
        TreeEvaluator {
            parent: vec![0],
            children: vec![vec![]],
            game_states: vec![init_state],
            worst_case_values: vec![0],
        }
    }

    pub fn new_with_default() -> TreeEvaluator<T> {
        let init_state = T::default();
        Self::new(init_state)
    }

    pub fn expand_and_get_children_idx(&mut self, idx_to_expand: &Vec<usize>) -> Vec<usize> {
        let mut expanded_children: Vec<usize> = Vec::new();

        for &idx in idx_to_expand.iter() {
            if let Some(children_idx) = self.expand_state(idx) {
                expanded_children.append(&mut Vec::from(children_idx));
            }
        }

        console::log_1(&format!("Generated {} new nodes", expanded_children.len()).into());

        expanded_children
    }

    fn expand_state(&mut self, idx: usize) -> Option<Vec<usize>> {
        let g_state = self.game_states.get(idx).expect("Game state");
        let pos_value = *self.worst_case_values.get(idx).expect("Position value");
        if pos_value == X_WIN_VALUE || pos_value == -X_WIN_VALUE {
            // State is final - skip expansion
            return None;
        }

        // Expand game state to possible child states
        let mut child_states = g_state.expand();

        // Indexes of child states in the tree data structure
        let child_idx: Vec<usize> = (self.parent.len()..self.parent.len() + child_states.len())
            .into_iter()
            .collect();

        // Initialize worst case values to positional values to catch final states
        let mut worst_case_values: Vec<i32> = child_states
            .iter()
            .map(|state| state.position_value())
            .collect();

        // Set children of parent
        self.children
            .get_mut(idx)
            .expect("Parent")
            .append(&mut child_idx.clone());

        // Add parent for children
        self.parent.append(&mut vec![idx; child_states.len()]);
        for _ in 0..child_states.len() {
            // Add empty children vectors for children
            self.children.push(vec![]);
        }

        // Add child states and positional values
        self.game_states.append(&mut child_states);
        self.worst_case_values.append(&mut worst_case_values);

        // Return child indexes for expansion
        Some(child_idx)
    }

    pub fn evaluate_states(&mut self, stop_idx: usize) {
        let reverse_bfs_order: Vec<usize> = self
            .bfs_iter(stop_idx)
            .collect::<Vec<usize>>()
            .into_iter()
            .rev()
            .collect();

        // By traversing the graph in reverse BFS-order, we can be sure that
        // children are evaluated before their parents.
        for idx in reverse_bfs_order {
            let side = self.game_states().get(idx).expect("Game state").side();
            let init_value = *self.worst_case_values.get(idx).expect("Avg value");
            if init_value == X_WIN_VALUE || init_value == -X_WIN_VALUE {
                // Skip evaluating the worst case of children for final states
                continue;
            }

            // Get children
            let child_vals: Vec<i32> = self
                .children
                .get(idx)
                .expect("Children")
                .iter()
                .map(|&child_idx| *self.worst_case_values.get(child_idx).expect("Avg value"))
                .collect();

            // Fix here TODO
            let avg_value = match (child_vals.is_empty(), side) {
                (true, _) => init_value,
                (false, Cell::X) => *child_vals
                    .iter()
                    .min()
                    .expect("safe due to .is_empty() check"),
                (false, Cell::O) => *child_vals
                    .iter()
                    .max()
                    .expect("save due to .is_empty() check"),
                (false, Cell::Empty) => {
                    console::log_1(&"Unexpected empty state in previous move".to_owned().into());
                    init_value
                }
            };

            self.worst_case_values[idx] = avg_value;
        }
    }

    pub fn game_states(&self) -> &Vec<T> {
        &self.game_states
    }

    pub fn worst_case_values(&self) -> &Vec<i32> {
        &self.worst_case_values
    }

    pub fn children(&self) -> &Vec<Vec<usize>> {
        &self.children
    }

    pub fn expand_states_by(&mut self, start_idx: usize, num_levels: u32) {
        let mut expand_now: VecDeque<usize> = VecDeque::from([start_idx]);

        for _ in 0..num_levels {
            let mut expand_next: VecDeque<usize> = VecDeque::new();

            for &idx in expand_now.iter() {
                if let Some(child_idx) = self.expand_state(idx) {
                    expand_next.append(&mut VecDeque::from(child_idx));
                }
            }

            expand_now = expand_next;
        }
    }

    pub fn bfs_iter(&'a self, start_idx: usize) -> BfsIterator<'a, T> {
        BfsIterator {
            tree_eval: &self,
            buffer: VecDeque::from([start_idx]),
        }
    }
}

pub struct BfsIterator<'a, T> {
    tree_eval: &'a TreeEvaluator<T>,
    buffer: VecDeque<usize>,
}

impl<'a, T> Iterator for BfsIterator<'a, T> {
    type Item = usize;

    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        match self.buffer.pop_front() {
            Some(idx) => {
                let children = self.tree_eval.children.get(idx).expect("Children");
                self.buffer.append(&mut VecDeque::from(children.clone()));
                return Some(idx);
            }
            None => return None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Board, BoardMove, Cell, Coords, T3GameState, TreeEvaluator, X_WIN_VALUE};

    fn get_ref_state() -> T3GameState {
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

        T3GameState::new(
            b1,
            BoardMove {
                coords: Coords { row: 2, col: 2 },
                side: Cell::O,
            },
        )
    }

    #[test]
    fn test_tree_evaluator_expand_by_one() {
        let mut tree_eval = TreeEvaluator::new(get_ref_state());
        tree_eval.expand_states_by(0, 1);

        assert_eq!(tree_eval.parent.len(), 4);
        assert_eq!(tree_eval.children.len(), 4);
        assert_eq!(tree_eval.game_states.len(), 4);
        assert_eq!(tree_eval.worst_case_values.len(), 4);

        assert_eq!(tree_eval.children.get(0).unwrap(), &vec![1, 2, 3]);
    }

    #[test]
    fn test_tree_evaluator_expand_by_two() {
        let mut tree_eval = TreeEvaluator::new(get_ref_state());
        tree_eval.expand_states_by(0, 2);

        assert_eq!(tree_eval.parent.len(), 6);
        assert_eq!(tree_eval.children.len(), 6);
        assert_eq!(tree_eval.game_states.len(), 6);
        assert_eq!(tree_eval.worst_case_values.len(), 6);
    }

    #[test]
    fn test_bfs_iter() {
        let mut tree_eval = TreeEvaluator::new(get_ref_state());
        tree_eval.expand_states_by(0, 2);

        let bfs_order: Vec<usize> = tree_eval.bfs_iter(0).collect();
        assert_eq!(bfs_order, (0..6).into_iter().collect::<Vec<usize>>());
    }

    #[test]
    fn test_evaluate_states() {
        let mut tree_eval = TreeEvaluator::new(get_ref_state());
        tree_eval.expand_states_by(0, 9);

        tree_eval.evaluate_states(0);
        // The reference state is one before winning, we expect a worst case
        // value of the full win value.
        assert_eq!(tree_eval.worst_case_values[0], X_WIN_VALUE);
    }

    // This is currently only used for debugging purposes, no real test
    fn test_t3_corner_state() {
        let mut b1 = Board::new(3, 3);
        // X
        //  O
        //   X
        // -> O should set on one of the edges, *not* the corners
        let _ = b1.set_state(vec![
            Cell::X,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::O,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::X,
        ]);

        let start_state = T3GameState::new(
            b1,
            BoardMove {
                coords: Coords { row: 0, col: 0 },
                side: Cell::X,
            },
        );
        let mut tree_eval = TreeEvaluator::new(start_state);

        tree_eval.expand_states_by(0, 9);
        tree_eval.evaluate_states(0);
        println!("Expansion done");
    }
}
