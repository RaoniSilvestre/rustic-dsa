use crate::data_structures::btree::{
    auxiliary::{InsertionResult, SearchResult},
    Key, Node,
};

impl Node {
    pub fn insert(&mut self, k: Key) -> InsertionResult {
        let result = self.search(&k);

        if self.is_leaf {
            return self.try_insert(k);
        }

        if let SearchResult::GoDown(i) = result {
            let child = self.children[i].insert(k);

            if let InsertionResult::AddToFater(middle_key, new_node) = child {
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

    pub fn try_insert(&mut self, k: Key) -> InsertionResult {
        self.keys.push(k);
        self.keys.sort();

        match self.is_full() {
            true => self.split_node(),
            false => InsertionResult::Inserted,
        }
    }

    pub fn split_node(&mut self) -> InsertionResult {
        let t = self.grade as usize;
        let mid = t;

        let mut right_node = Node::new(self.is_leaf, self.grade);

        right_node.keys = self.keys.drain(mid + 1..).collect();

        if !self.is_leaf {
            right_node.children = self.children.drain(mid + 1..).collect();
        }

        let middle_key = self.keys.remove(mid);

        InsertionResult::AddToFater(middle_key, right_node)
    }

    pub fn insert_key(&mut self, k: Key) -> InsertionResult {
        self.keys.push(k);
        self.keys.sort();

        InsertionResult::Inserted
    }
}
