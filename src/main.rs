use std::thread;
use std::time::Duration;

use rusty_algoritms::generate_list;
use rusty_algoritms::sorting::bubble_sort::BubbleSort;
use rusty_algoritms::sorting::merge_sort::MergeSort;
use rusty_algoritms::sorting::quick_sort::QuickSort;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let list_lenght: i32 = input.trim().parse().unwrap();

    let bubble_thread = thread::spawn(move || {
        let mut list = generate_list(list_lenght.clone());
        let elapsed_time = bubble_sort_time(&mut list);
        println!("Bubble Sort Timing: {:.2?}", elapsed_time);
    });

    let quick_thread = thread::spawn(move || {
        let mut list = generate_list(list_lenght.clone());
        let elapsed_time = quick_sort_time(&mut list);
        println!("Quick Sort Timing : {:.2?}", elapsed_time);
    });
    let merge_thread = thread::spawn(move || {
        let mut list = generate_list(list_lenght.clone());
        let elapsed_time = merge_sort_time(&mut list);
        println!("Merge Sort Timing : {:.2?}", elapsed_time);
    });

    bubble_thread.join().unwrap();
    quick_thread.join().unwrap();
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
