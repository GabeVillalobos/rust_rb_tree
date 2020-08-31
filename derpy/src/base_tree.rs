use generational_arena::{Arena, Index};
use std::collections::VecDeque;
use std::fmt::Display;
use std::mem::take;

#[derive(Default, Debug)]
pub struct Node<T: Display> {
    pub data: T,
    pub left: Option<Index>,
    pub right: Option<Index>,
    pub parent: Option<Index>, // Optional for doubly-linked trees
}

pub type BoxedNode<T> = Box<Node<T>>;

#[derive(Default)]
pub struct InternalBinarySearchTree<T: Display + PartialOrd + Default> {
    pub root: Option<Index>,
    pub nodes: Arena<BoxedNode<T>>,
}

// The copy trait facilitates easier node removal by allowing us to
//  copy the contents of
impl<T: Display + PartialOrd + Default> InternalBinarySearchTree<T> {
    pub fn new() -> Self {
        InternalBinarySearchTree {
            root: None,
            nodes: Arena::new(),
        }
    }

    pub fn insert_node(&mut self, mut new_leaf: Node<T>) -> Index {
        let mut cur_idx_option = self.root;
        let mut left_side = false;

        while let Some(cur_node_idx) = cur_idx_option {
            let cur_node = &mut self.nodes[cur_node_idx];

            left_side = new_leaf.data < cur_node.data;

            // choose a side to insert the new leaf
            let next_node_idx_option = if left_side {
                &mut cur_node.left
            } else {
                &mut cur_node.right
            };

            match next_node_idx_option {
                None => {
                    break;
                }
                _ => cur_idx_option = *next_node_idx_option,
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
        let mut cur_node_opt = self.root;
        while let Some(node_idx) = cur_node_opt {
            let node = &self.nodes[node_idx];

            if node.data == *item {
                return Some(node_idx);
            }

            if *item < node.data {
                cur_node_opt = node.left;
            } else {
                cur_node_opt = node.right;
            }
        }
        None
    }

    // Note: This is an incomplete implementation that does not currently
    //   handle the case where children/no left nodes exist.
    //   The future plan for this is to traverse up each parent until a left node is found
    pub fn get_inorder_successor(&self, node: &Node<T>) -> Option<Index> {
        // Start with greater node, return smallest node in this sub-tree
        let mut cur_node_opt = node.right;

        while let Some(cur_node_idx) = cur_node_opt {
            let cur_node = &self.nodes[cur_node_idx];

            if cur_node.left.is_none() {
                break;
            }

            cur_node_opt = cur_node.left;
        }

        cur_node_opt
    }

    // Recursive function that takes a Tree, a starting "root", and a node index to remove.
    //  Starting at the root, we find the specified node and remove it from the tree.
    //  When removing a node with multiple children, things get a little more complicated.
    //  First, the node's data is swapped with its inorder successor's, then the inorder
    //  successor is removed recursively.
    pub fn remove_node(&mut self, node_idx: Index) {
        let mut replacement_idx_opt = None;
        let mut recursive_remove = false;

        let node_to_remove = &self.nodes[node_idx];
        // Grab the parent index, in case we need it later
        let parent_opt = node_to_remove.parent;

        match (node_to_remove.left, node_to_remove.right) {
            // Only one child exists, so we swap the parent with the child
            (Some(solo_child_idx), None) | (None, Some(solo_child_idx)) => {
                replacement_idx_opt = Some(solo_child_idx);
            }
            // Both children exist, so we must find the inorder successor first
            (Some(_), Some(_)) => {
                let inorder_successor = self.get_inorder_successor(&node_to_remove);

                let successor_idx = inorder_successor
                .expect("Proper tree structure ensures that every node with children has an inorder successor");

                // Swap value of node to be deleted with its inorder successor's
                let successor_data = take(&mut self.nodes[successor_idx].data);
                self.nodes[node_idx].data = successor_data;

                recursive_remove = true;

                self.remove_node(successor_idx);
            }

            // If it's a leaf, just remove the reference from it's parent and delete the node
            (None, None) => {}
        }

        // Handles removing leaves & relinking leaves with a single child
        if !recursive_remove {
            if let Some(parent_idx) = parent_opt {
                let parent_node = &mut self.nodes[parent_idx];

                if parent_node.left == Some(node_idx) {
                    parent_node.left = replacement_idx_opt;
                } else {
                    parent_node.right = replacement_idx_opt;
                }
            } else {
                // It's the root node, and we nuke the world
                self.root = None;
            }

            if let Some(replacement_idx) = replacement_idx_opt {
                let replacement_node = &mut self.nodes[replacement_idx];
                replacement_node.parent = parent_opt;
            }

            // Finally, remove the node from the arena itself
            let _ = self.nodes.remove(node_idx).expect(
                "Exclusive access during mutation ensures that a node exists for every index",
            );
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
