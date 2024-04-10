use std::env;
use std::process;

use day1::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = day1::run(config) {
        println!("Something went wrong reading the file: {}", err);
        process::exit(1);
    };
}
