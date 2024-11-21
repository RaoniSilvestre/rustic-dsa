use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn read_numbers_from_file(file_path: &str) -> Result<Vec<i32>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let numbers: Vec<i32> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .collect();

    Ok(numbers)
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
