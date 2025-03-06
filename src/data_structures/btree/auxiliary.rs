use super::{BTree, Key, Node};

#[derive(Debug)]
pub enum InsertionResult<T: Key> {
    Inserted,
    AddToFater(T, Node<T>),
}

#[derive(Debug)]
pub enum SearchResult {
    Find(usize),
    GoDown(usize),
}

#[derive(Debug)]
pub enum RemovalResult {
    RemoveCompleted,
    InsuficientChildren,
}

pub fn new_root<T: Key>(root: Node<T>, k: T, new_node: Node<T>) -> BTree<T> {
    let mut new_root = Node::new(false, root.grade);

    new_root.push_key(k);

    new_root.push_child(root);
    new_root.push_child(new_node);

    new_root.sort();

    BTree { root: new_root }
}
