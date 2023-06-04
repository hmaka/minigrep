use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {} in {}", query, file_path);
    let file_content =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("File text:\n{file_content}");
}
