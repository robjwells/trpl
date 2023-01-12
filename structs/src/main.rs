fn main() {
    let u = build_user(
        String::from("user@example.com"),
        String::from("someuser123"),
    );
    print_user(&u);

    let u2 = User {
        email: String::from("another@example.com"),
        ..u
    };
    print_user(&u2);
    println!("{}", u.email);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn print_user(u: &User) {
    println!(
        "{} <{}> -- active: {}, sign-ins: {}",
        u.username, u.email, u.active, u.sign_in_count
    );
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
