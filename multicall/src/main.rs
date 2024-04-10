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

    let input_file = matches.value_of("input").unwrap().to_string();

    match day {
        1 => {
            let config = day1::Config::from_filename(input_file);
            if let Err(err) = day1::run(config) {
                println!("Something went wrong reading the file: {}", err);
                process::exit(1);
            };
        },
        2 => {
            day2::run(input_file);
        },
        3 => {
            day3::run(input_file);
        },
        4 => {
            day4::run(input_file);
        },
        5 => {
            day5::run(input_file);
        },
        6 => {
            day6::run(input_file);
        },
        7 => {
            day7::run(input_file);
        },
        8 => {
            day8::run(input_file);
        },
        9 => {
            day9::run(input_file);
        },
        10 => {
            day10::run(input_file);
        },
        _ => {
            println!("Day {} is not implemented", day);
            process::exit(1);
        }
    }
}
