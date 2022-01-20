use std::env;
use std::error::Error;
use std::fs;
use std::process;
use minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}\nUsage: {} <query> <filename>", err, &args[0]);
        process::exit(1);
    });
    println!("Searching for `{}`", config.query);
    println!("In file {}", config.filename);
    if let Err(e) = config.run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}