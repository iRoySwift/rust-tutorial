use std::{env, process};

use minigrep::Config;

fn main() {
    // 使用迭代器重写
    let config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1)
    });

    // IGNORE_CASE=1 cargo run -p minigrep -- who poem.txt
    if let Err(e) = minigrep::run(config) {
        eprint!("Application is_error: {}", e);
        process::exit(1);
    }
}
