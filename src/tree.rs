use std::cell::RefCell;
use std::cmp::Eq;
use std::collections::VecDeque;
use std::ops::Deref;
use std::rc::Rc;

use std::hash::{Hash, Hasher};
use std::ptr;

pub type NodeW<T> = Rc<RefCell<Node<T>>>;

/// Node with address as hash-target and comparison criterion.
///
/// In the evaluator, we build a `HashSet` of nodes that we have already visited. In this set,
/// we want nodes to be hashed and compared by their memory address, not by their content.
pub struct AddressHashNodeW<T> {
    pub inner: NodeW<T>,
}

impl<T> Deref for AddressHashNodeW<T> {
    type Target = NodeW<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Hash for AddressHashNodeW<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(&*(self.inner), state);
    }
}

impl<T> PartialEq for AddressHashNodeW<T> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(&*(self.inner), &*(other.inner))
    }
}

impl<T> Eq for AddressHashNodeW<T> {}

#[derive(Debug)]
pub struct Node<T> {
    pub parent: Option<NodeW<T>>,
    pub children: Vec<NodeW<T>>,
    pub data: T,
}

#[allow(dead_code)]
impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node {
            parent: None,
            children: Vec::new(),
            data,
        }
    }

    pub fn new_wrapped(data: T) -> NodeW<T> {
        Rc::new(RefCell::new(Node {
            parent: None,
            children: Vec::new(),
            data,
        }))
    }

    pub fn add_child_to_parent(child: NodeW<T>, parent: NodeW<T>) {
        // Set parent for child
        {
            (*child.borrow_mut()).parent = Some(Rc::clone(&parent));
        }

        // Add child to parent
        {
            (*parent.borrow_mut()).children.push(Rc::clone(&child));
        }
    }

    pub fn parent(node: NodeW<T>) -> Option<NodeW<T>> {
        (*node.borrow()).parent.clone()
    }

    pub fn children(node: NodeW<T>) -> Vec<NodeW<T>> {
        (*node.borrow()).children.clone()
    }
}

pub struct TreeIterator<T> {
    buffer: VecDeque<NodeW<T>>,
}

#[allow(dead_code)]
pub fn new_tree_iter<T>(start_node: NodeW<T>) -> TreeIterator<T> {
    let mut buffer = VecDeque::new();
    buffer.push_back(start_node);

    TreeIterator::<T> { buffer }
}

impl<T> Iterator for TreeIterator<T> {
    type Item = NodeW<T>;

    fn next(&mut self) -> Option<NodeW<T>> {
        let next_node = match self.buffer.pop_front() {
            Some(node) => {
                // Append children of this node
                let mut children: VecDeque<NodeW<T>> =
                    Node::children(Rc::clone(&node)).into_iter().collect();
                self.buffer.append(&mut children);

                Some(node)
            }
            None => None,
        };

        next_node
    }
}

/// Groups a node with the relative depth in the graph
#[derive(Clone)]
pub struct NodeDepth<T> {
    pub node: NodeW<T>,
    pub depth: i32,
}

pub struct TreeDepthIterator<T> {
    buffer: VecDeque<NodeDepth<T>>,
    max_depth: i32,
}

pub fn new_tree_depth_iter<T>(start_node: NodeW<T>, max_depth: i32) -> TreeDepthIterator<T> {
    let mut init_buffer = VecDeque::new();
    init_buffer.push_back(NodeDepth::<T> {
        node: start_node,
        depth: 0,
    });

    TreeDepthIterator::<T> {
        buffer: init_buffer,
        max_depth,
    }
}

impl<T> TreeDepthIterator<T> {
    pub fn append(&mut self, new_node_info: NodeDepth<T>) {
        if new_node_info.depth <= self.max_depth {
            self.buffer.push_back(new_node_info);
        }
    }

    pub fn from_vecdeque(
        vec_deque: VecDeque<NodeDepth<T>>,
        max_depth: i32,
    ) -> TreeDepthIterator<T> {
        TreeDepthIterator::<T> {
            buffer: vec_deque,
            max_depth,
        }
    }
}

impl<T> Iterator for TreeDepthIterator<T> {
    type Item = NodeDepth<T>;

