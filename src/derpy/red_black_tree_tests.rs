#[cfg(test)]
mod tests {
  use super::super::red_black_tree::RedBlackTree;

  #[test]
  fn test_instantiation() {
    let rb_tree : RedBlackTree<i32> = RedBlackTree::new();
  }

  #[test]
  fn test_insert_root_node() {
    let mut rb_tree = RedBlackTree::new();
    rb_tree.insert(42);
  }

  #[test]
  fn test_get_size_after_instert() {
    let mut rb_tree = RedBlackTree::new();
    rb_tree.insert(55);
    assert_eq!(rb_tree.getSize(), 1);

    rb_tree.insert(55);
    assert_eq!(rb_tree.getSize(), 2);
  }

  #[test]
  fn test_iterator() {
    let mut rb_tree = RedBlackTree::new();
    rb_tree.insert(55);
    rb_tree.insert(42);
    rb_tree.insert(44);

    let mut it = rb_tree.iter();
    assert_eq!(*it.next().unwrap(), 55);
    assert_eq!(*it.next().unwrap(), 42);
    assert_eq!(*it.next().unwrap(), 44);
    assert_eq!(it.next(), None);
  }
}