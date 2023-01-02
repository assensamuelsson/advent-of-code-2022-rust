use std::{env, process};
use std::time::Instant;
use advent_of_code_2022_rust as lib;

fn main() {
    let args = env::args().collect();
    let config = lib::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    let start = Instant::now();
    if let Err(e) = lib::run(&config) {
        eprintln!("Runtime error: {}", e);
        process::exit(1);
    }
    let duration = start.elapsed().as_micros();
    
    println!("It took {} us", duration);
}
