use rustic_dsa::data_structures::btree::{BTree, Key};

fn main() {
    let mut b: BTree = BTree::default();

    let k: Key = Key::new(10, String::from("nome"), 10);

    b.insert(k);
}
