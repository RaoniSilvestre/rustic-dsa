use crate::OrderedCopy;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AVLTree<T> {
    #[default]
    Nil,
    Some(Node<T>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<T> {
    value: T,
    height: i32,
    left: Box<AVLTree<T>>,
    right: Box<AVLTree<T>>,
}

impl<T: OrderedCopy> AVLTree<T> {
    pub fn left_rotate(self: Self) -> Self {
        let tree = self.clone();

        if let AVLTree::Some(node) = self {
            return node.left_rotation();
        }

        tree
    }

    pub fn right_rotate(self: Self) -> Self {
        let tree = self.clone();

        if let AVLTree::Some(node) = self {
            return node.right_rotation();
        }

        tree
    }

    pub fn double_left_rotation(self: Self) -> Self {
        let tree = self.clone();

        if let AVLTree::Some(node) = self {
            return node.double_left_rotation();
        }

        tree
    }
}

impl<T: OrderedCopy> Node<T> {
    pub fn left_rotation(mut self: Self) -> AVLTree<T> {
        match *self.right {
            AVLTree::Some(mut right_node) => {
                self.right = right_node.left;
                right_node.left = Box::new(AVLTree::Some(self));
                return AVLTree::Some(right_node);
            }
            AVLTree::Nil => *self.right,
        }
    }

    pub fn right_rotation(mut self: Self) -> AVLTree<T> {
        match *self.left {
            AVLTree::Some(mut left_node) => {
                self.left = left_node.right;
                left_node.right = Box::new(AVLTree::Some(self));
                return AVLTree::Some(left_node);
            }
            AVLTree::Nil => *self.left,
        }
    }

    pub fn double_left_rotation(mut self: Self) -> AVLTree<T> {
        match *self.right {
            AVLTree::Some(right_node) => {
                self.right = Box::new(right_node.right_rotation());
                return self.left_rotation();
            }
            AVLTree::Nil => *self.right,
        }
    }

    pub fn double_right_rotation(mut self: Self) -> AVLTree<T> {
        match *self.left {
            AVLTree::Some(left_node) => {
                self.left = Box::new(left_node.left_rotation());
                return self.right_rotation();
            }
            AVLTree::Nil => *self.left,
        }
    }
}
