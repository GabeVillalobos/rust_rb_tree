use generational_arena::Index;
use std::collections::{HashMap, VecDeque};

use super::base_tree::{BfsIter, DfsIter, InternalBinarySearchTree, Node};
use super::tree_errs::NodeNotFoundErr;

use std::cmp::PartialOrd;
use std::fmt::Display;

#[derive(Default)]
pub struct RedBlackTree<T: PartialOrd + Display + Default> {
    bst: InternalBinarySearchTree<T>,
    colors: HashMap<Index, TreeColors>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum TreeColors {
    Red,
    Black,
}

impl Default for TreeColors {
    fn default() -> TreeColors {
        TreeColors::Red
    }
}

impl<T: PartialOrd + Display + Default> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree {
            bst: InternalBinarySearchTree::new(),
            colors: HashMap::new(),
        }
    }

    pub fn get_size(&self) -> usize {
        self.bst.nodes.len()
    }

    pub fn insert(&mut self, item: T) {
        let leaf = Node {
            data: item,
            left: None,
            right: None,
            parent: None,
        };

        let new_leaf_idx = Some(self.bst.insert_node(leaf));
        self.set_node_color(new_leaf_idx, TreeColors::Red);
        self.recolor_nodes(new_leaf_idx);
    }

    fn recolor_nodes(&mut self, node_idx_opt: Option<Index>) {
        if node_idx_opt.is_none() {
            return;
        }

        // if node is root, color it black then return. Tree has been recolors successfully
        if self.bst.root == node_idx_opt {
            self.set_node_color(node_idx_opt, TreeColors::Black);
            return;
        }

        let parent_idx_opt = self.bst.nodes[node_idx_opt.unwrap()].parent;

        // parent color is black, no need to continue
        if self.get_node_color(parent_idx_opt) == TreeColors::Black {
            return;
        }

        let mut grandparent_idx_opt = None;
        let mut uncle_idx_opt = None;
        let mut parent_is_left_child = false;

        if let Some(parent_idx) = parent_idx_opt {
            let gramps = &self.bst.nodes[parent_idx];
            grandparent_idx_opt = gramps.parent;

            uncle_idx_opt = if gramps.left == parent_idx_opt {
                parent_is_left_child = true;
                gramps.right
            } else {
                gramps.left
            }
        }

        match self.get_node_color(uncle_idx_opt) {
            TreeColors::Red => {
                // simplest case: recolor and recurse, coloring the grandparent
                self.set_node_color(grandparent_idx_opt, TreeColors::Red);
                self.set_node_color(uncle_idx_opt, TreeColors::Black);
                self.set_node_color(parent_idx_opt, TreeColors::Black);
                self.recolor_nodes(grandparent_idx_opt);
            }
            TreeColors::Black => {
                // grandparent doesn't exist, so we're done
                if grandparent_idx_opt.is_none() {
                    return;
                }

                let node_is_left_child =
                    self.bst.nodes[parent_idx_opt.unwrap()].left == node_idx_opt;
                // 4 possible cases here:
                match (parent_is_left_child, node_is_left_child) {
                    // 1: parent is left child, node is left child
                    (true, true) => {
                        self.rotate_node_right(parent_idx_opt.unwrap());
                    }
                    // 2: parent is left child, node is right child
                    (true, false) => {
                        self.rotate_node_left(node_idx_opt.unwrap());
                        self.rotate_node_right(parent_idx_opt.unwrap());
                    }
                    // 3: mirror of 1
                    (false, true) => {
                        self.rotate_node_left(parent_idx_opt.unwrap());
                    }
                    // 5: mirror of 2
                    (false, false) => {
                        self.rotate_node_right(node_idx_opt.unwrap());
                        self.rotate_node_left(parent_idx_opt.unwrap());
                    }
                }

                // Swap grandparent & parent colors, and we're done!
                let grandparent_color = self.get_node_color(grandparent_idx_opt);
                let parent_color = self.get_node_color(parent_idx_opt);

                self.set_node_color(parent_idx_opt, grandparent_color);
                self.set_node_color(grandparent_idx_opt, parent_color);
            }
        };
    }

    // When rotating a node, we set the parent's right child equal to the
    //  target node's left child. Then we set the parent as the left child
    //  of the target. Finally we fix all of the parent references, et voila.
    fn rotate_node_right(&mut self, node_idx: Index) {
        let node = &mut self.bst.nodes[node_idx];
        let parent_idx = node
            .parent
            .expect("Proper tree structure ensures that a occurs only on nodes with parents");
        let left_node_idx_opt = node.left;
        node.left = Some(parent_idx);

        let parent_node = &mut self.bst.nodes[parent_idx];
        parent_node.parent = Some(node_idx);
        parent_node.right = left_node_idx_opt;

        let grandparent_idx_opt = parent_node.parent;
        self.bst.nodes[node_idx].parent = grandparent_idx_opt;

        // If grandparent exists, we replace its child idx with the parent with the target node
        if let Some(grandparent_idx) = grandparent_idx_opt {
            let grandparent_node = &mut self.bst.nodes[grandparent_idx];
            if grandparent_node.left == Some(parent_idx) {
                grandparent_node.left = Some(node_idx);
            } else {
                grandparent_node.right = Some(node_idx);
            }
        }
    }

    fn rotate_node_left(&mut self, node_idx: Index) {
        let node = &mut self.bst.nodes[node_idx];
        let parent_idx = node
            .parent
            .expect("Proper tree structure ensures that a occurs only on nodes with parents");

        let right_node_idx_opt = node.right;
        node.right = Some(parent_idx);

        let parent_node = &mut self.bst.nodes[parent_idx];
        parent_node.parent = Some(node_idx);
        parent_node.left = right_node_idx_opt;

        let grandparent_idx_opt = parent_node.parent;
        self.bst.nodes[node_idx].parent = grandparent_idx_opt;

        // If grandparent exists, we replace its child idx with the parent with the target node
        if let Some(grandparent_idx) = grandparent_idx_opt {
            let grandparent_node = &mut self.bst.nodes[grandparent_idx];
            if grandparent_node.left == Some(parent_idx) {
                grandparent_node.left = Some(node_idx);
            } else {
                grandparent_node.right = Some(node_idx);
            }
        }
    }

    fn get_node_color(&self, node_opt: Option<Index>) -> TreeColors {
        if let Some(node_idx) = node_opt {
            self.colors[&node_idx]
        } else {
            // Terminating nodes are black in color by default
            TreeColors::Black
        }
    }

    fn set_node_color(&mut self, node_idx_opt: Option<Index>, color: TreeColors) {
        if let Some(node_idx) = node_idx_opt {
            self.colors.insert(node_idx, color);
        } else if color == TreeColors::Red {
            panic!(
                "Proper tree structure ensures that recoloring a terminating node red cannot occur"
            );
        }
        // Terminating nodes are already black, so do nothing
    }

    pub fn remove(&mut self, item: &T) -> Result<(), NodeNotFoundErr> {
        let node_idx_to_remove = self.bst.find_node_index(item).ok_or(NodeNotFoundErr)?;
        self.bst.remove_node(node_idx_to_remove);
        Ok(())
    }

    // Create a new iterator w/ a stack for DFS taversal
    pub fn dfs_iter(&mut self) -> DfsIter<T> {
        let mut node_idx_stack = Vec::new();

        if let Some(root_idx) = self.bst.root {
            node_idx_stack.push(root_idx);
        }

        DfsIter {
            node_idx_stack,
            nodes: &self.bst.nodes,
        }
    }

    // Create a new iterator w/ a queue for BFS traversal
    pub fn bfs_iter(&mut self) -> BfsIter<T> {
        let mut node_idx_queue = VecDeque::new();

        if let Some(root_idx) = self.bst.root {
            node_idx_queue.push_front(root_idx);
        }

        BfsIter {
            node_idx_queue,
            nodes: &self.bst.nodes,
        }
    }
}
