#[cfg(test)]
mod tests {
    use rustic_dsa::data_structures::stack::Stack;

    #[test]
    fn test_is_empty() {
        let stack: Stack<i32> = Stack::default();
        assert!(stack.is_empty());

        let mut stack = Stack::default();
        stack.push(10);
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_top() {
        let mut stack = Stack::default();
        assert_eq!(stack.top(), None);

        stack.push(10);
        assert_eq!(stack.top(), Some(10));

        stack.push(20);
        assert_eq!(stack.top(), Some(20));

        stack.pop();
        assert_eq!(stack.top(), Some(10));
    }

    #[test]
    fn test_push() {
        let mut stack = Stack::default();
        stack.push(10);
        assert_eq!(stack.top(), Some(10));

        stack.push(20);
        assert_eq!(stack.top(), Some(20));

        stack.push(30);
        assert_eq!(stack.top(), Some(30));
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::default();
        stack.push(10);
        stack.push(20);
        stack.push(30);

        stack.pop();
        assert_eq!(stack.top(), Some(20));

        stack.pop();
        assert_eq!(stack.top(), Some(10));

        stack.pop();
        assert_eq!(stack.top(), None);

        stack.pop();
        assert_eq!(stack.top(), None);
    }

    #[test]
    fn test_push_and_pop() {
        let mut stack = Stack::default();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.top(), Some(3));
        stack.pop();
        assert_eq!(stack.top(), Some(2));
        stack.pop();
        assert_eq!(stack.top(), Some(1));
        stack.pop();
        assert_eq!(stack.top(), None);
        assert!(stack.is_empty());
    }
}
