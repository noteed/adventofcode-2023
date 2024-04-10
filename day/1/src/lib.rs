use std::error::Error;
use std::fs;

pub struct Config {
    filename: String,
}

impl Config {
    // Parse args. Used in e.g. day/1/src/main.rs.
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough command-line arguments");
        }

        let filename = args[1].clone();
        Ok(Config { filename })
    }

    // Take directly a filename. Used in the multicall binary.
    pub fn from_filename(filename: String) -> Config {
        Config { filename }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    println!("Opening file '{}'...", config.filename);
    let content = fs::read_to_string(config.filename)?;

    let numbers = calibrate(&content);
    let sum: u32 = numbers.iter().sum();
    println!("Sum: {}", sum);
    Ok(())
}

pub fn replace(line: &str) -> String {
    // Hack: we keep the first and last letter so that overlapping words
    // can be detected after a replacement.
    line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

pub fn calibrate(content: &str) -> Vec<u32> {
    let mut result = vec![];
    for line in content.lines() {
        let replaced = replace(line);
        let l = replaced.trim_matches(char::is_alphabetic);
        let s = l.chars().nth(0).unwrap().to_string()
              + &l.chars().last().unwrap().to_string();
        let n = s.parse().unwrap();
        result.push(n);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_calibrate() {
        let content = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(
            vec![29, 83, 13, 24, 42, 14, 76],
            calibrate(content)
        );
    }

    #[test]
    fn example_replace() {
        let line = "abcone2threexyz";

        assert_eq!(
            "abco1e2t3exyz",
            replace(line)
        );
    }
}
