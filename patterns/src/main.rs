#![allow(clippy::vec_init_then_push)]

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({:?}, {:?})", x, y);
}

fn main() {
    let favourite_colour: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(colour) = favourite_colour {
        println!("Using your favourite color, {colour}, as the background");
    } else if is_tuesday {
        println!("Tuesday is a green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background colour");
        } else {
            println!("Using orange as the background colour");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let point = (1, 2);
    print_coordinates(&point);

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);

    let x = Some(5);
    let y = 5;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, y = n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            );
        }
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("{feet}ft {inches}in at ({x}, {y})");

    let s = Some(String::from("Hello!"));
    // if let Some(_) = s {
    //     println!("Found a string!");
    // }
    println!("{:?}", s);

    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("{x} is even"),
        Some(x) => println!("{x} is odd"),
        None => {}
    }

    let msg = NewMessage::Hello { id: 5 };
    match msg {
        NewMessage::Hello { id: id_var @ 3..=7 } => println!("Found an id in range: {}", id_var),
        NewMessage::Hello { id: 10..=12 } => println!("Found an id in another range"),
        NewMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}

enum NewMessage {
    Hello { id: i32 },
}

struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
