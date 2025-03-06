use std::cmp::Ordering;

use super::Key;

pub mod insert;
pub mod remove;
pub mod search;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Node<T: Key> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
    pub is_leaf: bool,
    pub grade: usize,
}

impl<T: Key> Node<T> {
    pub fn new(is_leaf: bool, grade: usize) -> Self {
        Node {
            children: Vec::new(),
            is_leaf,
            keys: Vec::new(),
            grade,
        }
    }

    pub fn push_key(&mut self, key: T) {
        self.keys.push(key);
    }

    pub fn push_child(&mut self, child: Node<T>) {
        self.children.push(child);
    }

    pub fn sort(&mut self) {
        self.keys.sort();
        self.children.sort();
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn children(&self) -> &[Self] {
        &self.children
    }

    pub fn keys(&self) -> &[T] {
        &self.keys
    }

    pub fn pop_child(&mut self) -> Option<Self> {
        self.children.pop()
    }

    pub fn key(&self, i: usize) -> Option<&T> {
        self.keys.as_slice().get(i)
    }

    pub fn child(&self, i: usize) -> Option<&Node<T>> {
        self.children.as_slice().get(i)
    }

    fn child_mut(&mut self, i: usize) -> Option<&mut Node<T>> {
        self.children.get_mut(i)
    }

    fn is_full(&self) -> bool {
        self.keys.len() == (2 * self.grade + 1)
    }

    fn last_key(&mut self) -> T {
        self.keys.pop().expect("Keys vazias... ué :P")
    }

    fn last_child(&mut self) -> &mut Node<T> {
        self.children.last_mut().expect("Não tinha última child :P")
    }

    fn can_borrow(&self) -> bool {
        self.keys.len() > self.grade
    }
}

impl<T: Key> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.keys[0].cmp(&other.keys[0]))
    }
}

impl<T: Key> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.keys[0].cmp(&other.keys[0])
    }
}
