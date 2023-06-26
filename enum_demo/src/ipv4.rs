use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum IpvAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpvAddrS {
    kind: IpvAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpvAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

pub fn run() {
    let home1 = IpvAddrS {
        kind: IpvAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback1 = IpvAddrS {
        kind: IpvAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpvAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let loopback = IpvAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    println!("home1 = {:?}", home1);
    println!("loopback1 = {:?}", loopback1);
    println!("home = {:?}", home);
    println!("loopback = {:?}", loopback);
}
