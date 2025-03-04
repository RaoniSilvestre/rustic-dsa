use crate::{data_structures::heap::MaxHeap, OrderedCopy};

pub fn max_heapsort<T: OrderedCopy + Default>(array: &mut [T]) {
    let heap: MaxHeap<T> = MaxHeap::from(array.to_vec());
    array.copy_from_slice(&MaxHeap::heapsort(heap));
}
