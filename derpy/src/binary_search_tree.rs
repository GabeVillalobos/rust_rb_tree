use super::base_tree::{BfsIter, DfsIter, InternalBinarySearchTree, Node};
use super::tree_errs::NodeNotFoundErr;

use std::cmp::PartialOrd;
use std::collections::VecDeque;
use std::fmt::Display;

// Public class that wraps the internal Binary Search Tree impl without
//   leaking any abstractions.
#[derive(Default)]
pub struct BinarySearchTree<T: PartialOrd + Display + Default> {
    bst: InternalBinarySearchTree<T>,
}

impl<T: PartialOrd + Display + Default> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree {
            bst: InternalBinarySearchTree::new(),
        }
    }

    pub fn get_size(&self) -> usize {
        self.bst.nodes.len()
    }

    pub fn insert(&mut self, val: T) {
        let new_leaf = Node {
            data: val,
            left: None,
            right: None,
            parent: None,
        };

        self.bst.insert_node(new_leaf);
    }

    pub fn contains(&self, item: &T) -> bool {
        self.bst.find_node_index(item).is_some()
    }

    pub fn remove(&mut self, item: &T) -> Result<(), NodeNotFoundErr> {
        let node_idx_to_remove = self.bst.find_node_index(item).ok_or(NodeNotFoundErr)?;
        self.bst.remove_node(node_idx_to_remove);

        Ok(())
    }

    // Create a new iterator w/ a stack for DFS taversal
    pub fn dfs_iter(&mut self) -> DfsIter<T> {
        let mut node_idx_stack = Vec::new();

        if let Some(root_idx) = self.bst.root {
            node_idx_stack.push(root_idx);
        }

        DfsIter {
            node_idx_stack,
            nodes: &self.bst.nodes,
        }
    }

    // Create a new iterator w/ a queue for BFS traversal
    pub fn bfs_iter(&mut self) -> BfsIter<T> {
        let mut node_idx_queue = VecDeque::new();

        if let Some(root_idx) = self.bst.root {
            node_idx_queue.push_front(root_idx);
        }

        BfsIter {
            node_idx_queue,
            nodes: &self.bst.nodes,
        }
    }
}
