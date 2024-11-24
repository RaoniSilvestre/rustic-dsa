use std::fmt::{Debug, Display};

use super::Queue;

impl<T: Display + Debug> Display for Queue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut temp = self.front.clone();

        write!(f, "[")?;
        loop {
            match temp {
                Some(ref node) => {
                    let ref_cell_node = node.clone();
                    let next_node = &ref_cell_node.borrow().next;
                    let node_value = &ref_cell_node.borrow().value;

                    match next_node {
                        None => write!(f, "{:?}", node_value)?,
                        Some(_) => write!(f, "{:?}, ", node_value)?,
                    }

                    temp = next_node.clone();
                }
                None => {
                    write!(f, "]")?;
                    break;
                }
            }
        }

        Ok(())
    }
}
