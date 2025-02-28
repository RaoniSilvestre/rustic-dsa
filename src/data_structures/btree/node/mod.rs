use super::{Key, Node};

pub mod insert;
pub mod remove;
pub mod search;

impl Node {
    pub fn new(is_leaf: bool, grade: i32) -> Self {
        Node {
            children: Vec::new(),
            is_leaf,
            keys: Vec::new(),
            grade,
        }
    }

    pub fn key(&self, i: usize) -> Option<&Key> {
        self.keys.as_slice().get(i)
    }

    pub fn child(&self, i: usize) -> Option<&Node> {
        self.children.as_slice().get(i)
    }

    pub fn child_mut(&mut self, i: usize) -> Option<&mut Node> {
        self.children.get_mut(i)
    }

    pub fn is_full(&self) -> bool {
        self.keys.len() == (2 * self.grade + 1) as usize
    }
}
