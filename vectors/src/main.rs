use std::vec;

#[allow(clippy::vec_init_then_push)]
fn main() {
    let _v: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3];

    let mut _v = Vec::new();
    _v.push(5);
    _v.push(6);
    _v.push(7);
    _v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {third}");

    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![100, 32, 57];
    for n_ref in &v {
        let n_plus_one = *n_ref + 1;
        println!("{}", n_plus_one);
    }
    println!("{:?}", v);

    let mut v = vec![100, 32, 57];
    for n_ref in &mut v {
        *n_ref += 50;
    }
    println!("{:?}", v);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for cell in &row {
        println!("{:?}", cell);
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
