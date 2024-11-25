use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustic_dsa::algorithms::sorting::{
    bubble::{iterative::iterative_bubble_sort, recursive::recursive_bubble_sort},
    merge::{iterative::iterative_merge_sort, recursive::recursive_merge_sort},
    quick::{iterative::iterative_quick_sort, recursive::recursive_quick_sort},
};
use rustic_dsa::read_numbers_from_file;

fn sorting_benchmarks(c: &mut Criterion) {
    let v1 = read_numbers_from_file("./data/input/file_1000.txt").unwrap();
    let v2 = read_numbers_from_file("./data/input/file_10000.txt").unwrap();
    let v3 = read_numbers_from_file("./data/input/file_100000.txt").unwrap();

    let cases = [("1k Array", v1), ("10k Array", v2), ("100k Array", v3)];

    for (case_name, data) in cases.iter() {
        let mut arr1 = data.clone();
        let mut arr2 = data.clone();
        let mut arr3 = data.clone();
        let mut arr4 = data.clone();
        let mut arr5 = data.clone();
        let mut arr6 = data.clone();

        c.bench_function(&format!("Recursive QuickSort: {}", case_name), |b| {
            b.iter(|| {
                black_box(recursive_quick_sort(&mut arr1));
            })
        });

        c.bench_function(&format!("Iterative Quicksort: {}", case_name), |b| {
            b.iter(|| {
                black_box(iterative_quick_sort(&mut arr2));
            })
        });

        c.bench_function(&format!("Recursive MergeSort: {}", case_name), |b| {
            b.iter(|| {
                black_box(recursive_merge_sort(&mut arr3));
            })
        });

        c.bench_function(&format!("Iterative MergeSort: {}", case_name), |b| {
            b.iter(|| {
                black_box(iterative_merge_sort(&mut arr4));
            })
        });

        c.bench_function(&format!("Recursive BubbleSort: {}", case_name), |b| {
            b.iter(|| {
                black_box(recursive_bubble_sort(&mut arr5));
            })
        });

        c.bench_function(&format!("Iterative BubbleSort: {}", case_name), |b| {
            b.iter(|| {
                black_box(iterative_bubble_sort(&mut arr6));
            })
        });
    }
}

criterion_group!(benches, sorting_benchmarks);
criterion_main!(benches);
