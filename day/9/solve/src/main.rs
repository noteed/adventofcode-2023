use nom::{
  bytes::complete::tag,
  bytes::complete::take_while1,
  character::complete::alphanumeric1,
  character::complete::multispace0,
  character::complete::space0,
  combinator::map_res,
  combinator::opt,
  multi::many1,
  sequence::delimited,
  sequence::terminated,
  Finish,
  IResult,
  Parser,
};
use std::collections::HashMap;
use std::fs;

fn main() {
    //let content = fs::read_to_string("../example.txt").expect("XXX");
    let content = fs::read_to_string("../input.txt").expect("XXX");
    let result = parse_report(&content).finish();
    match result {
        Ok((_, report)) => {
            let mut sum = 0;
            for nbrs in &report.lines {
                println!("Line: {:?}", nbrs);
                let extra = extrapolate(&nbrs);
                println!("Extrapolate: {:?}", extra);
                sum += extra;
            }
            println!("Part one: {}", sum);

            let mut sum = 0;
            for nbrs in &report.lines {
                let mut nbrs = nbrs.clone();
                nbrs.reverse(); // Same as part one, but extrapolate backward.
                println!("Line: {:?}", nbrs);
                let extra = extrapolate(&nbrs);
                println!("Extrapolate: {:?}", extra);
                sum += extra;
            }
            println!("Part two: {}", sum);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

#[derive(Debug)]
pub struct Report {
    lines: Vec<Vec<i64>>,
}

fn extrapolate(nbrs: &Vec<i64>) -> i64 {
    if nbrs.iter().all(|nbr| *nbr == 0) {
        0
    } else {
        let diffs = differences(nbrs);
        let extra = extrapolate(&diffs);
        nbrs[nbrs.len() - 1] + extra
    }
}

fn differences(nbrs: &Vec<i64>) -> Vec<i64> {
    let mut vec = vec![0; nbrs.len() - 1];
    for i in 0..vec.len() {
        vec[i] = nbrs[i + 1] - nbrs[i];
    }
    vec
}

fn parse_report(input: &str) -> IResult<&str, Report> {
    let (input, lines) = many1(parse_line)(input)?;
    Ok((input, Report { lines }))
}

fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, nbrs) = parse_numbers(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, nbrs))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, numbers) = many1(
            terminated(parse_number, space0)
        )(input)?;
    Ok((input, numbers))
}

fn parse_number(input: &str) -> IResult<&str, i64> {
    let (input, mneg) = opt(tag("-"))(input)?;
    let (input, nbr) = map_res(
        take_while1(is_digit),
        from_digits
    ).parse(input)?;
    match mneg {
        Some(_) => {
            Ok((input, nbr * -1))
        }
        None => {
            Ok((input, nbr))
        }
    }
}

fn from_digits(input: &str) -> Result<i64, std::num::ParseIntError> {
    i64::from_str_radix(input, 10)
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}
