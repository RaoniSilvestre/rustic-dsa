use rusty_algoritms::generate_list;
use rusty_algoritms::sorting::quick_sort::QuickSort;

fn main() {
    let now = std::time::Instant::now();

    let _binding = generate_list(20).quick_sort();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed)
}
