use crate::data_structures::btree::{
    auxiliary::{RemovalResult, SearchResult},
    Key, Node,
};

impl Node {
    pub fn remove(&mut self, k: i32) -> RemovalResult {
        match (self.search(k), self.is_leaf) {
            (SearchResult::GoDown(_), true) => RemovalResult::RemoveCompleted,
            (SearchResult::GoDown(i), false) => self.remove_on_child(k, i),
            (SearchResult::Find(i), false) => self.remove_branch(i),
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
        let has_right = i < self.children.len() - 1;

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

        match self.keys.len() >= (self.grade - 1) as usize {
            true => RemovalResult::RemoveCompleted,
            false => RemovalResult::InsuficientChildren,
        }
    }

    fn merge_children(&mut self, i: usize) {
        let parent_key = self.keys.remove(i);
        let mut left_child = self.children.remove(i);
        let mut right_child = self.children.remove(i);

        left_child.keys.push(parent_key);
        left_child.keys.append(&mut right_child.keys);

        if !left_child.is_leaf {
            left_child.children.extend(right_child.children);
        }

        self.children.insert(i, left_child);
    }

    fn borrow_from_left(&mut self, i: usize) {
        let left_idx = i - 1;
        let left_child = &mut self.children[left_idx];
        let borrowed_key = left_child.keys.pop().unwrap();

        let borrowed_child = if !left_child.is_leaf {
            left_child.children.pop()
        } else {
            None
        };

        let parent_key = self.keys[left_idx].clone();
        self.keys[left_idx] = borrowed_key.clone();

        let target_child = &mut self.children[i];
        target_child.keys.insert(0, parent_key);
        if let Some(child) = borrowed_child {
            target_child.children.insert(0, child);
        }
    }

    fn borrow_from_right(&mut self, i: usize) {
        let right_idx = i + 1;
        let right_child = &mut self.children[right_idx];
        let borrowed_key = right_child.keys.remove(0);
        let borrowed_child = if !right_child.is_leaf {
            Some(right_child.children.remove(0))
        } else {
            None
        };

        let parent_key = self.keys[i].clone();
        self.keys[i] = borrowed_key.clone();

        let target_child = &mut self.children[i];
        target_child.keys.push(parent_key);
        if let Some(child) = borrowed_child {
            target_child.children.push(child);
        }
    }

    fn remove_leaf(&mut self, i: usize) -> RemovalResult {
        self.keys.remove(i);

        match self.keys.len() >= self.grade as usize {
            true => RemovalResult::RemoveCompleted,
            false => RemovalResult::InsuficientChildren,
        }
    }

    fn remove_branch(&mut self, i: usize) -> RemovalResult {
        self.keys[i] = self.pop_rightmost_left();

        let predecessor = self.get_predecessor(i);

        match self.children[i].remove(predecessor) {
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
