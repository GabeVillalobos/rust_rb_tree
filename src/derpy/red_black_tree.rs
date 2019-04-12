#[derive(Default)]
pub struct RedBlackTree<T: std::cmp::PartialOrd> {
    root: Node<T>,
    size: u64,
}

pub(crate) struct Leaf<T> {
    data: T,
    color: TreeColors,
    left: Node<T>,
    right: Node<T>,
    parent: Node<T>,
}

pub(crate) type Node<T> = Option<Box<Leaf<T>>>;

pub(crate) enum TreeColors {
    Red,
    Black,
}

impl<T: std::cmp::PartialOrd> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree {
            root: None,
            size: 0,
        }
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn insert(&mut self, val: T) {
        let new_leaf = Leaf {
            data: val,
            color: TreeColors::Red,
            left: None,
            right: None,
            parent: None,
        };

        self.bst_insert(new_leaf);
        self.size += 1;
        self.recolor_tree();
        // recolor step
    }

    fn bst_insert(&mut self, new_leaf: Leaf<T>) {
        let mut cur_node = self.root.as_mut();

        match cur_node {
            None => self.root = Some(Box::new(new_leaf)),
            _ => {
                while let Some(cur_leaf) = cur_node {
                    let right_side = cur_leaf.data > new_leaf.data;

                    let next_node = if right_side {
                        &mut cur_leaf.right
                    } else {
                        &mut cur_leaf.left
                    };

                    match next_node {
                        None => {
                            *next_node = Some(Box::new(new_leaf));
                            break;
                        }
                        _ => cur_node = next_node.as_mut(),
                    }
                }
            }
        }
    }

    fn recolor_tree(&mut self) {
        let _starting_node = self.root.as_mut();
    }

    pub fn iter(&mut self) -> Iter<T> {
        let mut leaf_stack = Vec::new();

        if let Some(leaf_box) = &self.root {
            leaf_stack.push(leaf_box.as_ref());
        }

        Iter { leaf_stack }
    }
}

pub struct Iter<'a, T> {
    leaf_stack: Vec<&'a Leaf<T>>,
}

impl<'a, T: std::cmp::PartialOrd> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.leaf_stack.pop().map(|leaf| {
            if let Some(left_leaf) = &leaf.left {
                self.leaf_stack.push(left_leaf.as_ref())
            }

            if let Some(right_leaf) = &leaf.right {
                self.leaf_stack.push(right_leaf.as_ref())
            }
            &leaf.data
        })
    }
}
