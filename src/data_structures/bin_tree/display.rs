use std::fmt::{self, Display};

use super::BinaryTree;

impl<T: Display> Display for BinaryTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ")?;
        if let Self::Some { value, left, right } = self {
            write!(f, "{}", *left)?;
            write!(f, "{} ", value)?;
            write!(f, "{}", *right)?;
        }

        write!(f, "]")?;

        Ok(())
    }
}
