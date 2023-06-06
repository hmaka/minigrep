use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn builder(args: Vec<String>) -> Result<Config, String> {
        if args.len() < 3 {
            let err = format!("Expected 2 arguments, got {}", args.len() - 1);
            return Err(err);
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Config { query, file_path });
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    return results;
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {} in {}", config.query, config.file_path);
    let file_content = fs::read_to_string(config.file_path.clone())?;
    for line in search(&config.query, &file_content) {
        println!("{}", line);
    }
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust: Safe, Fast, Productive.
        pick one.";

        assert_eq!(
            vec!["Rust: Safe, Fast, Productive."],
            search(query, contents)
        );
    }
}
