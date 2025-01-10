use std::ops::{Index, IndexMut};

pub mod max_heap;
pub mod min_heap;

#[derive(Debug, Default)]
pub struct MaxHeap<T> {
    pub data: Vec<T>,
}

impl<T: Ord> MaxHeap<T> {
    pub fn heapsort(mut self) -> Vec<T> {
        let mut sorted = Vec::with_capacity(self.len());

        while let Some(max) = self.pop() {
            sorted.push(max);
        }

        sorted.reverse();

        sorted
    }
}

impl<T: Default + Ord> From<Vec<T>> for MaxHeap<T> {
    fn from(data: Vec<T>) -> Self {
        let mut heap = MaxHeap { data };
        let len = heap.data.len();
        for i in (0..len / 2).rev() {
            heap.bubble_down(i);
        }
        heap
    }
}

impl<T> Index<usize> for MaxHeap<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for MaxHeap<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Ord> MaxHeap<T> {
    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.bubble_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let last_index = self.data.len() - 1;
        self.data.swap(0, last_index);
        let max_value = self.data.pop();
        self.bubble_down(0);
        max_value
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }

    pub fn bubble_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.data[index] <= self.data[parent] {
                break;
            }

            self.data.swap(index, parent);
            index = parent;
        }
    }

    pub fn bubble_down(&mut self, mut index: usize) {
        let last_index = match self.data.len() {
            0 => 0,
            n => n - 1,
        };
        loop {
            let left_child = (2 * index) + 1;
            let right_child = (2 * index) + 2;
            let mut largest = index;

            if left_child <= last_index && self.data[left_child] > self.data[largest] {
                largest = left_child;
            }

            if right_child <= last_index && self.data[right_child] > self.data[largest] {
                largest = right_child;
            }

            if largest == index {
                break;
            }

            self.data.swap(index, largest);
            index = largest;
        }
    }
}
