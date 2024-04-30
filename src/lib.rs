use rand::Rng;

pub fn generate_list(n: usize) -> Vec<i32> {
    let mut list: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        list.push(rng.gen_range(0..(n as i32)));
    }
    list
}

pub trait IsSorted<T: PartialOrd> {
    fn is_sortted(&self) -> bool;
}

impl<T> IsSorted<T> for Vec<T>
where
    T: PartialOrd,
{
    fn is_sortted(&self) -> bool {
        for i in 1..self.len() {
            if self[i - 1] > self[i] {
                return false;
            }
        }

        true
    }
}

pub mod running;
pub mod sorting;
pub mod structs;
