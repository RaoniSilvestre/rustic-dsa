use rusty_algoritms::{read_numbers_from_file, running::*};

fn main() {
    let files = vec![
        "./data/input/file_1000.txt",
        "./data/input/file_10000.txt",
        "./data/input/file_100000.txt",
    ];

    for file in files {
        let array = read_numbers_from_file(file).expect("Não foi possível ler arquivo");
        run_array(array);
    }
}
