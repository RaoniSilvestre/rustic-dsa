use rustic_dsa::data_structures::queue::Queue;

fn main() {
    let mut q: Queue<i32> = Queue::default();

    println!("{q}");
    q.push(3);
    println!("{q}");
    q.push(2);
    println!("{q}");
    q.push(1);
    println!("{q}");
    q.push(8);
    println!("{q}");
    q.push(9);
    println!("{q}");
    q.push(6);
    println!("{q}");
    q.pop();
    println!("{q}");
    q.pop();
    println!("{q}");
    q.pop();
    println!("{q}");
    q.pop();
    println!("{q}");
    q.pop();
    println!("{q}");
}
