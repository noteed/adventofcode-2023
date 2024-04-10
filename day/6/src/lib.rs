use nom::{
  bytes::complete::tag,
  bytes::complete::take_while1,
  character::complete::multispace0,
  character::complete::space0,
  combinator::map_res,
  multi::many1,
  sequence::terminated,
  Finish,
  IResult,
  Parser,
};
use std::fs;

pub fn run(filename: String) {
    let content = fs::read_to_string(filename).expect("XXX");
    let result = parse_races(&content).finish();
    match result {
        Ok((_, races)) => {
            println!("{:?}", races);
            let mut total = 1;
            for race in races {
                let a = 1 as f64;
                let b = -(race.time as f64);
                let c = race.distance as f64;
                let res = solve_quadratic(a, b, c).expect("Can't win");
                let count = count_between(res.0, res.1);
                println!("{:?} {}", res, count);
                total = total * count;
            }
            println!("Part one: {}", total);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

#[derive(Debug)]
pub struct Race {
    time: u64,
    distance: u64,
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = parse_times(input)?;
    let (input, distances) = parse_distances(input)?;
    let races = times.iter().zip(distances.iter())
        .map(|(t, d)| Race {time: *t, distance: *d}).collect();
    Ok((input, races))
}

fn parse_times(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = terminated(tag("Time:"), space0)(input)?;
    let (input, nbrs) = parse_numbers(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, nbrs))
}

fn parse_distances(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = terminated(tag("Distance:"), space0)(input)?;
    let (input, nbrs) = parse_numbers(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, nbrs))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, numbers) = many1(
            terminated(parse_number, space0)
        )(input)?;
    Ok((input, numbers))
}

fn parse_number(input: &str) -> IResult<&str, u64> {
    map_res(
        take_while1(is_digit),
        from_digits
    ).parse(input)
}

fn from_digits(input: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(input, 10)
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b*b - 4.0*a*c;

    if discriminant < 0.0 {
        None // No real roots
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        let root1 = (-b + sqrt_discriminant) / (2.0 * a);
        let root2 = (-b - sqrt_discriminant) / (2.0 * a);

        Some((root1, root2)) // Real roots
    }
}

fn count_between(a: f64, b: f64) -> u64 {
    let mut low = b.ceil() as u64;
    let mut high = a.floor() as u64;
    if (low as f64) == b { low = low + 1; }
    if (high as f64) == a { high = high - 1; }
    high - low + 1
}
