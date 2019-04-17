extern crate generational_arena;

use generational_arena::{Arena, Index};

pub struct RedBlackTree<T: std::cmp::PartialOrd> {
    root: Option<Index>,
    size: u64,
    nodes: Arena<Node<T>>
}

pub(crate) struct Leaf<T> {
    data: T,
    color: TreeColors,
    left: Option<Index>,
    right: Option<Index>,
    parent: Option<Index>,
}

pub(crate) type Node<T> = Box<Leaf<T>>;

pub(crate) enum TreeColors {
    Red,
    Black,
}

impl<T: std::cmp::PartialOrd> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree {
            root: None,
            size: 0,
            nodes : Arena::new()
        }
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn insert(&mut self, val: T) {
        let mut new_leaf = Leaf {
            data: val,
            color: TreeColors::Red,
            left: None,
            right: None,
            parent: None,
        };

        self.bst_insert(new_leaf);
        self.size += 1;
        self.recolor_tree();
    }

    fn bst_insert(&mut self, mut new_leaf: Leaf<T>) {
        let mut cur_idx_option = self.root;
        let mut right_side = false;

        while let Some(cur_leaf_idx) = cur_idx_option {
            let cur_leaf_option = self.nodes.get_mut(cur_leaf_idx);

            match cur_leaf_option {
                None => panic!("Attempted to reference a node which no longer exists"),
                Some(cur_leaf) => {
                    right_side = cur_leaf.data > new_leaf.data;

                    let next_leaf_idx_option = if right_side {
                        &mut cur_leaf.right
                    } else {
                        &mut cur_leaf.left
                    };

                    match next_leaf_idx_option {
                        None => {
                            break;
                        },
                        _ => cur_idx_option = *next_leaf_idx_option
                    }
                }
            }
        }

        new_leaf.parent = cur_idx_option;
        let leaf_id = self.nodes.insert(Box::new(new_leaf));
        if let Some(parent_idx) = cur_idx_option {
            let parent = self.nodes.get_mut(parent_idx).unwrap();

            if right_side {
                parent.right = Some(leaf_id);
            } else {
                parent.left = Some(leaf_id);
            }
        } else {
            self.root = Some(leaf_id);
        }
    }

    fn recolor_tree(&mut self) {
        let _starting_node = self.root.as_mut();
    }

    pub fn iter(&mut self) -> Iter<T> {
        let mut leaf_idx_stack = Vec::new();

        if let Some(root_idx) = self.root {
            leaf_idx_stack.push(root_idx);
        }

        Iter {
            leaf_idx_stack,
            nodes : &self.nodes
        }
    }
}

pub struct Iter<'a, T> {
    leaf_idx_stack: Vec<Index>,
    nodes: &'a Arena<Node<T>>
}

impl<'a, T: std::cmp::PartialOrd> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.leaf_idx_stack.pop().map(|leaf_idx| {
            let cur_leaf_option = self.nodes.get(leaf_idx);
            match cur_leaf_option {
                Some(cur_leaf) => {
                    if let Some(left_leaf_idx) = cur_leaf.left {
                        self.leaf_idx_stack.push(left_leaf_idx);
                    }

                    if let Some(right_leaf_idx) = cur_leaf.right {
                        self.leaf_idx_stack.push(right_leaf_idx);
                    }
                    &cur_leaf.data
                },
                None => {
                    panic!("Your tree is fucked. FIX IT")
                }
            }
        })
    }
}
