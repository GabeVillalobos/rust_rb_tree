use generational_arena::Index;
use std::fmt::Display;

use super::base_tree::{Node, Tree};

pub fn insert_node<T: Display + PartialOrd + Copy>(
    tree: &mut Tree<T>,
    mut new_leaf: Node<T>,
) -> Index {
    let mut cur_idx_option = tree.root;
    let mut left_side = false;

    while let Some(cur_node_idx) = cur_idx_option {
        let cur_node = &mut tree.nodes[cur_node_idx];

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
    tree: &Tree<T>,
    node: &Node<T>,
) -> Option<Index> {
    // Start with greater node, return smallest node in this sub-tree
    let mut cur_node_opt = node.right;

    while let Some(cur_node_idx) = cur_node_opt {
        let cur_node = &tree.nodes[cur_node_idx];

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
pub fn remove_node<T: Display + PartialOrd + Copy>(tree: &mut Tree<T>, node_idx: Index) {
    let mut replacement_idx_opt = None;
    let mut recurisve_remove = false;
    
    let node_to_remove = &tree.nodes[node_idx];
    // Grab the parent index, in case we need it later
    let parent_opt = node_to_remove.parent;

    match (node_to_remove.left, node_to_remove.right) {
        // Only one child exists, so we swap the parent with the child
        (Some(solo_child_idx), None) | (None, Some(solo_child_idx)) => {
            replacement_idx_opt = Some(solo_child_idx);
        }
        // Both children exist, so we must find the inorder successor first
        (Some(_), Some(_)) => {
            let inorder_successor = get_inorder_successor(tree, &node_to_remove);

            let successor_idx = inorder_successor
                .expect("Proper tree structure ensures that every node with children has an inorder successor");

            // Swap value of node to be deleted with its inorder successor's
            let successor_data = tree.nodes[successor_idx].data;
            let old_node = &mut tree.nodes[node_idx];
            old_node.data = successor_data;

            recurisve_remove = true;

            remove_node(tree, successor_idx);
        }

        // If it's a leaf, just remove the reference from it's parent and delete the node
        (None, None) => {}
    }

    // Handles removing leaves & relinking leaves with a single child
    if !recurisve_remove {
        if let Some(parent_idx) = parent_opt {
            let parent_node = &mut tree.nodes[parent_idx];

            if parent_node.left == Some(node_idx) {
                parent_node.left = replacement_idx_opt;
            } else {
                parent_node.right = replacement_idx_opt;
            }
        } else {
            // It's the root node, and we nuke the world
            tree.root = None;
        }

        if let Some(replacement_idx) = replacement_idx_opt {
            let replacement_node = &mut tree.nodes[replacement_idx];
            replacement_node.parent = parent_opt;
        }

        // Finally, remove the node from the arena itself
        let _ = tree
            .nodes
            .remove(node_idx)
            .expect("Exclusive access during mutation ensures that a node exists for every index");

        tree.size -= 1;
    }
}
