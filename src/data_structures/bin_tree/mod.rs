use std::{cmp::Ordering, fmt::Debug};

pub trait OrderedCopy: Ord + Copy + Debug {}
impl<T: Ord + Copy + Debug> OrderedCopy for T {}

pub type OptionNode<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<OrderedCopy> {
    value: OrderedCopy,
    left: OptionNode<OrderedCopy>,
    right: OptionNode<OrderedCopy>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BinTree<OrderedCopy> {
    root: OptionNode<OrderedCopy>,
}

impl<T: OrderedCopy> BinTree<T> {
    pub fn insert(&mut self, chave: T) {
        Node::insert(&mut self.root, chave)
    }

    pub fn search(&self, chave: T) -> bool {
        Node::search(&self.root, chave)
    }

    pub fn remove(&mut self, chave: T) {
        Node::remove(&mut self.root, chave);
    }
}

// Principal
impl<T: OrderedCopy> Node<T> {
    fn insert(node_opt: &mut OptionNode<T>, chave: T) {
        match node_opt {
            Some(node) => match chave.cmp(&node.value) {
                Ordering::Less => Self::insert(&mut node.left, chave),
                Ordering::Greater => Self::insert(&mut node.right, chave),
                _ => (),
            },
            None => *node_opt = Node::new_node(chave),
        }
    }

    fn search(node_opt: &OptionNode<T>, chave: T) -> bool {
        match node_opt {
            Some(node) => match chave.cmp(&node.value) {
                Ordering::Equal => true,
                Ordering::Less => Self::search(&node.left, chave),
                Ordering::Greater => Self::search(&node.right, chave),
            },
            None => false,
        }
    }

    fn remove(node_opt: &mut OptionNode<T>, chave: T) {
        if let Some(node) = node_opt {
            match chave.cmp(&node.value) {
                Ordering::Less => Self::remove(&mut node.left, chave),
                Ordering::Greater => Self::remove(&mut node.right, chave),
                Ordering::Equal => Self::remove_node(node_opt),
            }
        }
    }
}

// Inicialização
impl<T: OrderedCopy> Node<T> {
    pub fn new(value: T, left: OptionNode<T>, right: OptionNode<T>) -> Self {
        Self { value, left, right }
    }

    pub fn new_node(chave: T) -> OptionNode<T> {
        Some(Box::new(Self::new(chave, None, None)))
    }
}

// Auxiliar
impl<T: OrderedCopy> Node<T> {
    fn find_min(side: &OptionNode<T>) -> OptionNode<T> {
        match side {
            None => None,
            Some(node) if node.left.is_none() => Some(node.clone()),
            Some(node) => Self::find_min(&node.left),
        }
    }

    fn remove_node(node_opt: &mut OptionNode<T>) {
        if let Some(node) = node_opt {
            match (node.left.take(), node.right.take()) {
                (None, None) => {
                    *node_opt = None;
                }
                (None, Some(right_node)) => {
                    node.value = right_node.value;
                    node.left = right_node.left;
                    node.right = right_node.right;
                }
                (Some(left_node), None) => {
                    node.value = left_node.value;
                    node.left = left_node.left;
                    node.right = left_node.right;
                }
                (Some(left_node), Some(right_node)) => {
                    let mut min_node = Node::find_min(&mut Some(right_node));
                    if let Some(min) = min_node.as_mut() {
                        node.value = min.value;
                        node.left = Some(left_node);
                        node.right = min.right.clone();
                        Node::remove(&mut node.right, node.value);
                    }
                }
            }
        }
    }
}

pub mod display;
