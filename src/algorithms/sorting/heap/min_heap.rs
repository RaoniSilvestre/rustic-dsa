use crate::{data_structures::heap::min_heap::MinHeap, OrderedCopy};

pub fn min_heapsort<T: OrderedCopy + Default>(array: &mut Vec<T>) {
    let heap: MinHeap<T> = MinHeap::from(std::mem::take(array));
    *array = MinHeap::heapsort(heap);
}
