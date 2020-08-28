extern crate generational_arena;
use super::base_tree::{BfsIter, DfsIter, Node, Tree};
use super::binary_search_tree_helpers::{find_node_index, insert_node, remove_node};
use super::tree_errs::NodeNotFoundErr;

use std::cmp::PartialOrd;
use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Default)]
pub struct BinarySearchTree<T: PartialOrd + Display + Copy> {
    tree: Tree<T>,
}

impl<T: PartialOrd + Display + Copy> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { tree: Tree::new() }
    }

    pub fn get_size(&self) -> usize {
        self.tree.size
    }

    pub fn insert(&mut self, val: T) {
        let new_leaf = Node {
            data: val,
            left: None,
            right: None,
            parent: None,
        };

        insert_node(&mut self.tree, new_leaf);
        self.tree.size += 1;
    }

    pub fn contains(&self, item: &T) -> bool {
        find_node_index(&self.tree, item).is_some()
    }

    pub fn remove(&mut self, item: &T) -> Result<(), NodeNotFoundErr> {
        let node_idx_to_remove = find_node_index(&self.tree, item).ok_or(NodeNotFoundErr)?;
        remove_node(&mut self.tree, node_idx_to_remove);

        Ok(())
    }

    // Create a new iterator w/ a stack for DFS taversal
    pub fn dfs_iter(&mut self) -> DfsIter<T> {
        let mut node_idx_stack = Vec::new();

        if let Some(root_idx) = self.tree.root {
            node_idx_stack.push(root_idx);
        }

        DfsIter {
            node_idx_stack,
            nodes: &self.tree.nodes,
        }
    }

    // Create a new iterator w/ a queue for BFS traversal
    pub fn bfs_iter(&mut self) -> BfsIter<T> {
        let mut node_idx_queue = VecDeque::new();

        if let Some(root_idx) = self.tree.root {
            node_idx_queue.push_front(root_idx);
        }

        BfsIter {
            node_idx_queue,
            nodes: &self.tree.nodes,
        }
    }
}
