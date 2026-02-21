use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    if config.ignore_case {
        for line in insensitive_search(&config.query, &contents) {
            println!("{line}")
        }
    } else {
        for line in search(&config.query, &contents) {
            println!("{line}")
        }
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let mut args = args.peekable();
        args.next();

        let ignore_case_flag: bool = match args.peek() {
            Some(x) => {
                let is_ignore_case = x == "--ignore_case";
                if is_ignore_case {
                    args.next();
                }
                is_ignore_case
            }
            None => return Err("No arguments provided"),
        };

        let query = match args.next() {
            Some(x) => x,
            None => return Err("Failed to provide a query argument"),
        };

        let file_path = match args.next() {
            Some(x) => x,
            None => return Err("Failed to provide a file path argument`"),
        };

        let ignore_case_env = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case: ignore_case_flag || ignore_case_env,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|x| (*x).contains(query)).collect()
}

pub fn insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|x| (*x.to_lowercase()).contains(&query.to_ascii_lowercase()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sensitive_case() {
        let query = "duct";
        let contents = "/
Rust:
safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn insensitive_case() {
        let query = "RuSt";
        let contents = "/
Rust:
safe, fast, productive.
Pick three.
rust is fun!";

        assert_eq!(
            vec!["Rust:", "rust is fun!"],
            insensitive_search(query, contents)
        );
    }
}
