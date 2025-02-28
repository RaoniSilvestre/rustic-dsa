use crate::data_structures::btree::{
    auxiliary::{RemovalResult, SearchResult},
    Key, Node,
};

impl Node {
    pub fn remove(&mut self, k: Key) -> RemovalResult {
        match (self.search(&k), self.is_leaf) {
            (SearchResult::GoDown(_), true) => RemovalResult::RemoveCompleted,
            (SearchResult::Find(i), true) => self.remove_leaf(k, i),
            (SearchResult::Find(i), false) => self.remove_branch(k, i),
            (SearchResult::GoDown(i), false) => {
                let child = self.children[i].remove(k);

                match child {
                    RemovalResult::NotLeafRemoveFail(_failed_key, _new_node) => todo!(),
                    RemovalResult::LeafRemoveFail(_failed_key) => todo!(),
                    RemovalResult::RemoveCompleted => RemovalResult::RemoveCompleted,
                }
            }
        }
    }

    fn remove_leaf(&mut self, k: Key, i: usize) -> RemovalResult {
        let grade = (self.grade + 1) as usize;

        match self.keys.len() >= grade {
            true => {
                self.keys.remove(i);
                return RemovalResult::RemoveCompleted;
            }
            false => RemovalResult::LeafRemoveFail(k),
        }
    }

    fn remove_branch(&mut self, k: Key, i: usize) -> RemovalResult {
        let left_len = self.children[i].keys.len();
        let right_len = self.children[i + 1].keys.len();

        let grade = (self.grade + 1) as usize;

        match (left_len >= grade, right_len >= grade) {
            (true, _) => {
                let max = self.children[i].keys.pop().unwrap();
                self.keys[i] = max;
                return RemovalResult::RemoveCompleted;
            }
            (_, true) => {
                let min = self.children[i + 1].keys.remove(0);
                self.keys[i] = min;
                return RemovalResult::RemoveCompleted;
            }
            (false, false) => {
                // Precisa fazer merge
                todo!()
            }
        }
    }
}
