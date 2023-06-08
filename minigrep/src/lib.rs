use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &content) {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut arr = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            arr.push(line)
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "/
Rust:
safe, fast, production
        Pink three.";
        assert_eq!(vec!["safe, fast, production"], search(query, content));
    }
}
