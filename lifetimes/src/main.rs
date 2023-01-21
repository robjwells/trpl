fn main() {
    let s1 = String::from("abcd");
    let s2 = "xyz".to_owned();
    let result = longest(s1.as_str(), s2.as_str());
    println!("The longest string is {}", result);

    let novel = "Call me Ishmael. Some years ago...".to_owned();
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("{}", i.part);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 { 3 }
}
