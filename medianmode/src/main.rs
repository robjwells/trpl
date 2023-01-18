use std::collections::HashMap;

fn main() {
    println!("Enter space-separated integers:");
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line.");

    println!("Your input: {:?}", buf);
    let integers: Vec<i32> = buf.split_whitespace().flat_map(str::parse).collect();
    println!("Numbers: {:?}", integers);

    if integers.is_empty() {
        println!("No numbers entered.");
        return;
    }

    let median = median(&integers);
    println!("Median: {median}");

    let mode = mode(&integers);
    println!("Mode: {mode}")
}

fn median(xs: &[i32]) -> f64 {
    let mut sorted = xs.to_vec();
    sorted.sort_unstable();

    let mid_idx = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        // Even-length sequence.
        let left = sorted[mid_idx - 1];
        let right = sorted[mid_idx];
        f64::from(left + right) / 2.0
    } else {
        // Odd-length sequence.
        f64::from(sorted[mid_idx])
    }
}

fn mode(xs: &[i32]) -> i32 {
    let mut counter = HashMap::new();
    for &x in xs {
        counter
            .entry(x)
            .and_modify(|count| *count += 1)
            .or_insert(1u32);
    }
    let highest_frequency = *counter.values().max().unwrap();
    counter
        .into_iter()
        .filter(|&(_, v)| v == highest_frequency)
        .map(|(k, _)| k)
        .next()
        .unwrap()
}
