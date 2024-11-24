pub mod sorting;

use rand::Rng;

pub fn generate_list(n: usize) -> Vec<i32> {
    let mut list: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        list.push(rng.gen_range(0..1000));
    }
    list
}
