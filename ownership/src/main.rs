fn main() {
    let s1 = String::from("hello");
    let (mut s1, len) = take_and_return(s1);
    println!("The length of {:?} is {}.", s1, len);

    let len = borrow(&s1);
    println!("The length of {:?} is {}.", s1, len);

    change(&mut s1);
    let len = borrow(&s1);
    println!("The length of {:?} is {}.", s1, len);
}

fn take_and_return(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn borrow(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
