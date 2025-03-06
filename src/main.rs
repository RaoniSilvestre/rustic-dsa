use rustic_dsa::data_structures::btree::BTree;

fn main() {
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

    println!("\nInitial tree: \n{}", tree);
    tree.remove(k5);
    println!("\nRemove {}\n{}", k5, tree);
    tree.remove(k6);
    println!("\nRemove {}\n{}", k6, tree);
    tree.remove(k7);
    println!("\nRemove {}\n{}", k7, tree);
    tree.remove(k8);
    println!("\nRemove {}\n{}", k8, tree);
    tree.remove(k3);
    println!("\nRemove {}\n{}", k3, tree);
}
