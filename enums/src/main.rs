fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    route(&home);
    let loopback = IpAddr::V6("::1".into());
    route(&loopback);
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip: &IpAddr) {
    println!("{:?}", ip);
}
