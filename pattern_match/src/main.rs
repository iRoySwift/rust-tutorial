mod for_pattern;
mod let_if;
mod pattern_reg;
mod while_let;

fn main() {
    println!("Hello, world!");
    let_if::run();
    while_let::run();
    for_pattern::run();
    pattern_reg::run();
}
