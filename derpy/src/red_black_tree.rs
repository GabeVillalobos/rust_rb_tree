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
        self.recolor_nodes(new_leaf_idx.unwrap());
    }

    fn recolor_nodes(&mut self, node_idx: Index) {
        // If node is root, color it black then return. Tree has been recolored successfully
        if self.bst.root == Some(node_idx) {
            self.set_node_color(Some(node_idx), TreeColors::Black);
            return;
        }

        let mut parent_idx_opt = self.bst.nodes[node_idx].parent;

        // Parent or current node color is black, which means that the
        //  RB-property has been upheld
        if self.get_node_color(parent_idx_opt) == TreeColors::Black
            || self.get_node_color(Some(node_idx)) == TreeColors::Black
        {
            return;
        }

        let parent_idx = parent_idx_opt.unwrap();
        let grandparent_idx_opt = self.bst.nodes[parent_idx].parent;
        let grandparent_idx = grandparent_idx_opt
            .expect("Proper tree coloring ensures that a grandparent exists when checking uncle node colors");

        let grandparent_node = &self.bst.nodes[grandparent_idx];

        let mut parent_is_left_child = false;
        let uncle_idx_opt = if grandparent_node.left == parent_idx_opt {
            parent_is_left_child = true;
            grandparent_node.right
        } else {
            grandparent_node.left
        };

        match self.get_node_color(uncle_idx_opt) {
            TreeColors::Red => {
                // Simplest case: 'push' black color from grandparent down to its children
                self.set_node_color(grandparent_idx_opt, TreeColors::Red);
                self.set_node_color(uncle_idx_opt, TreeColors::Black);
                self.set_node_color(parent_idx_opt, TreeColors::Black);
            }
            TreeColors::Black => {
                let node_is_left_child =
                    self.bst.nodes[parent_idx_opt.unwrap()].left == Some(node_idx);
                // 4 possible cases here:
                match (parent_is_left_child, node_is_left_child) {
                    // 1: parent is left child, node is left child
                    (true, true) => {
                        self.rotate_node_right(parent_idx);
                    }
                    // 2: parent is left child, node is right child
                    (true, false) => {
                        self.rotate_node_left(node_idx);
                        self.rotate_node_right(node_idx);
                        parent_idx_opt = Some(node_idx);
                    }
                    // 3: mirror of 2
                    (false, true) => {
                        self.rotate_node_right(node_idx);
                        self.rotate_node_left(node_idx);
                        parent_idx_opt = Some(node_idx);
                    }
                    // 5: mirror of 1
                    (false, false) => {
                        self.rotate_node_left(parent_idx);
                    }
                }

                // Swap grandparent & parent colors, and we're done!
                let grandparent_color = self.get_node_color(grandparent_idx_opt);
                let parent_color = self.get_node_color(parent_idx_opt);

                self.set_node_color(parent_idx_opt, grandparent_color);
                self.set_node_color(grandparent_idx_opt, parent_color);
            }
        };

        if let Some(grandparent_idx) = grandparent_idx_opt {
            self.recolor_nodes(grandparent_idx);
        }
    }

    // When rotating a node right, we set the parent's left child equal to the
    //  target node's right child. Then we set the parent as the right child
    //  of the target. Finally we fix all of the parent references, et voila.
    fn rotate_node_right(&mut self, node_idx: Index) {
        let node = &mut self.bst.nodes[node_idx];
        let parent_idx = node.parent.expect(
            "Proper tree structure ensures that a rotation occurs only on nodes with parents",
        );

        // Make parent node a child of the rotating node
        let right_node_idx_opt = node.right;
        node.right = Some(parent_idx);

        // Make parent node a child of the rotating node
        let parent_node = &mut self.bst.nodes[parent_idx];
        let grandparent_idx_opt = parent_node.parent;

        // Update parent's 'parent' and 'left' children accordingly
        parent_node.parent = Some(node_idx);
        parent_node.left = right_node_idx_opt;

        // Update right child node with new parent idx
        if let Some(right_node_idx) = right_node_idx_opt {
            let right_node = &mut self.bst.nodes[right_node_idx];
            right_node.parent = Some(parent_idx);
        }

        // Set node's parent to the grandparent
        self.bst.nodes[node_idx].parent = grandparent_idx_opt;

        // If grandparent exists, we replace its child idx with the parent with the target node
        if let Some(grandparent_idx) = grandparent_idx_opt {
            let grandparent_node = &mut self.bst.nodes[grandparent_idx];
            if grandparent_node.left == Some(parent_idx) {
                grandparent_node.left = Some(node_idx);
            } else {
                grandparent_node.right = Some(node_idx);
            }
        } else {
            self.bst.root = Some(node_idx);
        }
    }

    // When rotating a node left, we set the parent's right child equal to the
    //  target node's left child. Then we set the parent as the left child
    //  of the target. Finally we fix all of the parent references, et voila.
    fn rotate_node_left(&mut self, node_idx: Index) {
        let node = &mut self.bst.nodes[node_idx];
        let parent_idx = node.parent.expect(
            "Proper tree structure ensures that a rotation occurs only on nodes with parents",
        );

        let left_node_idx_opt = node.left;
        node.left = Some(parent_idx);

        let parent_node = &mut self.bst.nodes[parent_idx];
        let grandparent_idx_opt = parent_node.parent;

        // Update parent's 'parent' and 'left' children accordingly
        parent_node.parent = Some(node_idx);
        parent_node.right = left_node_idx_opt;

        // If rotating node had a left child, update its parent index
        if let Some(left_node_idx) = left_node_idx_opt {
            let left_node = &mut self.bst.nodes[left_node_idx];
            left_node.parent = Some(parent_idx);
        }

        // Set node's parent to the grandparent
        self.bst.nodes[node_idx].parent = grandparent_idx_opt;

        // If grandparent exists, we replace its child idx with the parent with the target node
        if let Some(grandparent_idx) = grandparent_idx_opt {
            let grandparent_node = &mut self.bst.nodes[grandparent_idx];
            if grandparent_node.left == Some(parent_idx) {
                grandparent_node.left = Some(node_idx);
            } else {
                grandparent_node.right = Some(node_idx);
            }
        } else {
            self.bst.root = Some(node_idx);
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

    pub fn contains(&self, item: &T) -> bool {
        self.bst.find_node_index(item).is_some()
    }

    pub fn remove(&mut self, item: &T) -> Result<(), NodeNotFoundErr> {
        unimplemented!();
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

    // Private method for printing node diagnostic data
    fn node_to_str(&self, node_idx: Index) -> String {
        let node = &self.bst.nodes[node_idx];
        format!(
            "I: {:?}, C:{:?}, Data: {}, L: {:?}, R: {:?}, P: {:?}",
            node_idx,
            self.colors.get(&node_idx),
            node.data,
            node.left,
            node.right,
            node.parent
        )
    }
}

impl<T: PartialOrd + Display + Default> Display for RedBlackTree<T> {
    // Simple BFS traversing method that prints each node's information for diagnostic purposes
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut nodes = VecDeque::new();
        let mut node_strs = vec![];

        nodes.push_front(self.bst.root);
        while let Some(node_idx_opt) = nodes.pop_front() {
            let node_idx = node_idx_opt.unwrap();
            node_strs.push(self.node_to_str(node_idx));
            let node = &self.bst.nodes[node_idx];

            if node.left.is_some() {
                nodes.push_back(node.left);
            }

            if node.right.is_some() {
                nodes.push_back(node.right);
            }
        }

        write!(f, "{}", node_strs.join("\n"))
    }
}
