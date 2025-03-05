use std::{cell::RefCell, fmt::Debug, ops::Deref, rc::Rc};

// Why use Rc<RefCell<T>>:
// https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
type QueueLink<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug, Clone, Default)]
pub struct Queue<T> {
    front: QueueLink<T>,
    rear: QueueLink<T>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node<T> {
    value: T,
    next: QueueLink<T>,
}

impl<T: Copy + Clone + PartialEq> Queue<T> {
    pub fn push(&mut self, enqueued_value: T) {
        let enqueued_node = Node::new_link(enqueued_value);

        match &self.rear {
            None => {
                self.rear = enqueued_node.clone();
                self.front = enqueued_node;
            }
            Some(old_rear_node) => {
                old_rear_node.deref().borrow_mut().next = enqueued_node.clone();
                self.rear = enqueued_node;
            }
        }
    }

    pub fn pop(&mut self) {
        if let Some(node) = &self.front {
            self.front = node.clone().deref().borrow_mut().next.clone();
        }
    }

    pub fn front(&self) -> Option<T> {
        self.front.as_ref().map(|node| node.borrow().value)
    }

    pub fn back(&self) -> Option<T> {
        self.rear.as_ref().map(|node| node.borrow().value)
    }

    pub fn is_empty(&self) -> bool {
        self.front.is_none()
    }
}

impl<T: Copy> Node<T> {
    fn new(value: T, next: QueueLink<T>) -> Node<T> {
        Node { value, next }
    }

    fn new_link(value: T) -> QueueLink<T> {
        Some(Rc::new(RefCell::new(Node::new(value, None))))
    }
}

mod display;
