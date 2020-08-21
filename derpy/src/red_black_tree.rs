extern crate generational_arena;
use generational_arena::{Arena, Index};
use std::collections::{HashMap, VecDeque};

use std::fmt::Display;
use std::cmp::{PartialOrd};

use super::base_tree::{BfsIter, DfsIter, Leaf, Node};

#[derive(Default)]
pub struct RedBlackTree<T: PartialOrd + Display> {
    root: Option<Index>,
    size: u64,
    nodes: Arena<Node<T>>,
    colors: HashMap<Index, TreeColors>
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

impl<T: std::cmp::PartialOrd + std::fmt::Display> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree {
            root: None,
            size: 0,
            nodes: Arena::new(),
            colors: HashMap::new(),
        }
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn insert(&mut self, val: T) {
        let new_leaf = Leaf {
            data: val,
            left: None,
            right: None,
            parent: None,
        };

        self.bst_insert(new_leaf);
        self.size += 1;
    }

    fn get_node(&self, index: Index) -> &Node<T> {
        let leaf = self
            .nodes
            .get(index)
            .expect("Failed to get node from index");

        leaf
    }

    fn bst_insert(&mut self, mut new_leaf: Leaf<T>) {
        let mut cur_idx_option = self.root;
        let mut left_side = false;

        while let Some(cur_leaf_idx) = cur_idx_option {
            let cur_leaf = self
                .nodes
                .get_mut(cur_leaf_idx)
                .expect("Attempted to reference a node which no longer exists");

            left_side = new_leaf.data < cur_leaf.data;

            // choose a side to insert the new leaf
            let next_leaf_idx_option = if left_side {
                &mut cur_leaf.left
            } else {
                &mut cur_leaf.right
            };

            match next_leaf_idx_option {
                None => {
                    break;
                }
                _ => cur_idx_option = *next_leaf_idx_option,
            }
        }

        new_leaf.parent = cur_idx_option;
        let leaf_id = self.nodes.insert(Box::new(new_leaf));

        self.colors.insert(
            leaf_id,
            TreeColors::Red
        );

        if let Some(parent_idx) = cur_idx_option {
            let parent = self.nodes.get_mut(parent_idx).unwrap();

            if left_side {
                parent.left = Some(leaf_id);
            } else {
                parent.right = Some(leaf_id);
            }
        } else {
            self.root = Some(leaf_id);
        }
    }

    fn recolor_tree(&mut self) {
        let _starting_node = self.root.as_mut();
    }

    pub fn bfs_iter(&mut self) -> BfsIter<T> {
        let mut leaf_idx_queue = VecDeque::new();
        if let Some(root_idx) = self.root {
            leaf_idx_queue.push_front(root_idx);
        }

        BfsIter {
            leaf_idx_queue,
            nodes: &self.nodes,
        }
    }

    // Initialize a new iterator struct w/ the root node as the starting point
    pub fn dfs_iter(&mut self) -> DfsIter<T> {
        let mut leaf_idx_stack = Vec::new();

        if let Some(root_idx) = self.root {
            leaf_idx_stack.push(root_idx);
        }

        DfsIter {
            leaf_idx_stack,
            nodes: &self.nodes,
        }
    }
}

