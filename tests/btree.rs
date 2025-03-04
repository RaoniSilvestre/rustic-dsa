#[cfg(test)]
mod tests {
    use rustic_dsa::data_structures::btree::{BTree, Key};

    #[test]
    fn test_insert_and_find() {
        let mut tree = BTree::new(2);

        let k1 = Key::new_from_key(10);
        let k2 = Key::new_from_key(20);
        let k3 = Key::new_from_key(5);
        let k4 = Key::new_from_key(6);
        let k5 = Key::new_from_key(15);

        tree.insert(k1.clone());
        tree.insert(k2.clone());
        tree.insert(k3.clone());
        tree.insert(k4.clone());
        tree.insert(k5.clone());

        assert_eq!(tree.find(10), Some(k1));
        assert_eq!(tree.find(15), Some(k5));
        assert_eq!(tree.find(5), Some(k3));
        assert_eq!(tree.find(100), None);
    }

    #[test]
    fn test_remove_leaf_without_underflow() {
        let mut tree = BTree::new(2);

        let k1 = Key::new_from_key(10);
        let k2 = Key::new_from_key(20);
        let k3 = Key::new_from_key(30);

        tree.insert(k1.clone());
        tree.insert(k2.clone());
        tree.insert(k3.clone());

        tree.remove(20); // Remove um nó folha sem necessidade de balanceamento

        assert_eq!(tree.find(20), None);
        assert_eq!(tree.find(10), Some(k1));
        assert_eq!(tree.find(30), Some(k3));
    }

    #[test]
    fn test_remove_leaf_with_underflow_and_borrow() {
        let mut tree = BTree::new(2);

        let k1 = Key::new_from_key(10);
        let k2 = Key::new_from_key(20);
        let k3 = Key::new_from_key(30);
        let k4 = Key::new_from_key(40);

        tree.insert(k1.clone());
        tree.insert(k2.clone());
        tree.insert(k3.clone());
        tree.insert(k4.clone());

        tree.remove(30); // Agora a folha pode precisar emprestar do irmão

        assert_eq!(tree.find(30), None);
        assert_eq!(tree.find(10), Some(k1));
        assert_eq!(tree.find(20), Some(k2));
        assert_eq!(tree.find(40), Some(k4));
    }

    #[test]
    fn test_remove_leaf_with_fusion() {
        let mut tree = BTree::new(2);

        let k1 = Key::new_from_key(10);
        let k2 = Key::new_from_key(20);
        let k3 = Key::new_from_key(30);
        let k4 = Key::new_from_key(40);
        let k5 = Key::new_from_key(50);

        tree.insert(k1.clone());
        tree.insert(k2.clone());
        tree.insert(k3.clone());
        tree.insert(k4.clone());
        tree.insert(k5.clone());

        tree.remove(40);
        tree.remove(50); // Pode causar fusão dos nós

        assert_eq!(tree.find(40), None);
        assert_eq!(tree.find(50), None);
        assert_eq!(tree.find(10), Some(k1));
        assert_eq!(tree.find(20), Some(k2));
        assert_eq!(tree.find(30), Some(k3));
    }

    #[test]
    fn test_remove_internal_node() {
        let mut tree = BTree::new(2);

        let k1 = Key::new_from_key(10);
        let k2 = Key::new_from_key(20);
        let k3 = Key::new_from_key(30);
        let k4 = Key::new_from_key(40);
        let k5 = Key::new_from_key(50);
        let k6 = Key::new_from_key(60);

        tree.insert(k1.clone());
        tree.insert(k2.clone());
        tree.insert(k3.clone());
        tree.insert(k4.clone());
        tree.insert(k5.clone());
        tree.insert(k6.clone());

        tree.remove(30); // Remove um nó interno e precisa substituir pelo predecessor ou sucessor

        assert_eq!(tree.find(30), None);
        assert!(tree.find(20) == Some(k2) || tree.find(40) == Some(k4)); // Verifica que o substituto está correto
    }

    #[test]
    fn test_remove_root_replacement() {
        let mut tree = BTree::new(2);

        let k1 = Key::new_from_key(10);
        let k2 = Key::new_from_key(20);
        let k3 = Key::new_from_key(30);
        let k4 = Key::new_from_key(40);
        let k5 = Key::new_from_key(50);
        let k6 = Key::new_from_key(60);
        let k7 = Key::new_from_key(70);
        let k8 = Key::new_from_key(80);

        tree.insert(k1.clone());
        tree.insert(k2.clone());
        tree.insert(k3.clone());
        tree.insert(k4.clone());
        tree.insert(k5.clone());
        tree.insert(k6.clone());
        tree.insert(k7.clone());
        tree.insert(k8.clone());

        tree.remove(50);
        tree.remove(60);
        tree.remove(70);
        tree.remove(80);

        tree.remove(30);

        assert_eq!(tree.find(30), None);
        assert!(tree.find(40) == Some(k4));
    }
}
