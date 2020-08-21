extern crate generational_arena;
use super::base_tree::{BfsIter, DfsIter, Leaf, Node};
use super::tree_errs::NodeNotFoundErr;

use std::cmp::PartialOrd;
use std::collections::VecDeque;
use std::fmt::Display;

use generational_arena::{Arena, Index};

#[derive(Default)]
pub struct BinarySearchTree<T: PartialOrd + Display> {
    root: Option<Index>,
    size: usize,
    nodes: Arena<Node<T>>,
}

impl<T: PartialOrd + Display> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            size: 0,
            nodes: Arena::new(),
        }
    }

    pub fn get_size(&self) -> usize {
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

    fn bst_insert(&mut self, mut new_leaf: Leaf<T>) {
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
    }

    fn find_node_index(&self, item: &T) -> Option<Index> {
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

    fn get_inorder_successor(&self, leaf: &Leaf<T>) -> Option<Index> {
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

    pub fn find(&self, item: &T) -> bool {
        self.find_node_index(item).is_some()
    }

    pub fn remove(&mut self, item: &T) -> Result<(), NodeNotFoundErr> {
        // Grab larger of 2 nodes, place as root
        // Then if left node of root exists, set larger node left = root.left, insert orphaned node into sub-tree.
        let idx = self
            .find_node_index(item)
            .ok_or_else(|| NodeNotFoundErr {})?;

        let removed_leaf = self.nodes.remove(idx).expect(
            "exclusive access during mutation guarantees that a node exists for each index",
        );

        let mut replacement_leaf_opt = None;
        match (removed_leaf.left, removed_leaf.right) {
            (Some(solo_child_idx), None) | (None, Some(solo_child_idx)) => {
                replacement_leaf_opt = Some(solo_child_idx);
            }
            // Both children exist, so we must find the inorder successor first
            (Some(left_child_idx), Some(right_child_idx)) => {
                replacement_leaf_opt = self.get_inorder_successor(&removed_leaf);

                // Replace node with its inorder successor
                if let Some(successor_idx) = replacement_leaf_opt {
                    // Set inorder successor as the new parent node of the removed node's children
                    let left_leaf = &mut self.nodes[left_child_idx];
                    left_leaf.parent = replacement_leaf_opt;

                    let right_leaf = &mut self.nodes[right_child_idx];
                    right_leaf.parent = replacement_leaf_opt;

                    let successor_leaf = &mut self.nodes[successor_idx];
                    successor_leaf.left = removed_leaf.left;
                    successor_leaf.right = removed_leaf.right;

                    let successor_parent_opt = successor_leaf.parent;

                    // If successor's parent exists and is not the node being
                    //   removed, we remove the successor node from it
                    if let (Some(parent_idx), true) =
                        (successor_parent_opt, successor_parent_opt != Some(idx))
                    {
                        let successor_parent_leaf = &mut self.nodes[parent_idx];

                        if successor_parent_leaf.left == replacement_leaf_opt {
                            successor_parent_leaf.left = None;
                        } else {
                            successor_parent_leaf.right = None;
                        }
                    }
                }
            }
            // Do nothing because we handle the "leaf" case below
            (None, None) => {}
        }

        // update parent node with new replacement child node
        if let Some(parent_idx) = removed_leaf.parent {
            let parent_leaf = &mut self.nodes[parent_idx];
            if parent_leaf.left == Some(idx) {
                parent_leaf.left = replacement_leaf_opt;
            } else {
                parent_leaf.right = replacement_leaf_opt;
            }
        }

        // Update replacement node with new parent node
        if let Some(replacement_node_idx) = replacement_leaf_opt {
            let replacement_node = &mut self.nodes[replacement_node_idx];
            replacement_node.parent = removed_leaf.parent;
        }

        // if the removed node was the root, replace it with the replacement node
        if self.root == Some(idx) {
            self.root = replacement_leaf_opt;
        }

        self.size -= 1;

        Ok(())
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
