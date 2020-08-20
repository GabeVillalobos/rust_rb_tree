extern crate generational_arena;
use super::base_tree::{ Leaf, Node, DfsIter, BfsIter, NodeNotFoundErr };
use std::collections::VecDeque;
use std::fmt::Display;
use std::cmp::{ PartialOrd, PartialEq };

use generational_arena::{Arena, Index};


#[derive(Default)]
pub struct BinarySearchTree<T: PartialOrd + Display> {
    root: Option<Index>,
    size: u64,
    nodes: Arena<Node<T>>,
}

impl<T: std::cmp::PartialOrd + std::fmt::Display> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            size: 0,
            nodes: Arena::new(),
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

    fn bst_insert(&mut self, new_leaf: Leaf<T>) {
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

        let leaf_id = self.nodes.insert(Box::new(new_leaf));
        println!("Idx: {:?}", leaf_id);
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

    pub fn remove(item: &T) -> Result<(), NodeNotFoundErr<T>>{
        unimplemented!()
    }

    // Create a new iterator w/ a stack for DFS taversal
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

    // Create a new iterator w/ a queue for BFS traversal
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
}
