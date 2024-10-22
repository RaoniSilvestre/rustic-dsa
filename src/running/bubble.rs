use std::time::Duration;

use crate::{
    generate_list,
    sorting::{IterativeBubbleSort, RecursiveBubbleSort},
};

pub fn iterative_bubble_run(list_lenght: usize) {
    let mut list = generate_list(list_lenght.clone());
    let elapsed_time = iterative_bubble_sort_time(&mut list);
    println!("Iterative BubbleSort Timing: {:.2?}", elapsed_time);
}

fn iterative_bubble_sort_time(lista: &mut Vec<i32>) -> Duration {
    let now = std::time::Instant::now();
    lista.iterative_bubble_sort();
    now.elapsed()
}

pub fn recursive_bubble_run(list_lenght: usize) {
    let mut list = generate_list(list_lenght.clone());
    let elapsed_time = recursive_bubble_sort_time(&mut list);
    println!("Recursive BubbleSort Timing: {:.2?}", elapsed_time);
}

fn recursive_bubble_sort_time(lista: &mut Vec<i32>) -> Duration {
    let now = std::time::Instant::now();
    lista.recursive_bubble_sort();
    now.elapsed()
}
