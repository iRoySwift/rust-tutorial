use add_one;

// cargo run -p adder
fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus add one is {}",
        num,
        add_one::add_one(num)
    );
}
