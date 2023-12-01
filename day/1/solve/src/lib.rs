use std::error::Error;
use std::fs;

pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough command-line arguments");
        }

        let filename = args[1].clone();
        Ok(Config { filename })
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

pub fn calibrate(content: &str) -> Vec<u32> {
    let mut result = vec![];
    for line in content.lines() {
        let l = line.trim_matches(char::is_alphabetic);
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
    fn example() {
        let content = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(
            vec![12, 38, 15, 77],
            calibrate(content)
        );
    }
}
