#[cfg(test)]
mod binary_tree_tests {
    use rustic_dsa::data_structures::bin_tree::BinaryTree;

    #[test]
    fn test_insert() {
        let mut root = BinaryTree::default();

        root.insert(5);
        root.insert(10);
        root.insert(15);
        root.insert(2);
        root.insert(7);
        root.insert(12);
        root.insert(17);

        assert!(root.search(5));
        assert!(root.search(15));
        assert!(root.search(2));
        assert!(root.search(7));
        assert!(root.search(12));
        assert!(root.search(17));
        assert!(!root.search(100));
    }

    #[test]
    fn test_search_existing_node() {
        let mut root = BinaryTree::default();
        root.insert(5);
        root.insert(10);
        root.insert(15);

        assert!(root.search(5));
        assert!(root.search(10));
        assert!(root.search(15));
    }

    #[test]
    fn test_search_non_existing_node() {
        let mut root = BinaryTree::default();
        root.insert(5);
        root.insert(10);
        root.insert(15);

        assert!(!root.search(20));
        assert!(!root.search(100));
    }

    #[test]
    fn test_remove_leaf() {
        let mut root = BinaryTree::default();
        root.insert(5);
        root.insert(10);
        root.insert(15);

        root.remove(5);
        assert!(!root.search(5));
    }

    #[test]
    fn test_remove_node_with_one_child() {
        let mut root = BinaryTree::default();
        root.insert(5);
        root.insert(15);
        root.insert(10);

        root.remove(15);

        assert!(!root.search(15));
    }

    #[test]
    fn test_remove_node_with_two_children() {
        let mut root = BinaryTree::default();
        root.insert(5);
        root.insert(15);
        root.insert(12);
        root.insert(10);

        root.remove(10);

        assert!(root.search(12));
        assert!(!root.search(10));
    }

    #[test]
    fn test_remove_root_with_one_child() {
        let mut root = BinaryTree::default();
        root.insert(15);
        root.insert(10);

        root.remove(10);

        assert!(root.search(15));
        assert!(!root.search(10));
    }

    #[test]
    fn test_remove_root_with_no_children() {
        let root = BinaryTree::default();

        assert!(!root.search(10));
    }

    #[test]
    fn test_remove_non_existing_node() {
        let mut root = BinaryTree::default();
        root.insert(5);
        root.insert(15);

        root.remove(2);
    }
}
