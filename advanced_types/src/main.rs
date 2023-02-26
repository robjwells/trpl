fn main() {
    println!("Hello, world!");

    let x: i32 = 5;
    let y: Kilometres = 5;

    assert_eq!(x, y);

    println!("{:?}", is_equal("Hello", "world"));
}

type Kilometres = i32;

fn is_equal<T>(t1: &T, t2: &T) -> bool
where
    T: Eq + ?Sized,
{
    t1 == t2
}
