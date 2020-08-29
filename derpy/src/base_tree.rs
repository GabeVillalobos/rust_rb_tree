use generational_arena::{Arena, Index};
use std::collections::VecDeque;
use std::fmt::Display;
use std::mem::take;

#[derive(Default, Debug)]
pub struct Leaf<T: Display> {
    pub data: T,
    pub left: Option<Index>,
    pub right: Option<Index>,
    pub parent: Option<Index>, // Optional for doubly-linked trees
}

pub type Node<T> = Box<Leaf<T>>;

#[derive(Default)]
pub struct Tree<T: Display + PartialOrd + Default> {
    pub root: Option<Index>,
    pub nodes: Arena<Node<T>>,
}

// The copy trait facilitates easier node removal by allowing us to
//  copy the contents of
impl<T: Display + PartialOrd + Default> Tree<T> {
    pub fn new() -> Self {
        Tree {
            root: None,
            nodes: Arena::new(),
        }
    }

    pub fn insert_leaf(&mut self, mut new_leaf: Leaf<T>) -> Index {
        let mut cur_idx_option = self.root;
        let mut left_side = false;

        while let Some(cur_leaf_idx) = cur_idx_option {
            let cur_leaf = &mut self.nodes[cur_leaf_idx];

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

        // Parent node found, so we set it to the corresponding child node
        if let Some(parent_idx) = cur_idx_option {
            let parent = &mut self.nodes[parent_idx];

            if left_side {
                parent.left = Some(leaf_id);
            } else {
                parent.right = Some(leaf_id);
            }
        } else {
            // No parent found, so this leaf must be the root
            self.root = Some(leaf_id);
        }

        leaf_id
    }

    pub fn find_node_index(&self, item: &T) -> Option<Index> {
        let mut cur_leaf_opt = self.root;
        while let Some(node_idx) = cur_leaf_opt {
            let leaf = &self.nodes[node_idx];

            if leaf.data == *item {
                return Some(node_idx);
            }

            if *item < leaf.data {
                cur_leaf_opt = leaf.left;
            } else {
                cur_leaf_opt = leaf.right;
            }
        }
        None
    }

    // Note: This is an incomplete implementation that does not currently
    //   handle the case where children/no left nodes exist.
    //   The future plan for this is to traverse up each parent until a left node is found
    pub fn get_inorder_successor(&self, leaf: &Leaf<T>) -> Option<Index> {
        // Start with greater node, return smallest node in this sub-tree
        let mut cur_leaf_opt = leaf.right;

        while let Some(cur_leaf_idx) = cur_leaf_opt {
            let cur_leaf = &self.nodes[cur_leaf_idx];

            if cur_leaf.left.is_none() {
                break;
            }

            cur_leaf_opt = cur_leaf.left;
        }

        cur_leaf_opt
    }

    // Recursive function that takes a Tree, a starting "root", and a leaf index to remove.
    //  Starting at the root, we find the specified node and remove it from the tree.
    //  When removing a leaf with multiple children, things get a little more complicated.
    //  First, the node's data is swapped with its inorder successor's, then the inorder
    //  successor is removed recursively.
    pub fn remove_leaf(&mut self, leaf_idx: Index) {
        let mut replacement_idx_opt = None;
        let mut recursive_remove = false;

        let leaf_to_remove = &self.nodes[leaf_idx];
        // Grab the parent index, in case we need it later
        let parent_opt = leaf_to_remove.parent;

        match (leaf_to_remove.left, leaf_to_remove.right) {
            // Only one child exists, so we swap the parent with the child
            (Some(solo_child_idx), None) | (None, Some(solo_child_idx)) => {
                replacement_idx_opt = Some(solo_child_idx);
            }
            // Both children exist, so we must find the inorder successor first
            (Some(_), Some(_)) => {
                let inorder_successor = self.get_inorder_successor(&leaf_to_remove);

                let successor_idx = inorder_successor
                .expect("Proper tree structure ensures that every node with children has an inorder successor");

                // Swap value of leaf to be deleted with its inorder successor's
                let successor_data = take(&mut self.nodes[successor_idx].data);
                self.nodes[leaf_idx].data = successor_data;

                recursive_remove = true;

                self.remove_leaf(successor_idx);
            }

            // If it's a leaf, just remove the reference from it's parent and delete the node
            (None, None) => {}
        }

        // Handles removing leaves & relinking leaves with a single child
        if !recursive_remove {
            // let previous_parent = leaf_to_remove.parent;
            if let Some(parent_idx) = parent_opt {
                let parent_node = &mut self.nodes[parent_idx];

                if parent_node.left == Some(leaf_idx) {
                    parent_node.left = replacement_idx_opt;
                } else {
                    parent_node.right = replacement_idx_opt;
                }
            } else {
                // It's the root node, and we nuke the world
                self.root = None;
            }

            if let Some(replacement_idx) = replacement_idx_opt {
                let replacement_leaf = &mut self.nodes[replacement_idx];
                replacement_leaf.parent = parent_opt;
            }

            // Finally, remove the node from the arena itself
            let _ = self.nodes.remove(leaf_idx).expect(
                "Exclusive access during mutation ensures that a node exists for every index",
            );
        }
    }
}

pub struct DfsIter<'a, T: Display> {
    pub leaf_idx_stack: Vec<Index>,
    pub nodes: &'a Arena<Node<T>>,
}

// Iterate through leaves using depth-first traversal
impl<'a, T: PartialOrd + Display> Iterator for DfsIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let leaf_idx = self.leaf_idx_stack.pop()?;
        let cur_leaf = &self.nodes[leaf_idx];

        if let Some(right_leaf_idx) = cur_leaf.right {
            self.leaf_idx_stack.push(right_leaf_idx);
        }

        if let Some(left_leaf_idx) = cur_leaf.left {
            self.leaf_idx_stack.push(left_leaf_idx);
        }

        Some(&cur_leaf.data)
    }
}

pub struct BfsIter<'a, T: Display> {
    pub leaf_idx_queue: VecDeque<Index>,
    pub nodes: &'a Arena<Node<T>>,
}

// Iterate through leaves using breadth-first traversal
impl<'a, T: std::cmp::PartialOrd + std::fmt::Display> Iterator for BfsIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let leaf_idx = self.leaf_idx_queue.pop_front()?;
        let cur_leaf = &self.nodes[leaf_idx];

        // println!(
        //     "idx: {:?} val: {} parent: {:?}  left: {:?} right: {:?}",
        //     leaf_idx, cur_leaf.data, cur_leaf.parent, cur_leaf.left, cur_leaf.right
        // );

        if let Some(left_leaf_idx) = cur_leaf.left {
            self.leaf_idx_queue.push_back(left_leaf_idx);
        }

        if let Some(right_leaf_idx) = cur_leaf.right {
            self.leaf_idx_queue.push_back(right_leaf_idx);
        }

        Some(&cur_leaf.data)
    }
}