    fn next(&mut self) -> Option<NodeDepth<T>> {
        let next_node_info = match self.buffer.pop_front() {
            Some(node_info) => {
                match node_info.depth {
                    depth if depth < self.max_depth => {
                        // Append children of this node
                        let children = Node::children(Rc::clone(&node_info.node));
                        for child in children {
                            self.buffer.push_back(NodeDepth {
                                node: child,
                                depth: depth + 1,
                            });
                        }

                        Some(node_info)
                    }
                    depth if depth == self.max_depth => Some(node_info),
                    _ => None,
                }
            }
            None => None,
        };

        next_node_info
    }
}

#[cfg(test)]
mod test {

    use super::{new_tree_depth_iter, new_tree_iter, Node, NodeDepth};
    use std::rc::Rc;

    #[test]
    fn test_node_new() {
        let int_node_root: Node<i32> = Node::new(7);
        assert!(int_node_root.parent.is_none());
        assert!(int_node_root.children.is_empty());
        assert_eq!(int_node_root.data, 7);
    }

    #[test]
    fn test_node_new_wrapped() {
        let root = Node::new_wrapped(7);
        assert_eq!((*root.borrow()).data, 7);
    }

    #[test]
    fn test_add_child_to_parent() {
        let root = Node::new_wrapped(7);
        let child1 = Node::new_wrapped(15);
        let child2 = Node::new_wrapped(16);

        // Add children separately
        Node::add_child_to_parent(Rc::clone(&child1), Rc::clone(&root));
        Node::add_child_to_parent(Rc::clone(&child2), Rc::clone(&root));
    }

    #[test]
    fn test_get_parent_and_children() {
        let root = Node::new_wrapped(7);
        let child1 = Node::new_wrapped(15);
        let child2 = Node::new_wrapped(16);
        let child3 = Node::new_wrapped(17);

        let children = [Rc::clone(&child1), Rc::clone(&child2), Rc::clone(&child3)];
        for child in children.iter() {
            assert!(Node::parent(Rc::clone(&child)).is_none());
        }

        for child in children.iter() {
            // Add children
            Node::add_child_to_parent(Rc::clone(&child), Rc::clone(&root));
        }

        assert_eq!(Node::children(root).len(), 3);

        for child in children.iter() {
            assert!(Node::parent(Rc::clone(&child)).is_some());
        }
    }

    #[test]
    fn test_tree_iterator_iter() {
        let root = Node::new_wrapped(7);
        let child1 = Node::new_wrapped(15);
        let child2 = Node::new_wrapped(16);
        let child3 = Node::new_wrapped(17);

        let children = [Rc::clone(&child1), Rc::clone(&child2), Rc::clone(&child3)];
        for child in children.iter() {
            // Add children
            Node::add_child_to_parent(Rc::clone(&child), Rc::clone(&root));
        }

        let expected_full_iterator_output = vec![
            Rc::clone(&root),
            Rc::clone(&child1),
            Rc::clone(&child2),
            Rc::clone(&child3),
        ];

        let tree_iter = new_tree_iter(Rc::clone(&root));

        for (a, b) in tree_iter.zip(expected_full_iterator_output) {
            assert_eq!((*a).borrow().data, (*b).borrow().data);
        }
    }

    #[test]
    fn test_tree_depth_iterator_iter() {
        let root = Node::new_wrapped(0);
        let child1 = Node::new_wrapped(1);
        let child2 = Node::new_wrapped(1);
        let child3 = Node::new_wrapped(1);

        let children = [Rc::clone(&child1), Rc::clone(&child2), Rc::clone(&child3)];
        for child in children.iter() {
            // Add children
            Node::add_child_to_parent(Rc::clone(&child), Rc::clone(&root));
        }

        let child1child1 = Node::new_wrapped(2);
        Node::add_child_to_parent(Rc::clone(&child1child1), Rc::clone(&child1));
        let child1child1child1 = Node::new_wrapped(3);
        Node::add_child_to_parent(Rc::clone(&child1child1child1), Rc::clone(&child1child1));

        let iterator_output: Vec<NodeDepth<i32>> =
            new_tree_depth_iter(Rc::clone(&root), 2).collect();
        assert_eq!(iterator_output.len(), 5);

        for node_info in iterator_output {
            assert_eq!((*node_info.node).borrow().data, node_info.depth);
        }
    }
}
