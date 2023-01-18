fn main() {
    let mut buf = String::new();
    println!("Enter your phrase:");
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line.");

    let phrase: String = buf
        .split_whitespace()
        .map(pig_latinise)
        .collect::<Vec<_>>()
        .join(" ");
    println!("> {}", phrase);
}

fn pig_latinise(word: &str) -> String {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    if word.is_empty() {
        return word.to_string();
    }

    let mut chars = word.chars();
    let first = chars.next().unwrap().to_ascii_lowercase();
    let rest: String = chars.collect();

    if VOWELS.contains(&first) {
        format!("{}-hay", word)
    } else {
        format!("{}-{}ay", rest, first)
    }
}
