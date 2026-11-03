use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filepath = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, filepath, ignore_case })
    } 
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {} in file {} with case sensitivity {}",
        config.query, config.filepath,
        if config.ignore_case == true { "disabled" } else { "enabled" });

    let file_contents = fs::read_to_string(config.filepath)?;

    let results = if config.ignore_case == false {
        search(&config.query, &file_contents)
    } else {
        search_case_insensitive(&config.query, &file_contents)
    };

    for result in results {
        println!("{result}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let query = "hello";

        let contents = "\nhello kitty\nyou're so pretty\nhalloween";

        assert_eq!(vec!["hello kitty"], search(query, contents));


    }

    #[test]
    fn test_case_sensitivity() {
        let query = "rust";
        let contents = "RuSt\nrUsTy\n";

        assert_eq!(vec!["RuSt", "rUsTy"], search_case_insensitive(query, contents));
    }

}

