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
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let mut ignore_case_flag = false;

        let query;
        let file_path;
        if args.len() == 4 {
            let flag = args[1].clone();
            query = args[2].clone();
            file_path = args[3].clone();

            if flag != "--ignore-case" {
                return Err("Unknown flag");
            }

            ignore_case_flag = true;
        } else {
            query = args[1].clone();
            file_path = args[2].clone();
        }

        let ignore_case_env = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case: ignore_case_flag || ignore_case_env,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line);
        }
    }

    result
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
