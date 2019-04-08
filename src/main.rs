struct RedBlackTree<T> {
    root : Node<T>
}

struct Leaf<T> {
    data: T,
    color: TreeColors,
    left: Node<T>,
    right: Node<T>,
    parent: Node<T>
}

type Node<T> = Option<Box<Leaf<T>>>;
enum TreeColors {
    Red,
    Black
}

impl<T> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree{ root: None }
    }

    pub fn insert(&mut self, val: T) {
        let mut new_node = Box::new(Leaf {
            data: val,
            color: TreeColors::Red,
            left: None,
            right: None,
            parent: None
        });

        match self.root.take() {
            None => {
                new_node.color = TreeColors::Black;
                self.root = Some(new_node);
            },
            Some(_node) => {

            }
        }
    }

    fn insertRecursive(&mut self, node: Node<T>) {
        // Check parent color
        // If Black return yasssss
        // If Red, we have to go deeper
    }
}

fn main() {
    let mut my_box = RedBlackTree::new() ;
    my_box.insert(55);
}