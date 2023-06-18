#[derive(Debug)]
enum IpAddrkind {
    V4,
    V6,
}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrkind,
    address: String,
}
#[derive(Debug)]
enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    println!("Hello, world!");
    let home = IpAddr {
        kind: IpAddrkind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrkind::V6,
        address: String::from("::1"),
    };

    println!("{:?}{}{:?}", home, home.address, home.kind);
    println!("{:?}", loopback);

    let home1 = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback1 = IpAddrEnum::V6(String::from("::1"));
    println!("{:?}", home1);
    println!("{:?}", loopback1);
}
