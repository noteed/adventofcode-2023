use nom::{
  bytes::complete::tag,
  character::complete::alphanumeric1,
  character::complete::multispace0,
  character::complete::space0,
  multi::many1,
  sequence::delimited,
  Finish,
  IResult,
};
use num_integer::lcm;
use std::collections::HashMap;
use std::fs;

pub fn run(filename: String) {
    let content = fs::read_to_string(filename).expect("XXX");
    let result = parse_network(&content).finish();
    match result {
        Ok((_, network)) => {
            println!("{:?}", network);

            let mut map = HashMap::new();
            for node in &network.nodes {
                map.insert(
                    node.name.to_string(),
                    (node.name1.to_string(), node.name2.to_string())
                    );
            }
            let mut current = "AAA";
            let mut steps = 0;
            for direction in (&network.directions.iter()).clone().cycle() {
                if current == "ZZZ" {
                    break;
                }
                match direction {
                    Direction::Left => {
                        current = &map.get(current).unwrap().0;
                    }
                    Direction::Right => {
                        current = &map.get(current).unwrap().1;
                    }
                }
                steps += 1;
            }
            println!("Part one: {}", steps);

            let starts: &mut Vec<String> = &mut network.nodes.iter().map(|node| node.name.to_string()).filter(|name| name.ends_with('A')).collect();
            println!("{:?}", starts);
            for i in 0..starts.len() {
                let mut current = starts[i].to_string();
                let mut steps = 0;
                for direction in (&network.directions.iter()).clone().cycle() {
                    if current.ends_with('Z') {
                        break;
                    }
                    match direction {
                        Direction::Left => {
                            current = map.get(&current).unwrap().0.to_string();
                        }
                        Direction::Right => {
                            current = map.get(&current).unwrap().1.to_string();
                        }
                    }
                    steps += 1;
                }
                println!("{}: {}", starts[i], steps);
            }
            // QXA: 12643
            // PDA: 14257
            // TDA: 15871
            // QQA: 18023
            // PPA: 19637
            // AAA: 16409
            // I think that each xxZ position will act exactly as its starting xxA,
            // forming a cycle.
            let results: Vec<u64> = vec![12643, 14257, 15871, 18023, 19637, 16409];
            let steps = results.iter().cloned().reduce(|a, b| lcm(a, b)).unwrap_or(1);
            println!("Part two: {}", steps);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

#[derive(Debug)]
pub struct Network {
    directions: Vec<Direction>,
    nodes: Vec<Node>,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Node {
    name: String,
    name1: String,
    name2: String,
}

fn parse_network(input: &str) -> IResult<&str, Network> {
    let (input, directions) = parse_directions(input)?;
    let (input, _) = multispace0(input)?;
    let (input, nodes) = many1(parse_node)(input)?;
    Ok((input, Network { directions, nodes }))
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, directions) = alphanumeric1(input)?;
    let directions = directions.chars().map(char_to_direction).collect();
    Ok((input, directions))
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let (input, name) = parse_name(input)?;
    let (input, _) = delimited(space0, tag("="), space0)(input)?;
    let (input, _) = delimited(space0, tag("("), space0)(input)?;
    let (input, name1) = parse_name(input)?;
    let (input, _) = delimited(space0, tag(","), space0)(input)?;
    let (input, name2) = parse_name(input)?;
    let (input, _) = delimited(space0, tag(")"), space0)(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Node { name, name1, name2 }))
}

fn parse_name(input: &str) -> IResult<&str, String> {
    let (input, name) = alphanumeric1(input)?;
    Ok((input, name.to_string()))
}

fn char_to_direction(direction: char) -> Direction {
    match direction {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("XXXX"),
    }
}
