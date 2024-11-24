use rustic_dsa::data_structures::bin_tree::BinTree;

fn main() {
    let mut tree: BinTree<i32> = BinTree::default();

    tree.insert(5);
    tree.insert(2);
    tree.insert(8);
    tree.insert(3);
    tree.insert(9);
    tree.insert(7);

    println!("{tree}");
    tree.remove(8);
    println!("{tree}")
}
