use std::collections::VecDeque;

use super::{GameState, X_WIN_VALUE};

pub struct TreeEvaluator<T> {
    parent: Vec<usize>,
    children: Vec<Vec<usize>>,
    game_states: Vec<T>,
    avg_values: Vec<i32>,
}

impl<'a, T: GameState> TreeEvaluator<T> {
    pub fn new(init_state: T) -> TreeEvaluator<T> {
        TreeEvaluator {
            parent: vec![0],
            children: vec![vec![]],
            game_states: vec![init_state],
            avg_values: vec![0],
        }
    }

    fn expand_state(&mut self, idx: usize) -> Option<Vec<usize>> {
        let g_state = self.game_states.get(idx).expect("Game state");
        let pos_value = *self.avg_values.get(idx).expect("Position value");
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

        // Positional values of child states
        let mut avg_values: Vec<i32> = child_states
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
        self.avg_values.append(&mut avg_values);

        // Return child indexes for expansion
        Some(child_idx)
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

    fn bfs_iter(&'a self, start_idx: usize) -> BfsIterator<'a, T> {
        BfsIterator {
            tree_eval: &self,
            buffer: VecDeque::from([start_idx]),
        }
    }

    fn evaluate_states(&mut self, stop_idx: usize) {
        let reverse_bfs_order: Vec<usize> = self
            .bfs_iter(stop_idx)
            .collect::<Vec<usize>>()
            .into_iter()
            .rev()
            .collect();

        // By traversing the graph in reverse BFS-order, we can be sure that
        // children are evaluated before their parents.
        for idx in reverse_bfs_order {
            let init_value = *self.avg_values.get(idx).expect("Avg value");
            if init_value == X_WIN_VALUE || init_value == -X_WIN_VALUE {
                // Skip evaluating the average of children for final states
                continue;
            }

            // Get children
            let child_vals: Vec<i32> = self
                .children
                .get(idx)
                .expect("Children")
                .iter()
                .map(|&child_idx| *self.avg_values.get(child_idx).expect("Avg value"))
                .collect();

            let avg_value = match child_vals.is_empty() {
                true => init_value,
                false => child_vals.iter().sum::<i32>() / child_vals.len() as i32,
            };

            self.avg_values[idx] = avg_value;
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
    use crate::refactor::X_WIN_VALUE;

    use super::{
        super::{
            board::{Board, Cell},
            t3_game::{T3GameState, T3Move},
        },
        TreeEvaluator,
    };

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
            T3Move {
                row: 2,
                col: 2,
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
        assert_eq!(tree_eval.avg_values.len(), 4);

        assert_eq!(tree_eval.children.get(0).unwrap(), &vec![1, 2, 3]);
    }

    #[test]
    fn test_tree_evaluator_expand_by_two() {
        let mut tree_eval = TreeEvaluator::new(get_ref_state());
        tree_eval.expand_states_by(0, 2);

        assert_eq!(tree_eval.parent.len(), 6);
        assert_eq!(tree_eval.children.len(), 6);
        assert_eq!(tree_eval.game_states.len(), 6);
        assert_eq!(tree_eval.avg_values.len(), 6);
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
        tree_eval.expand_states_by(0, 2);

        tree_eval.evaluate_states(0);
        assert_eq!(tree_eval.avg_values[0], X_WIN_VALUE / 2);
    }
}
