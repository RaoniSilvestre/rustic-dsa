use crate::{data_structures::heap::MaxHeap, OrderedCopy};

pub fn heap_sort<T: OrderedCopy + Default>(array: &mut Vec<T>) {
    let heap: MaxHeap<T> = MaxHeap::from(std::mem::take(array));
    *array = MaxHeap::heapsort(heap);
}
