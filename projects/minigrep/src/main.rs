use std::{env, process};

use minigrep::Config;

fn main() {
    // 使用迭代器重写
    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1)
    });

    // cargo run -p minigrep -- to poem.txt > output.txt
    // IGNORE_CASE=1 cargo run -p minigrep -- who poem.txt
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application is_error: {}", e);
        process::exit(1);
    }
}
