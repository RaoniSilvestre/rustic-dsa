use rusty_algoritms::running::*;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let list_lenght: usize = input.trim().parse().unwrap();

    merge_run(list_lenght);
    quick_run(list_lenght);
    bubble_run(list_lenght)
}
