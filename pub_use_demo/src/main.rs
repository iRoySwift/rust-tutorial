// use pub_use_demo::{kind::PrimaryColor, utils::mix};

use pub_use_demo::{mix, PrimaryColor};

fn main() {
    let yellow = PrimaryColor::Yellow;
    let red = PrimaryColor::Red;
    mix(yellow, red);
}
