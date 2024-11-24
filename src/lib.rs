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

pub mod algorithms;
pub mod running;
