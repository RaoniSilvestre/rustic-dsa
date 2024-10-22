use super::{IterativeBubbleSort, OrderedCopy, RecursiveBubbleSort};

impl<T: OrderedCopy> IterativeBubbleSort<T> for Vec<T> {
    fn iterative_bubble_sort(&mut self) {
        let len = self.len();
        let mut is_ordered = true;

        for i in 0..len {
            for j in 0..len - 1 - i {
                if self[j] > self[j + 1] {
                    is_ordered = false;
                    self.swap(j, j + 1)
                }
            }

            if is_ordered {
                break;
            }
        }
    }
}

impl<T: OrderedCopy> RecursiveBubbleSort<T> for Vec<T> {
    fn recursive_bubble_sort(&mut self) {
        let len = self.len();
        recursive_bubble(self, len)
    }
}

fn recursive_bubble<T: OrderedCopy>(vec: &mut Vec<T>, n: usize) {
    if n == 1 {
        return;
    }
    bubble_pass(vec, 0, n);
    recursive_bubble(vec, n - 1);
}

fn bubble_pass<T: OrderedCopy>(vec: &mut Vec<T>, i: usize, j: usize) {
    if i == j - 1 {
        return;
    }

    if vec[i] > vec[i + 1] {
        vec.swap(i, i + 1);
    }

    bubble_pass(vec, i + 1, j);
}
