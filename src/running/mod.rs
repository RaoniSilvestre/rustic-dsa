use crate::generate_list;
use crate::sorting::bubble_sort::BubbleSort;
use crate::sorting::merge_sort::MergeSort;
use crate::sorting::quick_sort::QuickSort;
use std::thread;
use std::time::Duration;
pub fn bubble_run(list_lenght: usize) {
    let bubble_thread = thread::spawn(move || {
        let mut list = generate_list(list_lenght.clone());
        let elapsed_time = bubble_sort_time(&mut list);
        println!("Bubble Sort Timing: {:.2?}", elapsed_time);
    });

    bubble_thread.join().unwrap();
}

pub fn quick_run(list_lenght: usize) {
    let quick_thread = thread::spawn(move || {
        let mut list = generate_list(list_lenght.clone());
        let elapsed_time = quick_sort_time(&mut list);
        println!("Quick Sort Timing : {:.2?}", elapsed_time);
    });

    quick_thread.join().unwrap();
}
pub fn merge_run(list_lenght: usize) {
    let merge_thread = thread::spawn(move || {
        let mut list = generate_list(list_lenght.clone());
        let elapsed_time = merge_sort_time(&mut list);
        println!("Merge Sort Timing : {:.2?}", elapsed_time);
    });
    merge_thread.join().unwrap();
}
fn merge_sort_time(lista: &mut Vec<i32>) -> Duration {
    let now = std::time::Instant::now();
    lista.merge_sort();
    now.elapsed()
}

fn bubble_sort_time(lista: &mut Vec<i32>) -> Duration {
    let now = std::time::Instant::now();
    lista.bubble_sort();
    now.elapsed()
}

fn quick_sort_time(lista: &mut Vec<i32>) -> Duration {
    let now = std::time::Instant::now();
    lista.quick_sort();
    now.elapsed()
}
