use std::env;
use std::process;

use rust_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
     
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}\n\n", config.file_path);

    if let Err(e) = rust_grep::run(config) {
        println!("Error: {e}");
        process::exit(1);
    };
}
