#[cfg(test)]
mod sorting_tests {

    use rustic_dsa::algorithms::generate_list;
    use rustic_dsa::algorithms::sorting::heap::max_heap::max_heapsort;
    use rustic_dsa::algorithms::sorting::heap::min_heap::min_heapsort;
    use rustic_dsa::algorithms::sorting::{
        bubble::{iterative::iterative_bubble_sort, recursive::recursive_bubble_sort},
        merge::{iterative::iterative_merge_sort, recursive::recursive_merge_sort},
        quick::{iterative::iterative_quick_sort, recursive::recursive_quick_sort},
        SortFunction,
    };

    fn make_test(func: SortFunction<i32>) -> (Vec<i32>, Vec<i32>) {
        let mut random_list = generate_list(20);
        let mut random_list_copy = random_list.clone();
        random_list.sort();
        func(&mut random_list_copy);
        (random_list, random_list_copy)
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn iterative_bubble_test() {
        for _ in 0..10 {
            let (l1, l2) = make_test(iterative_bubble_sort);
            assert_eq!(l1, l2)
        }
    }

    #[test]
    fn recursive_bubble_test() {
        for _ in 0..10 {
            let (l1, l2) = make_test(recursive_bubble_sort);
            assert_eq!(l1, l2)
        }
    }

    #[test]
    fn iterative_quick_test() {
        for _ in 0..10 {
            let (l1, l2) = make_test(iterative_quick_sort);
            assert_eq!(l1, l2)
        }
    }

    #[test]
    fn recursive_quick_test() {
        for _ in 0..10 {
            let (l1, l2) = make_test(recursive_quick_sort);
            assert_eq!(l1, l2)
        }
    }

    #[test]
    fn iterative_merge_test() {
        for _ in 0..10 {
            let (l1, l2) = make_test(iterative_merge_sort);
            assert_eq!(l1, l2)
        }
    }

    #[test]
    fn recursive_merge_test() {
        for _ in 0..10 {
            let (l1, l2) = make_test(recursive_merge_sort);
            assert_eq!(l1, l2)
        }
    }

    #[test]
    fn max_heapsort_test() {
        for _ in 0..10 {
            let (l1, l2) = make_test(max_heapsort);
            assert_eq!(l1, l2)
        }
    }

    #[test]
    fn min_heapsort_test() {
        for _ in 0..10 {
            let (l1, l2) = make_test(min_heapsort);
            assert_eq!(l1, l2)
        }
    }
}
