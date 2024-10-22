use std::time::Duration;

use crate::{generate_list, sorting::merge_sort::MergeSort};

pub fn merge_run(list_lenght: usize) {
    let mut list = generate_list(list_lenght.clone());
    let elapsed_time = merge_sort_time(&mut list);
    println!("Merge Sort Timing : {:.2?}", elapsed_time);
}

fn merge_sort_time(lista: &mut Vec<i32>) -> Duration {
    let now = std::time::Instant::now();
    lista.merge_sort();
    now.elapsed()
}
