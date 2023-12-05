use nom::{
  bytes::complete::tag,
  bytes::complete::take_while1,
  character::complete::multispace0,
  character::complete::space0,
  character::complete::space1,
  combinator::map_res,
  multi::many1,
  sequence::delimited,
  sequence::terminated,
  Finish,
  IResult,
  Parser,
};
use std::fs;

fn main() {
    println!("Opening file '{}'...", "../input.txt");
    let content = fs::read_to_string("../input.txt").expect("XXX");
    let result = parse_almanac(&content).finish();
    match result {
      Ok((_, almanac)) => {
          println!("{:?}", almanac);
          for seed in &almanac.seeds {
              println!("{:?}", resolvex(*seed, &almanac));
          }
          println!("Part one: {:?}", almanac.seeds.iter().map(|seed| resolvex(*seed, &almanac)).min());
/*
 * Converting the Mapping ((dest, src, len) tripes) to Map (for easy indexing)
 * i.e. converting from Almanac to Almanac_.
 * was a bad idea. I guess it allocates a lot of memory.
 *
          let almanac = convert_almanac(&almanac);
          println!("{:?}", almanac);
          for seed in &almanac.seeds {
              println!("{:?}", resolve(*seed, &almanac));
          }
 */
          let seeds: Vec<(u64, u64)> = almanac.seeds.chunks(2)
                  .filter_map(|chunk| {
                      if chunk.len() == 2 {
                          Some((chunk[0], chunk[1]))
                      } else {
                          None
                      }
                  })
                  .collect();
          let mut minimum = u64::max_value();
          for (initial, range) in seeds {
              for i in 0..range {
                  let current = resolvex(initial + i, &almanac);
                  if current < minimum { minimum = current; }
              }
          }
          println!("Part two: {:?}", minimum);
        }
      Err(err) => {
        println!("{:?}", err);
      }
    }
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u64>,
    mappings: Vec<Mapping>,
}

#[derive(Debug)]
pub struct Almanac_ {
    seeds: Vec<u64>,
    mappings: Vec<Map>,
}

#[derive(Debug)]
pub struct Mapping {
    from: String,
    to: String,
    map: Vec<(u64, u64, u64)>, // destination, source, length
}

// Similar to Mapping, but express the "sparse" `map` as a contiguous
// array for easy lookups.
#[derive(Debug)]
pub struct Map {
    from: String,
    to: String,
    map: Vec<u64>, // destination (source is the array index)
}

fn convert_almanac(a: &Almanac) -> Almanac_ {
    Almanac_ { seeds: a.seeds.iter().map(|&x| x).collect(), mappings: a.mappings.iter().map(convert_mapping).collect() }
}

fn convert_mapping(mapping: &Mapping) -> Map {
    let max_source = mapping.map.iter().map(|(_, s, l)| s + l).max().unwrap();
    let mut map: Vec<u64> = (0..max_source).collect();
    for (destination, source, length) in &mapping.map {
        for i in 0..*length {
            let j: usize = (source + i).try_into().unwrap();
            if j < map.len() {
              map[j] = destination + i;
            }
        }
    }
    Map { from: mapping.from.to_string(), to: mapping.to.to_string(), map }
}

fn resolve(seed: u64, almanac: &Almanac_) -> u64 {
    almanac.mappings.iter().fold(seed, |acc, map| resolve1(acc, map))
}

fn resolve1(seed: u64, map: &Map) -> u64 {
    if (seed as usize) < map.map.len() {
        map.map[seed as usize]
    } else {
        seed
    }
}

fn resolvex(seed: u64, almanac: &Almanac) -> u64 {
    almanac.mappings.iter().fold(seed, |acc, map| resolve1x(acc, map))
}

fn resolve1x(seed: u64, mapping: &Mapping) -> u64 {
    for (destination, source, length) in &mapping.map {
        if seed >= *source && seed < source + length {
            return destination + (seed - source);
        }
    }
    seed
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, mappings) = many1(parse_mapping)(input)?;
    Ok((input, Almanac { seeds, mappings }))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = terminated(tag("seeds:"), space0)(input)?;
    let (input, seeds) = parse_numbers(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, seeds))
}

fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
    let (input, from) = take_while1(is_alpha)(input)?;
    let (input, _) = tag("-to-")(input)?;
    let (input, to) = take_while1(is_alpha)(input)?;
    let (input, _) = delimited(space0, tag("map:"), space0)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, map) = many1(parse_triple)(input)?;
    Ok((input, Mapping {from: from.to_string(), to: to.to_string(), map}))
}

fn parse_triple(input: &str) -> IResult<&str, (u64, u64, u64)> {
    let (input, destination) = parse_number(input)?;
    let (input, _) = space1(input)?;
    let (input, source) = parse_number(input)?;
    let (input, _) = space1(input)?;
    let (input, length) = parse_number(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, (destination, source, length)))
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

fn is_alpha(c: char) -> bool {
  c.is_ascii_alphabetic()
}
