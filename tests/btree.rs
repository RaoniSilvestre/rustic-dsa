#[cfg(test)]
mod tests {
    use rustic_dsa::data_structures::btree::BTree;

    #[test]
    fn test_insert_and_find() {
        let mut tree = BTree::new(2);

        let k1 = 10;
        let k2 = 20;
        let k3 = 5;
        let k4 = 6;
        let k5 = 15;

        tree.insert(k1);
        tree.insert(k2);
        tree.insert(k3);
        tree.insert(k4);
        tree.insert(k5);

        assert_eq!(tree.find(10), Some(k1));
        assert_eq!(tree.find(15), Some(k5));
        assert_eq!(tree.find(5), Some(k3));
        assert_eq!(tree.find(100), None);
    }

    #[test]
    fn test_remove_leaf_without_underflow() {
        let mut tree = BTree::new(2);

        let k1 = 10;
        let k2 = 20;
        let k3 = 30;

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

        let k1 = 10;
        let k2 = 20;
        let k3 = 30;
        let k4 = 40;

        tree.insert(k1);
        tree.insert(k2);
        tree.insert(k3);
        tree.insert(k4);

        tree.remove(30); // Agora a folha pode precisar emprestar do irmão

        assert_eq!(tree.find(30), None);
        assert_eq!(tree.find(10), Some(k1));
        assert_eq!(tree.find(20), Some(k2));
        assert_eq!(tree.find(40), Some(k4));
    }

    #[test]
    fn test_remove_leaf_with_fusion() {
        let mut tree = BTree::new(2);

        let k1 = 10;
        let k2 = 20;
        let k3 = 30;
        let k4 = 40;
        let k5 = 50;

        tree.insert(k1);
        tree.insert(k2);
        tree.insert(k3);
        tree.insert(k4);
        tree.insert(k5);

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

        let k1 = 10;
        let k2 = 20;
        let k3 = 30;
        let k4 = 40;
        let k5 = 50;
        let k6 = 60;

        tree.insert(k1);
        tree.insert(k2);
        tree.insert(k3);
        tree.insert(k4);
        tree.insert(k5);
        tree.insert(k6);

        tree.remove(30); // Remove um nó interno e precisa substituir pelo predecessor ou sucessor

        assert_eq!(tree.find(30), None);
        assert!(tree.find(20) == Some(k2) || tree.find(40) == Some(k4)); // Verifica que o substituto está correto
    }

    #[test]
    fn test_remove_root_replacement() {
        let mut tree = BTree::new(2);

        let k1 = 10;
        let k2 = 20;
        let k3 = 30;
        let k4 = 40;
        let k5 = 50;
        let k6 = 60;
        let k7 = 70;
        let k8 = 80;

        tree.insert(k1);
        tree.insert(k2);
        tree.insert(k3);
        tree.insert(k4);
        tree.insert(k5);
        tree.insert(k6);
        tree.insert(k7);
        tree.insert(k8);

        tree.remove(50);
        tree.remove(60);
        tree.remove(70);
        tree.remove(80);

        tree.remove(30);

        let k11 = tree.find(30);
        let k22 = tree.find(40);

        assert_eq!(k11, None);
        assert_eq!(k22, Some(k4));
    }
}
