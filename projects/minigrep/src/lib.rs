use std::{env, error::Error, fs};

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("没有足够的参数!");
        // }
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("没有获取到query"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("没有获取到filename"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    let lines = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    for line in lines {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // let mut vec = Vec::new();
    // for line in content.lines() {
    //     if line.contains(query) {
    //         vec.push(line)
    //     }
    // }
    // vec
    // 使用迭代器适配器来使代码更简明
    content
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // let mut vec = Vec::new();
    // let query = query.to_lowercase();
    // for line in content.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         vec.push(line);
    //     }
    // }
    // vec
    // 使用迭代器适配器来使代码更简明
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_sensitive() {
        let query = "duct";
        let content = "
Rust:
safe, fast, production
    Pink three.";
        assert_eq!(vec!["safe, fast, production"], search(query, content));
    }

    // cargo test -p minigrep test_case_insensitive
    #[test]
    fn test_case_insensitive() {
        let query = "duct";
        let content = "
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
