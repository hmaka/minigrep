use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn builder(args: Vec<String>) -> Result<Config, String> {
        if args.len() < 3 {
            let err = format!("Expected 2 arguments, got {}", args.len() - 1);
            return Err(err);
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Config { query, file_path });
    }
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let config = Config::builder(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.file_path);
    let file_content =
        fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

    println!("File text:\n{file_content}");
}
