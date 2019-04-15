extern crate derpy_lib;
use derpy_lib::red_black_tree::RedBlackTree;

fn main() {
    let mut rb_tree = RedBlackTree::new();
    rb_tree.insert(42);
}