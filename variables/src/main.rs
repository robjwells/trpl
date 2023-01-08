use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");

    let t = ([1; 2], [3; 4]);
    let (f, _) = t;
    println!("{} {}", t.0[0], f[0] + t.1[0]);
}
