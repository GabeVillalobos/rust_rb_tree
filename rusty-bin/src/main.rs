extern crate derpy;
use derpy::red_black_tree::RedBlackTree;

fn main() {
    let mut rb_tree = RedBlackTree::new();
    rb_tree.insert(55);
    rb_tree.insert(60);
    rb_tree.insert(25);
    rb_tree.insert(12);
    rb_tree.insert(66);
    rb_tree.insert(55);
    rb_tree.insert(54);

    for leaf_val in rb_tree.dfs_iter() {
        println!("{}", leaf_val);
    }
}
