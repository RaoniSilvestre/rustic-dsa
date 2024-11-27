use std::{cmp::Ordering, fmt::Debug, mem::replace};

pub trait OrderedCopy: Ord + Copy {}
impl<T: Ord + Copy> OrderedCopy for T {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryTree<T> {
    Nil,
    Some {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Self::Nil
    }
}

impl<T: OrderedCopy> BinaryTree<T> {
    pub fn new(value: T, l: BinaryTree<T>, r: BinaryTree<T>) -> Self {
        Self::Some {
            value,
            left: Box::new(l),
            right: Box::new(r),
        }
    }

    pub fn insert(&mut self, chave: T) {
        match self {
            Self::Nil => *self = BinaryTree::new(chave, Self::Nil, Self::Nil),
            Self::Some { value, left, right } => match chave.cmp(value) {
                Ordering::Less => left.insert(chave),
                Ordering::Greater => right.insert(chave),
                Ordering::Equal => (),
            },
        }
    }

    pub fn search(&self, chave: T) -> bool {
        match self {
            Self::Nil => false,
            Self::Some { value, left, right } => match chave.cmp(value) {
                Ordering::Less => left.search(chave),
                Ordering::Greater => right.search(chave),
                Ordering::Equal => true,
            },
        }
    }

    pub fn remove(&mut self, chave: T) {
        match self {
            Self::Nil => (),
            Self::Some { value, left, right } => match chave.cmp(value) {
                Ordering::Less => left.remove(chave),
                Ordering::Greater => right.remove(chave),
                Ordering::Equal => self.remove_node(),
            },
        }
    }

    fn remove_node(&mut self) {
        match self {
            Self::Nil => (),
            Self::Some { left, right, value } => match (&mut **left, &mut **right) {
                (Self::Nil, Self::Nil) => *self = Self::Nil,
                (Self::Nil, _) => *self = replace(&mut **right, Self::Nil),
                (_, Self::Nil) => *self = replace(&mut **left, Self::Nil),
                (_, _) => {
                    if let Self::Some {
                        value: min_value, ..
                    } = right.find_min()
                    {
                        *value = *min_value;
                        right.remove(*min_value);
                    }
                }
            },
        }
    }

    fn find_min(&self) -> &Self {
        match self {
            Self::Some { left, .. } => match **left {
                Self::Nil => self,
                _ => left.find_min(),
            },
            _ => self,
        }
    }
}

pub mod display;
