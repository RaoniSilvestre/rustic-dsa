use crate::data_structures::btree::{
    auxiliary::{RemovalResult, SearchResult},
    Key, Node,
};

impl Node {
    pub fn remove(&mut self, k: i32) -> RemovalResult {
        match (self.search(k), self.is_leaf) {
            (SearchResult::GoDown(_), true) => RemovalResult::RemoveCompleted,
            (SearchResult::GoDown(i), false) => self.remove_on_child(k, i),
            (SearchResult::Find(i), false) => self.remove_branch(k, i),
            (SearchResult::Find(i), true) => self.remove_leaf(i),
        }
    }

    fn remove_on_child(&mut self, k: i32, i: usize) -> RemovalResult {
        match self.children[i].remove(k) {
            RemovalResult::InsuficientChildren => self.handle_underflow(i),
            RemovalResult::RemoveCompleted => RemovalResult::RemoveCompleted,
        }
    }

    fn handle_underflow(&mut self, i: usize) -> RemovalResult {
        let has_left = i > 0;
        let has_right = i < self.max_grade();

        if has_left && self.children[i - 1].can_lend() {
            self.borrow_from_left(i);
            return RemovalResult::RemoveCompleted;
        }

        if has_right && self.children[i + 1].can_lend() {
            self.borrow_from_right(i);
            return RemovalResult::RemoveCompleted;
        }

        match (has_left, has_right) {
            (true, _) => self.merge_children(i - 1),
            (_, true) => self.merge_children(i),
            // É raiz!
            (false, false) => return RemovalResult::RemoveCompleted,
        }

        match self.keys.len() >= self.max_grade() {
            true => RemovalResult::RemoveCompleted,
            false => RemovalResult::InsuficientChildren,
        }
    }

    fn merge_children(&mut self, _i: usize) {
        todo!("Merge não implementado ainda")
    }

    fn borrow_from_left(&mut self, _i: usize) {
        todo!("Emprestar do esquerdo não implementado ainda")
    }

    fn borrow_from_right(&mut self, _i: usize) {
        todo!("Emprestar do direito não implementado ainda")
    }

    fn remove_leaf(&mut self, i: usize) -> RemovalResult {
        self.keys.remove(i);

        match self.keys.len() >= self.grade as usize {
            true => RemovalResult::RemoveCompleted,
            false => RemovalResult::InsuficientChildren,
        }
    }

    fn remove_branch(&mut self, _k: i32, i: usize) -> RemovalResult {
        let predecessor = self.get_predecessor(i);
        let result = self.children[i].remove(predecessor);
        let k = self.pop_rightmost_left();

        self.keys[i] = k;

        match result {
            RemovalResult::InsuficientChildren => self.handle_underflow(i),
            completed => completed,
        }
    }

    fn get_predecessor(&mut self, i: usize) -> i32 {
        let mut node = &mut self.children[i];

        while !node.is_leaf {
            node = node.last_child();
        }

        (node.keys.last().unwrap()).key
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
