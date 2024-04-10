use nom::{
  bytes::complete::tag,
  bytes::complete::take_while1,
  character::complete::space0,
  combinator::map_res,
  multi::many1,
  sequence::delimited,
  sequence::terminated,
  Finish,
  IResult,
  Parser,
};
use std::fs;

pub fn run(filename: String) {
    println!("Opening file '{}'...", filename);
    let content = fs::read_to_string(filename).expect("XXX");

    let mut total = 0;
    let mut cards = vec![]; // For part two.
    for line in content.lines() {
        let result = parse_card(line).finish();
        match result {
          Ok((_, card)) => {
            println!("{:?} {}", card, worth_1(&card));
            total += worth_1(&card);
            cards.push(card);
          }
          Err(err) => {
            println!("{:?}", err);
          }
        }
    }
    println!("Part one: {}", total);
    for i in 0..cards.len() {
        let len = cards.len();
        let mut card = &mut cards[i];
        let rep = card.repetition;
        let n: usize = worth_2(card).try_into().unwrap();
        for j in 0..n {
            if i + j + 1 < len {
                cards[i + j + 1].repetition += rep;
            }
        }
    }
    for card in &cards {
        println!("{:?}", card);
    }

    println!("Part two: {}", cards.iter().map(|card| card.repetition).fold(0, |a, b| a + b));
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Card {
  nbr: u32,
  winning: Vec<u32>,
  having: Vec<u32>,
  repetition: u32, // For part two.
}

fn worth_1(card: &Card) -> u32 {
    let mut n = 0;
    for b in &card.having {
        for a in &card.winning {
            if b == a {
                if n == 0 { n = 1; }
                else { n = n * 2; }
            }
        }
    }
    n
}

fn worth_2(card: &Card) -> u32 {
    let mut n = 0;
    for b in &card.having {
        for a in &card.winning {
            if b == a { n += 1; }
        }
    }
    n
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, nbr) = parse_card_head(input)?;
    let (input, winning) = parse_numbers(input)?;
    let (input, m) = delimited(space0, tag("|"), space0)(input)?;
    let (input, having) = parse_numbers(input)?;
    Ok((input, Card {nbr, winning, having, repetition: 1}))
}

fn parse_card_head(input: &str) -> IResult<&str, u32> {
  let (input, _) = delimited(space0, tag("Card"), space0)(input)?;
  let (input, nbr) = parse_number(input)?;
  let (input, _) = delimited(space0, tag(":"), space0)(input)?;
  Ok((input, nbr))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, numbers) = many1(
            terminated(parse_number, space0)
        )(input)?;
    Ok((input, numbers))
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
