use crate::{data_structures::heap::MinHeap, OrderedCopy};

pub fn min_heapsort<T: OrderedCopy + Default>(array: &mut [T]) {
    let heap: MinHeap<T> = MinHeap::from(array.to_vec());
    array.copy_from_slice(&MinHeap::heapsort(heap));
}
