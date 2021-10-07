use std::collections::{HashSet, VecDeque};

use super::board::{Board, Cell};
use super::common::Coord;
use super::game_state::GameState;
use super::tree::{
    new_tree_depth_iter, AddressHashNodeW, Node, NodeDepth, NodeW, TreeDepthIterator,
};
use std::rc::Rc;

pub struct GameEvaluator<T>
where
    T: GameState,
{
    root_node: NodeW<T>,
    num_expanded_nodes: i32,
    to_expand: TreeDepthIterator<T>,
    completed_depth: i32,
    inserting_at_depth: i32,
    max_depth: i32,
    width: i32,
    height: i32,
}

impl<T> GameEvaluator<T>
where
    T: GameState,
{
    pub fn new(width: i32, height: i32, max_depth: i32) -> GameEvaluator<T> {
        let init_node = Node::new_wrapped(T::new(Board::new(width, height), Cell::X));
        GameEvaluator {
            root_node: Rc::clone(&init_node),
            num_expanded_nodes: 0,
            to_expand: new_tree_depth_iter(init_node, 0),
            completed_depth: -1,
            inserting_at_depth: 0,
            max_depth,
            width,
            height,
        }
    }

    pub fn reset(&mut self) {
        self.root_node = Node::new_wrapped(T::new(Board::new(self.width, self.height), Cell::X));
        // The other fields are reset in `init_expansion`

        self.init_expansion(self.max_depth);
    }

    pub fn init_expansion(&mut self, max_depth: i32) {
        self.max_depth = max_depth;
        self.completed_depth = -1;
        self.inserting_at_depth = 0;
        self.num_expanded_nodes = 0;

        // To limit evaluation to `max_depth`, it is enough to expand nodes on the rank of
        // `max_depth - 1`.
        let max_expansion_depth = self.max_depth - 1;

        let unexpanded_non_final_nodes: VecDeque<NodeDepth<T>> =
            new_tree_depth_iter(Rc::clone(&self.root_node), max_expansion_depth)
                .filter(|node_info| {
                    let node = (*node_info.node).borrow();
                    if node.data.is_final() {
                        // Skip nodes which are final positions
                        return false;
                    }

                    if !node.children.is_empty() {
                        // Skip nodes which have children and are thus already expanded
                        return false;
                    }

                    true
                })
                .collect();

        self.to_expand =
            TreeDepthIterator::from_vecdeque(unexpanded_non_final_nodes, max_expansion_depth);
    }

    pub fn expand_by(&mut self, num_elements: u32) -> Result<bool, &'static str> {
        // Expand a maximum of `num_elements` nodes in this call
        for _ in 0..num_elements {
            match self.to_expand.next() {
                None => {
                    if self.inserting_at_depth > self.completed_depth {
                        self.completed_depth = self.inserting_at_depth;
                    }
                    return Ok(true);
                }
                Some(node_info) => {
                    // Track depth information
                    self.inserting_at_depth = node_info.depth + 1;
                    if self.inserting_at_depth > self.completed_depth + 1 {
                        // New depth level reached
                        self.completed_depth = self.inserting_at_depth - 1;
                    }

                    // Get child states
                    let child_states = (*node_info.node).borrow().data.expand()?;

                    // TODO Set as final if no children

                    for child_state in child_states {
                        self.num_expanded_nodes += 1;
                        let child = Node::new_wrapped(child_state);
                        let parent = node_info.node.clone();
                        Node::add_child_to_parent(Rc::clone(&child), parent);

                        // Add children as potentially expandable nodes
                        self.to_expand.append(NodeDepth {
                            node: child,
                            depth: node_info.depth + 1,
                        });
                    }
                }
            }
        }

        Ok(false)
    }

    pub fn evaluate_tree(
        &mut self,
        start_node: NodeW<T>,
        max_depth: i32,
    ) -> Result<(), &'static str> {
        let mut computed: HashSet<AddressHashNodeW<T>> = HashSet::new();

        let mut all_nodes: VecDeque<NodeDepth<T>> =
            new_tree_depth_iter(start_node, max_depth).collect();
        for node_info in all_nodes.drain(..).rev() {
            let mut child_vals: Vec<f32> = Vec::new();
            let children = Node::children(Rc::clone(&node_info.node));

            for child in children {
                if computed.contains(&AddressHashNodeW {
                    inner: Rc::clone(&child),
                }) {
                    let child_val = (*child).borrow().data.avg_value();
                    child_vals.push(child_val);
                }
            }

            {
                let avg_value = match child_vals.len() {
                    0 => (*node_info.node).borrow().data.position_value(),
                    _ => child_vals.iter().sum::<f32>() / child_vals.len() as f32,
                };

                (*node_info.node).borrow_mut().data.set_avg_value(avg_value);
                computed.insert(AddressHashNodeW {
                    inner: Rc::clone(&node_info.node),
                });
            }
        }

        Ok(())
    }

    pub fn track_move(&mut self, last_move: Coord) -> Result<(), &'static str> {
        // Iterate over children of current node (root)
        let children = Node::children(Rc::clone(&self.root_node));

        for child in children {
            let last_child_move = (*child).borrow().data.last_move();
            if last_move == last_child_move {
                self.root_node = Rc::clone(&child);
                return Ok(());
            }
        }

        Err("Could not identify last move")
    }

    pub fn get_root_node(&self) -> NodeW<T> {
        Rc::clone(&self.root_node)
    }

    pub fn set_root_node(&mut self, new_root: NodeW<T>) {
        self.root_node = new_root;
    }
}

