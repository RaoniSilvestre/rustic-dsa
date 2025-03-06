type StackLink<T> = Option<Box<StackNode<T>>>;

#[derive(Debug, Default)]
pub struct Stack<T> {
    top: StackLink<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct StackNode<T> {
    pub value: T,
    pub pred: StackLink<T>,
}

impl<T: Copy + Clone + PartialEq> Stack<T> {
    pub fn is_empty(&self) -> bool {
        self.top.is_none()
    }

    pub fn top(&self) -> Option<T> {
        self.top.as_ref().map(|node| node.value)
    }

    pub fn push(&mut self, value: T) {
        match &self.top {
            Option::Some(node) => {
                self.top = new_link(value, Some(node.clone()));
            }
            Option::None => {
                self.top = new_link(value, None);
            }
        };
    }

    pub fn pop(&mut self) {
        if let Some(node) = &self.top {
            self.top = node.pred.clone();
        }
    }
}

impl<T: Copy> StackNode<T> {
    fn new(value: T, pred: Option<Box<StackNode<T>>>) -> StackNode<T> {
        StackNode { value, pred }
    }
}

#[test]
fn new_stack_node() {
    let sn = StackNode {
        pred: None,
        value: 0,
    };

    let new_sn = StackNode::new(0, None);

    assert_eq!(sn, new_sn)
}

fn new_link<T: Copy>(value: T, predecessor: Option<Box<StackNode<T>>>) -> StackLink<T> {
    Some(Box::new(StackNode::new(value, predecessor)))
}

#[test]
fn new_link_test() {
    let link = new_link(0, None);

    assert_eq!(Some(Box::new(StackNode::new(0, None))), link)
}
