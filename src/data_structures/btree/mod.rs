use serde::Serialize;

mod auxiliary;
mod btree;
mod display;
mod node;

#[derive(Debug, Clone, Default)]
pub struct BTree {
    root: Node,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Default)]
pub struct Node {
    keys: Vec<Key>,
    children: Vec<Node>,
    is_leaf: bool,
    grade: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Serialize)]
pub struct Key {
    key: i32,
    nome: String,
    quantidade: usize,
}
