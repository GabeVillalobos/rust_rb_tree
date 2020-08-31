use generational_arena::Index;
use std::collections::{HashMap, VecDeque};

use super::base_tree::{BfsIter, DfsIter, Node, InternalBinarySearchTree};
use super::tree_errs::NodeNotFoundErr;

use std::cmp::PartialOrd;
use std::fmt::Display;

#[derive(Default)]
pub struct RedBlackTree<T: PartialOrd + Display + Default> {
    bst: InternalBinarySearchTree<T>,
    colors: HashMap<Index, TreeColors>,
}

#[derive(Debug)]
pub(crate) enum TreeColors {
    Red,
    Black,
}

impl Default for TreeColors {
    fn default() -> TreeColors {
        TreeColors::Red
    }
}

impl<T: PartialOrd + Display + Default> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree {
            bst: InternalBinarySearchTree::new(),
            colors: HashMap::new(),
        }
    }

    pub fn get_size(&self) -> usize {
        self.bst.nodes.len()
    }

    pub fn insert(&mut self, item: T) {
        let leaf = Node {
            data: item,
            left: None,
            right: None,
            parent: None,
        };

        self.bst.insert_node(leaf);
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
