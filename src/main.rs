use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use rustic_dsa::data_structures::btree::{BTree, Key};

fn main() {
    let mut b = BTree::new(2);

    let file = File::open("dadosB.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| serde_plain::from_str(&line.trim()).unwrap())
        .take(13)
        .for_each(|k: Key| b.insert(k));

    b.insert(Key::new(100, "nome", 10));
    b.insert(Key::new(99, "nome", 10));

    b.remove(103);

    println!("{}", b);
}
