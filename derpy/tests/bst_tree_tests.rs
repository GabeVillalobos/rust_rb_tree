extern crate derpy;
use derpy::binary_search_tree::BinarySearchTree;

#[test]
fn test_instantiation() {
    let _b_tree: BinarySearchTree<i32> = BinarySearchTree::new();
}

#[test]
fn test_insert_root_node() {
    let mut b_tree = BinarySearchTree::new();
    b_tree.insert(42);
}

#[test]
fn test_get_size_after_instert() {
    let mut b_tree = BinarySearchTree::new();
    b_tree.insert(55);
    assert_eq!(b_tree.get_size(), 1);

    b_tree.insert(55);
    assert_eq!(b_tree.get_size(), 2);
}

#[test]
fn test_iterator() {
    let mut b_tree = BinarySearchTree::new();
    b_tree.insert(55);
    b_tree.insert(42);
    b_tree.insert(44);

    let mut it = b_tree.dfs_iter();
    assert_eq!(*it.next().unwrap(), 55);
    assert_eq!(*it.next().unwrap(), 42);
    assert_eq!(*it.next().unwrap(), 44);
    assert_eq!(it.next(), None);
}

#[test]
fn bst_insertion() {
    let mut b_tree = BinarySearchTree::new();
    b_tree.insert(55);
    b_tree.insert(60);
    b_tree.insert(25);
    b_tree.insert(12);
    b_tree.insert(66);
    b_tree.insert(55);
    b_tree.insert(54);

    let expected_order = vec![55, 25, 12, 54, 60, 55, 66];
    let mut idx = 0;
    let mut b_tree_iter = b_tree.dfs_iter();
    while let Some(leaf) = b_tree_iter.next() {
        assert_eq!(leaf, expected_order.get(idx).unwrap());
        idx = idx + 1;
    }
}

#[test]
fn bfs_traversal() {
    let mut b_tree = BinarySearchTree::new();
    b_tree.insert(55);
    b_tree.insert(60);
    b_tree.insert(25);
    b_tree.insert(12);
    b_tree.insert(66);
    b_tree.insert(55);
    b_tree.insert(54);

    let expected_order = vec![55, 25, 60, 12, 54, 55, 66];
    let mut idx = 0;
    let mut b_tree_iter = b_tree.bfs_iter();
    while let Some(leaf) = b_tree_iter.next() {
        assert_eq!(leaf, expected_order.get(idx).unwrap());
        idx = idx + 1;
    }
}

