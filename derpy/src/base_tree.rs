use generational_arena::{Arena, Index};
use std::collections::VecDeque;
use std::fmt::{Display, Result, Formatter};

#[derive(Debug, Clone)]
pub struct NodeNotFoundErr<'a, T: std::fmt::Display> {
    node: &'a Node<T>
}

impl<T: Display> Display for NodeNotFoundErr<'_, T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Node with value {} was not found", self.node.data)
    }
}

#[derive(Default, Debug)]
pub struct Leaf<T: std::fmt::Display> {
    pub data: T,
    pub left: Option<Index>,
    pub right: Option<Index>,
    pub parent: Option<Index>, // Optional for doubly-linked trees
}

pub type Node<T> = Box<Leaf<T>>;

pub struct DfsIter<'a, T: Display> {
    pub leaf_idx_stack: Vec<Index>,
    pub nodes: &'a Arena<Node<T>>,
}

// Iterate through leaves using depth-first traversal
impl<'a, T: std::cmp::PartialOrd + std::fmt::Display> Iterator for DfsIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.leaf_idx_stack.pop().map(|leaf_idx| {
            let cur_leaf_option = self.nodes.get(leaf_idx);
            match cur_leaf_option {
                Some(cur_leaf) => {
                    if let Some(right_leaf_idx) = cur_leaf.right {
                        self.leaf_idx_stack.push(right_leaf_idx);
                    }

                    if let Some(left_leaf_idx) = cur_leaf.left {
                        self.leaf_idx_stack.push(left_leaf_idx);
                    }

                    &cur_leaf.data
                }
                None => panic!("Attempted to access an empty node"),
            }
        })
    }
}

pub struct BfsIter<'a, T: Display> {
    pub leaf_idx_queue: VecDeque<Index>,
    pub nodes: &'a Arena<Node<T>>,
}

// Iterate through leaves using depth-first traversal
impl<'a, T: std::cmp::PartialOrd + std::fmt::Display> Iterator for BfsIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.leaf_idx_queue.pop_front().map(|leaf_idx| {
            let cur_leaf_option = self.nodes.get(leaf_idx);
            match cur_leaf_option {
                Some(cur_leaf) => {
                    if let Some(left_leaf_idx) = cur_leaf.left {
                        self.leaf_idx_queue.push_back(left_leaf_idx);
                    }

                    if let Some(right_leaf_idx) = cur_leaf.right {
                        self.leaf_idx_queue.push_back(right_leaf_idx);
                    }

                    &cur_leaf.data
                }
                None => panic!("Attempted to access an empty node"),
            }
        })
    }
}