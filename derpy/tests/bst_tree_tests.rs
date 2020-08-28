extern crate derpy;
use derpy::binary_search_tree::BinarySearchTree;

fn verify_tree_bfs(bst: &mut BinarySearchTree<i32>, expected_vals: Vec<i32>) {
    assert!(
        bst.get_size() == expected_vals.len(),
        "Expected tree size to be {}, but it was {}",
        expected_vals.len(),
        bst.get_size()
    );

    let mut b_tree_iter = bst.bfs_iter();
    for val in expected_vals {
        let node = b_tree_iter.next();

        assert!(
            node == Some(&val),
            "Expected node to be {:?}, but it was {:?}",
            Some(&val),
            node
        );
    }
    assert_eq!(b_tree_iter.next(), None);
}

fn verify_tree_dfs(bst: &mut BinarySearchTree<i32>, expected_vals: Vec<i32>) {
    assert_eq!(bst.get_size(), expected_vals.len());
    let mut b_tree_iter = bst.dfs_iter();
    for val in expected_vals {
        assert_eq!(b_tree_iter.next(), Some(&val));
    }
    assert_eq!(b_tree_iter.next(), None);
}

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
fn test_get_size_after_insert() {
    let mut b_tree = BinarySearchTree::new();
    b_tree.insert(55);
    assert_eq!(b_tree.get_size(), 1);

    b_tree.insert(55);
    assert_eq!(b_tree.get_size(), 2);
}

#[test]
fn dfs_traversal() {
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
    verify_tree_dfs(&mut b_tree, expected_order);
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
    verify_tree_bfs(&mut b_tree, expected_order);
}

#[test]
fn find_items() {
    let mut b_tree = BinarySearchTree::new();
    b_tree.insert(55);
    b_tree.insert(42);
    b_tree.insert(44);
    b_tree.insert(88);
    b_tree.insert(66);

    assert_eq!(b_tree.contains(&55), true);
    assert_eq!(b_tree.contains(&42), true);
    assert_eq!(b_tree.contains(&66), true);
    assert_eq!(b_tree.contains(&44), true);
    assert_eq!(b_tree.contains(&88), true);
    assert_eq!(b_tree.contains(&99), false);
}

#[test]
fn remove_node() {
    let mut b_tree = BinarySearchTree::new();
    b_tree.insert(55);
    b_tree.insert(42);
    b_tree.insert(44);
    b_tree.insert(88);
    b_tree.insert(66);
    b_tree.insert(99);
    b_tree.insert(43);
    b_tree.insert(65);
    b_tree.insert(97);
    b_tree.insert(100);

    assert_eq!(b_tree.remove(&12).is_err(), true);

    println!("Remove a leaf node");
    assert_eq!(b_tree.remove(&43).is_ok(), true);

    let mut expected_order = vec![55, 42, 88, 44, 66, 99, 65, 97, 100];
    verify_tree_bfs(&mut b_tree, expected_order);

    println!("Remove a node with a single leaf");
    assert_eq!(b_tree.remove(&42).is_ok(), true);
    expected_order = vec![55, 44, 88, 66, 99, 65, 97, 100];
    verify_tree_bfs(&mut b_tree, expected_order);

    println!("Remove a node with multiple leaves");
    assert_eq!(b_tree.remove(&88).is_ok(), true);
    expected_order = vec![55, 44, 97, 66, 99, 65, 100];
    verify_tree_bfs(&mut b_tree, expected_order);

    println!("Remove root node with multiple leaves");
    assert_eq!(b_tree.remove(&55).is_ok(), true);
    expected_order = vec![65, 44, 97, 66, 99, 100];
    verify_tree_bfs(&mut b_tree, expected_order);

    println!("Remove node with 2 children, without an inorder successor");
    assert_eq!(b_tree.remove(&97).is_ok(), true);
    expected_order = vec![65, 44, 99, 66, 100];
    verify_tree_bfs(&mut b_tree, expected_order);

    println!("Remove node with inorder successor with right subtree");
    b_tree.insert(110);
    b_tree.insert(109);

    // Verify that the tree is as we expect before testing
    expected_order = vec![65, 44, 99, 66, 100, 110, 109];
    verify_tree_bfs(&mut b_tree, expected_order);

    assert_eq!(b_tree.remove(&99).is_ok(), true);
    expected_order = vec![65, 44, 100, 66, 110, 109];
    verify_tree_bfs(&mut b_tree, expected_order);
}

#[test]
fn remove_single_root() {
    let mut b_tree = BinarySearchTree::new();
    b_tree.insert(55);

    assert_eq!(b_tree.remove(&55).is_ok(), true);
    assert_eq!(b_tree.get_size(), 0);
    assert_eq!(b_tree.bfs_iter().next(), None);
}
