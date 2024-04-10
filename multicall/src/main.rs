use clap::{App, Arg};
use std::process;

fn main() {
    let matches = App::new("Multicall")
        .arg(Arg::with_name("day")
            .long("day")
            .takes_value(true)
            .required(true)
            .help("Specifies the day to run")
            .validator(|v| v.parse::<u32>().map_err(|e| e.to_string())))
        .arg(Arg::with_name("input")
            .takes_value(true)
            .required(true)
            .help("Input file"))
        .get_matches();

    let day: u32 = matches.value_of("day")
        .and_then(|d| d.parse().ok())
        .unwrap_or_else(|| {
            println!("Invalid day specified");
            process::exit(1);
        });

    let input_file = matches.value_of("input").unwrap();

    match day {
        1 => {
            let config = day1::Config::from_filename(input_file.to_string());
            if let Err(err) = day1::run(config) {
                println!("Something went wrong reading the file: {}", err);
                process::exit(1);
            };
        },
        2 => {
            // Handle day 2
        },
        _ => {
            println!("Day {} is not implemented", day);
            process::exit(1);
        }
    }
}
