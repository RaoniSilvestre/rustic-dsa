use crate::data_structures::btree::{
    auxiliary::{InsertionResult, SearchResult},
    Key, Node,
};

impl<T: Key> Node<T> {
    pub fn insert(&mut self, k: T) -> InsertionResult<T> {
        let result = self.search(&k);

        if self.is_leaf {
            return self.try_insert(k);
        }

        if let SearchResult::GoDown(i) = result {
            if let InsertionResult::AddToFater(middle_key, new_node) = self.children[i].insert(k) {
                self.children.insert(i + 1, new_node);
                self.children.sort();

                self.keys.insert(i, middle_key);
                self.keys.sort();

                if self.is_full() {
                    return self.split_node();
                }
            }
        }

        InsertionResult::Inserted
    }

    fn try_insert(&mut self, k: T) -> InsertionResult<T> {
        self.keys.push(k);
        self.keys.sort();

        match self.is_full() {
            true => self.split_node(),
            false => InsertionResult::Inserted,
        }
    }

    fn split_node(&mut self) -> InsertionResult<T> {
        let mid = self.grade;

        let mut right_node = Node::new(self.is_leaf, self.grade);

        right_node.keys = self.keys.drain(mid + 1..).collect();

        if !self.is_leaf {
            right_node.children = self.children.drain(mid + 1..).collect();
        }

        let middle_key = self.keys.remove(mid);

        InsertionResult::AddToFater(middle_key, right_node)
    }
}
