#[cfg(test)]
mod queue_tests {
    use rustic_dsa::data_structures::queue::Queue;

    #[test]
    fn test_push_and_front() {
        let mut queue: Queue<i32> = Queue::default();

        assert!(queue.is_empty());

        queue.push(10);

        assert_eq!(queue.front(), Some(10));
        assert!(!queue.is_empty());
    }

    #[test]
    fn test_push_and_back() {
        let mut queue: Queue<i32> = Queue::default();

        queue.push(10);
        queue.push(20);

        assert_eq!(queue.back(), Some(20));
    }

    #[test]
    fn test_pop() {
        let mut queue: Queue<i32> = Queue::default();

        queue.push(10);
        queue.push(20);

        assert_eq!(queue.front(), Some(10));

        queue.pop();

        assert_eq!(queue.front(), Some(20));

        assert_eq!(queue.back(), Some(20));

        assert!(!queue.is_empty());

        queue.pop();

        assert!(queue.is_empty());
    }

    #[test]
    fn test_pop_empty_queue() {
        let mut queue: Queue<i32> = Queue::default();

        queue.pop();

        assert!(queue.is_empty());
    }

    #[test]
    fn test_back_empty_queue() {
        let queue: Queue<i32> = Queue::default();

        assert_eq!(queue.back(), None);
    }

    #[test]
    fn test_front_empty_queue() {
        let queue: Queue<i32> = Queue::default();

        assert_eq!(queue.front(), None);
    }

    #[test]
    fn test_multiple_push_pop() {
        let mut queue: Queue<i32> = Queue::default();

        queue.push(10);
        queue.push(20);
        queue.push(30);

        queue.pop();
        assert_eq!(queue.front(), Some(20));

        queue.pop();
        assert_eq!(queue.front(), Some(30));

        queue.pop();
        assert!(queue.is_empty());
    }
}
