use generational_arena::Index;
use std::fmt::Display;

use super::base_tree::{Leaf, Tree};

pub fn bst_insert<T: Display + PartialOrd + Copy>(
    tree: &mut Tree<T>,
    mut new_leaf: Leaf<T>,
) -> Index {
    let mut cur_idx_option = tree.root;
    let mut left_side = false;

    while let Some(cur_leaf_idx) = cur_idx_option {
        let cur_leaf = &mut tree.nodes[cur_leaf_idx];

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
    let leaf_id = tree.nodes.insert(Box::new(new_leaf));

    // Parent node found, so we set it to the corresponding child node
    if let Some(parent_idx) = cur_idx_option {
        let parent = &mut tree.nodes[parent_idx];

        if left_side {
            parent.left = Some(leaf_id);
        } else {
            parent.right = Some(leaf_id);
        }
    } else {
        // No parent found, so this leaf must be the root
        tree.root = Some(leaf_id);
    }

    leaf_id
}

pub fn find_node_index<T: Display + PartialOrd + Copy>(tree: &Tree<T>, item: &T) -> Option<Index> {
    let mut cur_leaf_opt = tree.root;
    while let Some(node_idx) = cur_leaf_opt {
        let leaf = &tree.nodes[node_idx];

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
pub fn get_inorder_successor<T: Display + PartialOrd + Copy>(
    tree: &mut Tree<T>,
    leaf: &Leaf<T>,
) -> Option<Index> {
    // Start with greater node, return smallest node in this sub-tree
    let mut cur_leaf_opt = leaf.right;

    while let Some(cur_leaf_idx) = cur_leaf_opt {
        let cur_leaf = &tree.nodes[cur_leaf_idx];

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
pub fn bst_remove_leaf<T: Display + PartialOrd + Copy>(tree: &mut Tree<T>, leaf_idx: Index) {
    let removed_leaf = tree
        .nodes
        .remove(leaf_idx)
        .expect("exclusive access during mutation guarantees that a node exists for each index");

    let mut replacement_leaf_opt = None;
    match (removed_leaf.left, removed_leaf.right) {
        // Only one child exists, so we swap the parent with the child
        (Some(solo_child_idx), None) | (None, Some(solo_child_idx)) => {
            replacement_leaf_opt = Some(solo_child_idx);
        }
        // Both children exist, so we must find the inorder successor first
        (Some(left_child_idx), Some(right_child_idx)) => {
            replacement_leaf_opt = get_inorder_successor(tree, &removed_leaf);

            // Replace node with its inorder successor
            if let Some(successor_idx) = replacement_leaf_opt {
                // Set inorder successor as the new parent node of the removed node's children
                let left_leaf = &mut tree.nodes[left_child_idx];
                left_leaf.parent = replacement_leaf_opt;

                let right_leaf = &mut tree.nodes[right_child_idx];
                right_leaf.parent = replacement_leaf_opt;

                let successor_leaf = &mut tree.nodes[successor_idx];
                successor_leaf.left = removed_leaf.left;
                successor_leaf.right = removed_leaf.right;

                let successor_parent_opt = successor_leaf.parent;

                // If successor's parent exists and is not the node being
                //   removed, we remove the successor node from it
                if let (Some(parent_idx), true) =
                    (successor_parent_opt, successor_parent_opt != Some(leaf_idx))
                {
                    let successor_parent_leaf = &mut tree.nodes[parent_idx];

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
        let parent_leaf = &mut tree.nodes[parent_idx];
        if parent_leaf.left == Some(leaf_idx) {
            parent_leaf.left = replacement_leaf_opt;
        } else {
            parent_leaf.right = replacement_leaf_opt;
        }
    }

    // Update replacement node with new parent node
    if let Some(replacement_node_idx) = replacement_leaf_opt {
        let replacement_node = &mut tree.nodes[replacement_node_idx];
        replacement_node.parent = removed_leaf.parent;
    }

    // if the removed node was the root, replace it with the replacement node
    if tree.root == Some(leaf_idx) {
        tree.root = replacement_leaf_opt;
    }
    tree.size -= 1;
}
