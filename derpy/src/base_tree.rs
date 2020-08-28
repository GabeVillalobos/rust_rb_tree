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

// Iterate through nodes using depth-first traversal
impl<'a, T: PartialOrd + Display> Iterator for DfsIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node_idx = self.node_idx_stack.pop()?;
        let cur_node = &self.nodes[node_idx];

        if let Some(right_node_idx) = cur_node.right {
            self.node_idx_stack.push(right_node_idx);
        }

        if let Some(left_node_idx) = cur_node.left {
            self.node_idx_stack.push(left_node_idx);
        }

        Some(&cur_node.data)
    }
}

pub struct BfsIter<'a, T: Display> {
    pub node_idx_queue: VecDeque<Index>,
    pub nodes: &'a Arena<BoxedNode<T>>,
}

// Iterate through nodes using breadth-first traversal
impl<'a, T: std::cmp::PartialOrd + std::fmt::Display> Iterator for BfsIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node_idx = self.node_idx_queue.pop_front()?;
        let cur_node = &self.nodes[node_idx];

        // println!(
        //     "idx: {:?} val: {} parent: {:?}  left: {:?} right: {:?}",
        //     node_idx, cur_node.data, cur_node.parent, cur_node.left, cur_node.right
        // );

        if let Some(left_node_idx) = cur_node.left {
            self.node_idx_queue.push_back(left_node_idx);
        }

        if let Some(right_node_idx) = cur_node.right {
            self.node_idx_queue.push_back(right_node_idx);
        }

        Some(&cur_node.data)
    }
}
