use auxiliary::{InsertionResult, SearchResult};
use serde::Serialize;

mod auxiliary;
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

impl BTree {
    pub fn new(g: i32) -> Self {
        BTree {
            root: Node::new(true, g),
        }
    }

    pub fn find(&self, k: i32) -> Option<Key> {
        let mut curr_node = &self.root;
        loop {
            match curr_node.search(k) {
                SearchResult::Find(i) => return curr_node.key(i).cloned(),
                SearchResult::GoDown(i) => match curr_node.child(i) {
                    None => return None,
                    Some(next_node) => curr_node = next_node,
                },
            }
        }
    }

    pub fn insert(&mut self, k: Key) {
        let insertion = self.root.insert(k);

        if let InsertionResult::AddToFater(k, new_node) = insertion {
            *self = Self::new_root(self.root.clone(), k, new_node);
        }
    }

    pub fn remove(&mut self, k: i32) {
        self.root.remove(k);
    }

    fn new_root(root: Node, k: Key, new_node: Node) -> Self {
        let mut new_root = Node::new(false, root.grade);

        new_root.keys.push(k);

        new_root.children.push(root);
        new_root.children.push(new_node);

        new_root.children.sort();

        Self { root: new_root }
    }
}
