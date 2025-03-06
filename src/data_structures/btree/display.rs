use std::fmt::{self, Display};

use super::{BTree, Key, Node};

impl<T: Display + Key> fmt::Display for BTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn display_node<T: Key + Display>(
            node: &Node<T>,
            f: &mut fmt::Formatter<'_>,
            level: usize,
            child_number: usize,
        ) -> fmt::Result {
            for _ in 0..level {
                write!(f, "    ")?;
            }

            writeln!(f, "Nó {}: {}", child_number, node)?;

            for (i, child) in node.children().iter().enumerate() {
                display_node(child, f, level + 1, i + 1)?;
            }

            Ok(())
        }

        writeln!(f, "Árvore B:")?;
        display_node(&self.root, f, 0, 1)?;

        Ok(())
    }
}

impl<T: Display + Key> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, key) in self.keys().iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", key)?;
        }
        write!(f, "]")
    }
}
