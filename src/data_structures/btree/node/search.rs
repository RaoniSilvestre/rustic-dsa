use std::cmp::Ordering;

use crate::data_structures::btree::{auxiliary::SearchResult, Key, Node};

impl<T: Key> Node<T> {
    pub fn search(&self, k: &T) -> SearchResult {
        for (i, key) in self.keys.iter().enumerate() {
            match key.cmp(k) {
                Ordering::Less => {}
                Ordering::Equal => return SearchResult::Find(i),
                Ordering::Greater => return SearchResult::GoDown(i),
            }
        }
        SearchResult::GoDown(self.keys.len())
    }
}
