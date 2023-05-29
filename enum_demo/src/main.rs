#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrKind2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(&four);
    route(&six);
    route(&IpAddrKind::V4);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // enum 不依赖struct
    let home2 = IpAddrKind2::V4(127, 0, 0, 1);
    let loopback2 = IpAddrKind2::V6(String::from("::1"));

    let q = Message::Quit;
    let m = Message::Move { x: 1, y: 2 };
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(1, 2, 3);
    q.call();
}

fn route(ip_kind: &IpAddrKind) {
    println!("{:?}", ip_kind);
}
