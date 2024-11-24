use std::cmp::Ordering;

pub trait OrderedCopy: Ord + Copy {}
impl<T: Ord + Copy> OrderedCopy for T {}

pub type OptionNode<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone)]
pub struct Node<OrderedCopy> {
    value: OrderedCopy,
    left: OptionNode<OrderedCopy>,
    right: OptionNode<OrderedCopy>,
}

#[derive(Debug, Clone)]
pub struct BinTree<OrderedCopy> {
    root: OptionNode<OrderedCopy>,
}

impl<T: OrderedCopy> BinTree<T> {
    pub fn inserir(&mut self, chave: T) {
        match self.root.as_mut() {
            Some(node) => node.insert(chave),
            None => self.root = Node::new_node(chave),
        }
    }

    pub fn search(&self, chave: T) -> Option<Node<T>> {
        self.root.as_ref().and_then(|node| node.search(chave))
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

// Principal
impl<T: OrderedCopy> Node<T> {
    pub fn insert(&mut self, chave: T) {
        match self.value.cmp(&chave) {
            Ordering::Equal => return,
            Ordering::Less => Self::insert_on_side(&mut self.left, chave),
            Ordering::Greater => Self::insert_on_side(&mut self.right, chave),
        }
    }

    fn search(&self, chave: T) -> Option<Node<T>> {
        match self.value.cmp(&chave) {
            Ordering::Equal => Some(self.clone()),
            Ordering::Less => Self::search_on_side(&self.left, chave),
            Ordering::Greater => Self::search_on_side(&self.right, chave),
        }
    }
}

// Auxiliar
impl<T: OrderedCopy> Node<T> {
    fn insert_on_side(side: &mut OptionNode<T>, chave: T) {
        match side {
            Some(node) => node.insert(chave),
            None => *side = Self::new_node(chave),
        }
    }

    fn search_on_side(side: &OptionNode<T>, chave: T) -> Option<Node<T>> {
        side.as_ref().and_then(|node| node.search(chave))
    }
}
