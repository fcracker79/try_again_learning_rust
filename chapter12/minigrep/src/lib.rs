use std::error::Error;
use std::{env, fs};
mod playing_with_mut;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("Wrong number of arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_insensitive = env::var("CASE_INSENSITIVE").is_ok();
        Ok(Config { query, filename, case_insensitive })
    }

    fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }

        results
    }

    fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();
        let query = query.to_lowercase();
        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }

        results
    }
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(&self.filename)?;
        // println!("With text:\n{}", contents);
        let result = if self.case_insensitive
                {Config::search_case_insensitive(&self.query, &contents)}
            else
                {Config::search(&self.query, &contents)};
        println!("Result:\n{:?}", result);
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], Config::search(query, contents));
    }
}