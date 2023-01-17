fn main() {
    let mut _s = String::new();

    let data = "initial contents";
    let _s = data.to_string();
    let _s = "initial contents".to_string();

    let _s = String::from("initial contents");

    let _emoji = String::from("Hello 🦀!");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    // let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
}
