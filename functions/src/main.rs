fn main() {
    another_function(5);
    println!("Five! {}", five());
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}
