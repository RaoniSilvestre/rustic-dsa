use std::time::Duration;

use crate::{generate_list, sorting::quick_sort::QuickSort};

pub fn quick_run(list_lenght: usize) {
    let mut list = generate_list(list_lenght.clone());
    let elapsed_time = quick_sort_time(&mut list);
    println!("Quick Sort Timing : {:.2?}", elapsed_time);
}

fn quick_sort_time(lista: &mut Vec<i32>) -> Duration {
    let now = std::time::Instant::now();
    lista.quick_sort();
    now.elapsed()
}
