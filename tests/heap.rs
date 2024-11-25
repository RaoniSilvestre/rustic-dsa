#[cfg(test)]
mod tests {
    use rustic_dsa::data_structures::heap::MaxHeap;

    #[test]
    fn test_new_heap_is_empty() {
        let heap: MaxHeap<i32> = MaxHeap::default();
        assert!(heap.is_empty());
    }

    #[test]
    fn test_push_and_peek() {
        let mut heap = MaxHeap::default();
        heap.push(10);
        heap.push(20);
        heap.push(15);
        assert_eq!(heap.peek(), Some(&20));
    }

    #[test]
    fn test_pop() {
        let mut heap = MaxHeap::default();
        heap.push(10);
        heap.push(20);
        heap.push(15);

        assert_eq!(heap.pop(), Some(20));
        assert_eq!(heap.pop(), Some(15));
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), None);
    }
    #[test]
    fn test_heapify() {
        let vec = vec![3, 9, 2, 1, 4, 5];
        let heap = MaxHeap::from(vec);

        for i in 0..heap.data.len() {
            let left_child = 2 * i + 1;
            let right_child = 2 * i + 2;

            if left_child < heap.data.len() {
                assert!(heap.data[i] >= heap.data[left_child]);
            }

            if right_child < heap.data.len() {
                let x = heap.data[i];
                let y = heap.data[right_child];
                debug_assert!(
                    x >= y,
                    "{}",
                    format!(
                        "data[{}] = {}\ndata[{}] = {}\n {} >= {}",
                        i, x, right_child, y, x, y
                    )
                );
            }
        }
    }

    #[test]
    fn test_len_and_is_empty() {
        let mut heap = MaxHeap::default();
        assert_eq!(heap.len(), 0);
        assert!(heap.is_empty());

        heap.push(30);
        heap.push(40);
        assert_eq!(heap.len(), 2);
        assert!(!heap.is_empty());
    }

    #[test]
    fn test_push_pop_sequence() {
        let mut heap = MaxHeap::default();
        heap.push(50);
        heap.push(60);
        heap.push(40);
        heap.push(30);

        // Elementos removidos devem ser na ordem de prioridade
        assert_eq!(heap.pop(), Some(60));
        assert_eq!(heap.pop(), Some(50));
        assert_eq!(heap.pop(), Some(40));
        assert_eq!(heap.pop(), Some(30));
    }
}
