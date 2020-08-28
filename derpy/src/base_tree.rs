use generational_arena::{Arena, Index};
use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Default, Debug)]
pub struct Node<T: Display> {
    pub data: T,
    pub left: Option<Index>,
    pub right: Option<Index>,
    pub parent: Option<Index>, // Optional for doubly-linked trees
}

pub type BoxedNode<T> = Box<Node<T>>;

#[derive(Default)]
pub struct Tree<T: Display + PartialOrd + Copy> {
    pub root: Option<Index>,
    pub size: usize,
    pub nodes: Arena<BoxedNode<T>>,
}

impl<T: Display + PartialOrd + Copy> Tree<T> {
    pub fn new() -> Self {
        Tree {
            root: None,
            size: 0,
            nodes: Arena::new(),
        }
    }
}

pub struct DfsIter<'a, T: Display> {
    pub node_idx_stack: Vec<Index>,
    pub nodes: &'a Arena<BoxedNode<T>>,
}

// Iterate through leaves using depth-first traversal
impl<'a, T: PartialOrd + Display> Iterator for DfsIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let leaf_idx = self.node_idx_stack.pop()?;
        let cur_leaf = &self.nodes[leaf_idx];

        if let Some(right_leaf_idx) = cur_leaf.right {
            self.node_idx_stack.push(right_leaf_idx);
        }

        if let Some(left_leaf_idx) = cur_leaf.left {
            self.node_idx_stack.push(left_leaf_idx);
        }

        Some(&cur_leaf.data)
    }
}

pub struct BfsIter<'a, T: Display> {
    pub node_idx_queue: VecDeque<Index>,
    pub nodes: &'a Arena<BoxedNode<T>>,
}

// Iterate through leaves using breadth-first traversal
impl<'a, T: std::cmp::PartialOrd + std::fmt::Display> Iterator for BfsIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let leaf_idx = self.node_idx_queue.pop_front()?;
        let cur_leaf = &self.nodes[leaf_idx];

        // println!(
        //     "idx: {:?} val: {} parent: {:?}  left: {:?} right: {:?}",
        //     leaf_idx, cur_leaf.data, cur_leaf.parent, cur_leaf.left, cur_leaf.right
        // );

        if let Some(left_leaf_idx) = cur_leaf.left {
            self.node_idx_queue.push_back(left_leaf_idx);
        }

        if let Some(right_leaf_idx) = cur_leaf.right {
            self.node_idx_queue.push_back(right_leaf_idx);
        }

        Some(&cur_leaf.data)
    }
}
