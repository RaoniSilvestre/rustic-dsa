use auxiliary::{new_root, InsertionResult, RemovalResult, SearchResult};
use node::Node;

mod auxiliary;
mod display;
mod node;

pub trait Key: Ord + Clone {}
impl<T: Ord + Clone> Key for T {}

#[derive(Debug, Clone, Default)]
pub struct BTree<T: Key> {
    root: Node<T>,
}

impl<T: Key> BTree<T> {
    pub fn new(g: usize) -> Self {
        BTree {
            root: Node::new(true, g),
        }
    }

    pub fn find(&self, k: T) -> Option<T> {
        let mut curr_node = &self.root;
        loop {
            match curr_node.search(&k) {
                SearchResult::Find(i) => return curr_node.key(i).cloned(),
                SearchResult::GoDown(i) => match curr_node.child(i) {
                    None => return None,
                    Some(next_node) => curr_node = next_node,
                },
            }
        }
    }

    pub fn insert(&mut self, k: T) {
        let insertion = self.root.insert(k);

        if let InsertionResult::AddToFater(k, new_node) = insertion {
            *self = new_root(self.root.clone(), k, new_node);
        }
    }

    pub fn remove(&mut self, k: T) {
        let result = self.root.remove(k);

        // Trata underflow na raiz
        if let RemovalResult::InsuficientChildren = result {
            if !self.root.is_leaf && self.root.is_empty() {
                if let Some(new_root) = self.root.pop_child() {
                    self.root = new_root;
                }
            }
        }
    }
}
