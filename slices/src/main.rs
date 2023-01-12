fn main() {
    let s = String::from("hello world");
    let hello = first_word(&s);
    println!("{:?}, {:?}", s, hello);
}

fn first_word(s: &str) -> &str {
    s.find(' ').map(|idx| &s[..idx]).unwrap_or(s)
}
