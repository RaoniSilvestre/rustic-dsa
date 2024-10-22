use rusty_algoritms::running::*;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let list_lenght: usize = input.trim().parse().unwrap();

    merge::merge_run(list_lenght);
    quick::quick_run(list_lenght);
    bubble::iterative_bubble_run(list_lenght);
    bubble::recursive_bubble_run(list_lenght);
}
