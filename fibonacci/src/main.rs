use std::io;

fn main() {
    println!("Which Fibonacci number would you like?");

    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line.");
    let n: u32 = buf.trim().parse().expect("Failed to parse number.");

    if n < 2 {
        println!("#{n} = {n}");
        return;
    };

    let mut a: u128 = 0;
    let mut b: u128 = 1;
    for _ in 2..=n {
        let temp = a;
        a = b;
        b += temp;
    }
    println!("#{n} = {b}");
}
