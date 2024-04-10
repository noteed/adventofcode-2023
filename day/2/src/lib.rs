use nom::{
  bytes::complete::tag,
  bytes::complete::take_while1,
  character::complete::space0,
  combinator::map_res,
  multi::separated_list1,
  sequence::delimited,
  Finish,
  IResult,
  Parser,
};
use std::cmp;
use std::fs;

pub fn run(filename: String) {
  let game_str = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
  let result = parse_game(game_str).finish();
  match result {
    Ok((_, game)) => {
      println!("{:?}", game);
      println!("{:?}", is_game_possible(&game, 12, 13, 14));
    }
    Err(_) => {
    }
  }

    println!("Opening file '{}'...", filename);
    let content = fs::read_to_string(filename).expect("XXX");

    let mut numbers = vec![];
    let mut powers = vec![];
    for line in content.lines() {
        let result = parse_game(line).finish();
        match &result {
          Ok((_, game)) => {
            println!("{:?}", game);
            let possible = is_game_possible(game, 12, 13, 14);
            println!("Possible: {:?}", possible);
            if possible {
              numbers.push(game.nbr);
            }
            powers.push(power(max_game(game)));
          }
          Err(_) => {
          }
        }
    }

    let sum: u32 = numbers.iter().sum();
    println!("Sum: {}", sum);
    let sum: u32 = powers.iter().sum();
    println!("Sum of powers: {}", sum);
}

#[derive(Debug)]
pub struct Game {
  nbr: u32,
  sets: Vec<Set>,
}

#[derive(Debug)]
pub struct Set {
  red: u32,
  green: u32,
  blue: u32,
}

fn is_game_possible(game: &Game, red: u32, green: u32, blue: u32) -> bool {
  game.sets.iter().map(|set| {is_set_possible(set, red, green, blue)}).all(|val| val)
}

fn is_set_possible(set: &Set, red: u32, green: u32, blue: u32) -> bool {
  set.red <= red
  && set.green <= green
  && set.blue <= blue
}

fn power(set: Set) -> u32 {
    set.red * set.green * set.blue
}

fn max_game(game: &Game) -> Set {
  let set0 = Set { red: 0, green: 0, blue: 0 };
  game.sets.iter().fold(set0, max_set)
}

fn max_set(a: Set, b: &Set) -> Set {
  Set {
    red: cmp::max(a.red, b.red),
    green: cmp::max(a.green, b.green),
    blue: cmp::max(a.blue, b.blue),
  }
}

fn parse_game(input: &str) -> IResult<&str, Game> {
  let (input, nbr) = parse_game_head(input)?;
  let (input, sets) = parse_sets(input)?;
  Ok((input, Game { nbr, sets }))
}

fn parse_game_head(input: &str) -> IResult<&str, u32> {
  let (input, _) = delimited(space0, tag("Game"), space0)(input)?;
  let (input, nbr) = parse_number(input)?;
  let (input, _) = delimited(space0, tag(":"), space0)(input)?;
  Ok((input, nbr))
}

fn parse_sets(input: &str) -> IResult<&str, Vec<Set>> {
  let (input, sets) = separated_list1(
      delimited(space0, tag(";"), space0),
      parse_set
      )(input)?;
  Ok((input, sets))
}

fn parse_set(input: &str) -> IResult<&str, Set> {
  let (input, set) = separated_list1(
      delimited(space0, tag(","), space0),
      parse_color
      )(input)?;
  let mut set_ = Set { red: 0, green: 0, blue: 0 };
  for (name, nbr) in &set {
      if name == "red" {
          set_.red += nbr;
      }
      if name == "green" {
          set_.green += nbr;
      }
      if name == "blue" {
          set_.blue += nbr;
      }
  }
  Ok((input, set_))
}

fn parse_color(input: &str) -> IResult<&str, (String, u32)> {
  let (input, nbr) = delimited(space0, parse_number, space0)(input)?;
  let (input, name) = take_while1(is_alpha)(input)?;
  Ok((input, (name.to_string(), nbr)))
}

fn parse_number(input: &str) -> IResult<&str, u32> {
  map_res(
    take_while1(is_digit),
    from_digits
  ).parse(input)
}

fn from_digits(input: &str) -> Result<u32, std::num::ParseIntError> {
  u32::from_str_radix(input, 10)
}

fn is_digit(c: char) -> bool {
  c.is_digit(10)
}

fn is_alpha(c: char) -> bool {
  c.is_ascii_alphabetic()
}
