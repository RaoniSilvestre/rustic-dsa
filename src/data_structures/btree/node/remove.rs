use crate::data_structures::btree::{
    auxiliary::{RemovalResult, SearchResult},
    Key, Node,
};

impl Node {
    pub fn remove(&mut self, k: i32) -> RemovalResult {
        match (self.search(k), self.is_leaf) {
            (SearchResult::GoDown(_), true) => RemovalResult::RemoveCompleted,
            (SearchResult::Find(i), true) => self.remove_leaf(k, i),
            (SearchResult::Find(i), false) => self.remove_branch(k, i),
            (SearchResult::GoDown(i), false) => {
                let child = self.children[i].remove(k);

                match child {
                    RemovalResult::NotLeafRemoveFail(_failed_key, _new_node) => {
                        todo!("NotLeafRemoveFail ainda não foi implementado")
                    }
                    RemovalResult::LeafRemoveFail(_failed_key) => {
                        todo!("LeafRemoveFail não foi implementado")
                    }
                    RemovalResult::RemoveCompleted => RemovalResult::RemoveCompleted,
                }
            }
        }
    }

    fn remove_leaf(&mut self, k: i32, i: usize) -> RemovalResult {
        let grade = (self.grade + 1) as usize;

        match self.keys.len() >= grade {
            true => {
                self.keys.remove(i);
                return RemovalResult::RemoveCompleted;
            }
            false => RemovalResult::LeafRemoveFail(k),
        }
    }

    fn remove_branch(&mut self, _k: i32, i: usize) -> RemovalResult {
        let k = self.pop_rightmost_left();

        self.keys[i] = k;

        return RemovalResult::RemoveCompleted;
    }

    fn pop_rightmost_left(&mut self) -> Key {
        let left_node = self
            .child_mut(0)
            .expect("Left node inexistente na função pop_rightmost_left");

        left_node.pop_rightmost()
    }

    fn pop_rightmost(&mut self) -> Key {
        match self.is_leaf {
            true => self.last_key(),
            false => self.last_child().pop_rightmost(),
        }
    }
}
