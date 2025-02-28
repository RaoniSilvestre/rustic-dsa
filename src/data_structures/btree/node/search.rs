use std::cmp::Ordering;

use crate::data_structures::btree::{auxiliary::SearchResult, Key, Node};

impl Node {
    pub fn search(&self, k: &Key) -> SearchResult {
        println!("{}", self);
        for (i, key) in self.keys.iter().enumerate() {
            match key.key.cmp(&k.key) {
                Ordering::Less => {}
                Ordering::Equal => return SearchResult::Find(i),
                Ordering::Greater => return SearchResult::GoDown(i),
            }
        }
        SearchResult::GoDown(self.keys.len())
    }
}
