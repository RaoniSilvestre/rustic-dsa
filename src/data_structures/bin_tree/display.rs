use std::fmt::{self, Display};

use super::{BinTree, Node, OptionNode};

impl<T: fmt::Display> fmt::Display for BinTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ")?;
        Node::print(&self.root, f)?;
        write!(f, "]")?;

        Ok(())
    }
}

impl<T: Display> Node<T> {
    pub fn print(node_opt: &OptionNode<T>, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(node) = node_opt {
            Self::print(&node.left, f)?;
            write!(f, "{} ", node.value)?;
            Self::print(&node.right, f)?;
        }

        Ok(())
    }
}
