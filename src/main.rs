use rustic_dsa::data_structures::btree::{BTree, Key};

fn main() {
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

    println!("\nInitial tree: \n{}", tree);
    tree.remove(50);
    println!("\nRemove {}\n{}", 50, tree);
    tree.remove(60);
    println!("\nRemove {}\n{}", 60, tree);
    tree.remove(70);
    println!("\nRemove {}\n{}", 70, tree);
    tree.remove(80);
    println!("\nRemove {}\n{}", 80, tree);
    tree.remove(30);
    println!("\nRemove {}\n{}", 30, tree);
}
