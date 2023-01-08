use std::io;

fn main() {
    println!("Enter a temperature in Fahrenheit (42F) or Celsius (21C):");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Could not read line.");

    let temp = temp.trim().replace(' ', "");
    let number: f32 = temp[..temp.len() - 1]
        .parse()
        .expect("Could not parse as a number.");

    match temp.chars().last().unwrap() {
        'C' => {
            println!("{number}°C is {:.1}°F", c_to_f(number));
        }
        'F' => {
            println!("{number}°F is {:.1}°C", f_to_c(number));
        }
        suffix => {
            println!("Unknown suffix: {suffix}");
        }
    }
}

fn c_to_f(c: f32) -> f32 {
    c * 1.8 + 32.0
}

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) * (5.0 / 9.0)
}
