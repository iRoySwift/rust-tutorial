use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
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

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut arr = Vec::new();
    let query = query.to_lowercase();
    println!("{}", query);
    for line in content.lines() {
        // println!("{}", line.to_lowercase());
        if line.to_lowercase().contains(&query) {
            arr.push(line)
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "/
Rust:
safe, fast, production
        Pink three.";
        assert_eq!(vec!["safe, fast, production"], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let content = "/
Rust:
safe, fast, production
Duct
        Pink three.";
        assert_eq!(
            vec!["safe, fast, production", "Duct"],
            search_case_insensitive(query, content)
        );
    }
}
