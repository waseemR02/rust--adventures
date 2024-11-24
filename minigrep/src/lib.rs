use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build<T: Iterator<Item = String>>(mut args: T) -> Result<Config, &'static str> {
        args.next();

        Ok(Self {
            query: match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string."),
            },
            filepath: match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a filepath"),
            },
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let results = contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let results = contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect();

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
