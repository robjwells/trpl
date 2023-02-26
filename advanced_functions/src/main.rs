fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let xs = vec![1, 2, 3];
    let _strings: Vec<String> = xs.iter().map(|i| i.to_string()).collect();

    let xs = vec![1, 2, 3];
    let _strings: Vec<String> = xs.iter().map(ToString::to_string).collect();

    let _statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    let _closure = returns_closure();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[allow(dead_code)]
enum Status {
    Value(u32),
    Stop,
}

/// `Box<dyn trait>` returns any value that implements the trait,
/// behind a pointer (the box).
///
/// `impl trait` on the other hand returns _one_ concrete type,
/// but that type is not known to the caller.
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