pub fn get_best_child_state<T>(parent: NodeW<T>) -> Result<NodeW<T>, &'static str>
where
    T: GameState,
{
    // Iterate over children and find child with highest/lowest value
    let children = Node::children(Rc::clone(&parent));
    if children.is_empty() {
        return Err("`parent` has no children");
    }

    // We want to identify the best move by folding the iterator of children with a optimum function
    // which differs depending on which player's turn it is.
    let player = (*parent).borrow().data.next_turn();
    let left_better = get_left_better_for_player(player)?;

    let best_child_node =
        &children
            .iter()
            .fold(Rc::clone(&children[0]), |best_child, cur_child| {
                let best_value = (*best_child).borrow().data.avg_value();
                let child_value = (*cur_child).borrow().data.avg_value();

                if left_better(best_value, child_value) {
                    best_child
                } else {
                    Rc::clone(cur_child)
                }
            });

    Ok(Rc::clone(best_child_node))
}

pub fn get_left_better_for_player(
    player: Cell,
) -> Result<Box<dyn Fn(f32, f32) -> bool>, &'static str> {
    let is_optimal = match player {
        Cell::X => |val_opti: f32, val: f32| val_opti > val,
        Cell::O => |val_opti: f32, val: f32| val_opti < val,
        Cell::Empty => return Err("Cannot determine optimum function for empty cell"),
    };

    Ok(Box::new(is_optimal))
}

#[cfg(test)]
mod test {

    use crate::game_state::FiarGameState;

    use super::super::game_state::TTTGameState;
    use super::GameEvaluator;

    #[test]
    fn test_expanding_ttt_one_level() -> Result<(), &'static str> {
        let mut t3_eval: GameEvaluator<TTTGameState> = GameEvaluator::new(3, 3, 9);

        t3_eval.init_expansion(2);

        loop {
            match t3_eval.expand_by(5) {
                Err(e) => return Err(e),
                Ok(false) => (),
                Ok(true) => break,
            }
        }

        Ok(())
    }

    #[test]
    fn test_expanding_fiar() -> Result<(), &'static str> {
        let mut fiar_eval: GameEvaluator<FiarGameState> = GameEvaluator::new(7, 6, 6);

        fiar_eval.init_expansion(2);

        loop {
            match fiar_eval.expand_by(5) {
                Err(e) => return Err(e),
                Ok(false) => (),
                Ok(true) => break,
            }
        }
        Ok(())
    }
}
