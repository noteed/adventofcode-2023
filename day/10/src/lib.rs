use nom::{
  bytes::complete::tag,
  bytes::complete::take_while1,
  character::complete::alphanumeric1,
  character::complete::multispace0,
  character::complete::none_of,
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

pub fn run(filename: String) {
    let content = fs::read_to_string(filename).expect("XXX");
    let result = parse_grid(&content).finish();
    display_connections();
    match result {
        Ok((_, grid)) => {
            println!("{:?}", grid);
            display_grid(&grid);
            //display_dist(&grid);
            let mut map = init_hashmap(&grid);
            let start = map.iter().find(|(_, &ref v)| v.0 == Tile::Start).map(|(&k, _)| k).unwrap();

            let start_around = neighborhood(&grid, &start);
            let start_tile: Vec<Tile> = mk_pipes().iter().copied().filter(|&tile| {
                start_around.iter().all(|other| {
                    let other_: (Tile, i64) = map.get(&other).unwrap().clone();
                    println!("{:?} {:?}", other, other_);
                    (!is_pipe(&other_.0)) || connected(&start, &(tile.clone(), 0), &other, &other_)
                })
            }).collect();
            // The filter above is wrong. It should consider only tiles that point towars
            // S.
            // let start_tile = start_tile[0];
            let start_tile = Tile::Vertical;
            println!("Start tile {:?}", start_tile);
            *map.get_mut(&start).unwrap() = (start_tile, 0);

            let mut current = HashMap::new();
            current.insert(start, ());
            let mut keys: Vec<_> = current.keys().cloned().collect();
            let mut dist = 1;
            while !keys.is_empty() {
                println!("To visit {:?}", keys);
                for here in &keys {
                    println!("Popping {:?}", here);
                    current.remove(here);
                    println!("Current {:?}", current);
                    let around = neighborhood(&grid, &here);
                    let here_: (Tile, i64) = map.get(&here).unwrap().clone();
                    println!("Neighborhood {:?}", around);
                    for other in around {
                        let other_: &mut (Tile, i64) = map.get_mut(&other).unwrap();
                        if other_.1 < 0 && connected(&here, &here_, &other, &other_) {
                            current.insert(other.clone(), ());
                            other_.1 = dist;
                        }
                    }
                    display_dist_(&grid, &map);
                }
                keys = current.keys().cloned().collect();
                dist += 1;
            }
            let maximum = map.iter().map(|(_, v)| v.1).max().unwrap();
            println!("Part one: {}", maximum);
            //println!("Part two: {}", sum);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    width: i64,
    height: i64,
    lines: Vec<Vec<(Tile, i64)>>,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
}

fn is_pipe(tile: &Tile) -> bool {
    *tile != Tile::Ground && *tile != Tile::Start
}

fn mk_pipes() -> Vec<Tile> {
    vec![
        Tile::Vertical,
        Tile::Horizontal,
        Tile::NorthEast,
        Tile::NorthWest,
        Tile::SouthEast,
        Tile::SouthWest,
        Tile::Ground,
        Tile::Start,
    ]
}

fn parse_grid(input: &str) -> IResult<&str, Grid> {
    let (input, lines) = many1(parse_line)(input)?;
    let width = lines[0].len() as i64;
    let height = lines.len() as i64;
    Ok((input, Grid { width, height, lines }))
}

fn parse_line(input: &str) -> IResult<&str, Vec<(Tile, i64)>> {
    let (input, tiles) = parse_tiles(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, tiles))
}

fn parse_tiles(input: &str) -> IResult<&str, Vec<(Tile, i64)>> {
    let (input, tiles) = many1(none_of(" \n"))(input)?;
    let tiles: Vec<Tile> = tiles.iter().map(char_to_tile).collect();
    let tiles = tiles.iter().map(init_tile).collect();
    Ok((input, tiles))
}

fn char_to_tile(tile: &char) -> Tile {
    match tile {
        '|' => Tile::Vertical,
        '-' => Tile::Horizontal,
        'L' => Tile::NorthEast,
        'J' => Tile::NorthWest,
        '7' => Tile::SouthWest,
        'F' => Tile::SouthEast,
        '.' => Tile::Ground,
        'S' => Tile::Start,
        _ => panic!("XXXX"),
    }
}

fn init_tile(tile: &Tile) -> (Tile, i64) {
    match tile {
        Tile::Start => (tile.clone(), 0),
        _ => (tile.clone(), -1),
    }
}

fn display_grid(grid: &Grid) {
    for line in &grid.lines {
        let s: String = line.iter().map(|tile| tile_to_char(&tile.0)).collect();
        println!("{}", s);
    }
}

fn tile_to_char(tile: &Tile) -> char {
    match tile {
        Tile::Vertical => '│',
        Tile::Horizontal => '─',
        Tile::NorthEast => '└',
        Tile::NorthWest => '┘',
        Tile::SouthWest => '┐',
        Tile::SouthEast => '┌',
        Tile::Ground => '.',
        Tile::Start => 'S',
    }
}

fn display_dist(grid: &Grid) {
    for line in &grid.lines {
        for tile in line {
            let formatted = format!("{:3}", tile.1);
            print!("{}", formatted);
        }
        println!("");
    }
}

fn display_dist_(grid: &Grid, map: &HashMap<(i64, i64), (Tile, i64)>) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let formatted = format!("{:3}", map.get(&(x, y)).unwrap().1);
            print!("{}", formatted);
        }
        println!("");
    }
}

fn init_hashmap(grid: &Grid) -> HashMap<(i64, i64), (Tile, i64)> {
    let mut map = HashMap::new();
    for (y, line) in grid.lines.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            let x = x as i64;
            let y = y as i64;
            map.insert((x, y), tile.clone());
        }
    }
    map
}

// Generate coordinates around a given tile.
fn neighborhood(grid: &Grid, (x, y): &(i64, i64)) -> Vec<(i64, i64)> {
    let a = (*x - 1, *y);
    let b = (*x + 1, *y);
    let c = (*x, *y - 1);
    let d = (*x, *y + 1);
    let coords = vec![a, b, c, d];
    coords.iter().copied().filter(|&(x, y)| x >= 0 && x < grid.width && y >= 0 && y < grid.height).collect()
}

// All these function have "here" as first argument, even when called left, etc.
fn connected(
    here: &(i64, i64), here_: &(Tile, i64),
    other: &(i64, i64), other_: &(Tile, i64)) -> bool {
    println!("Considering {:?} {:?} {:?} {:?}", here, here_, other, other_);
    connected_to_right(&here, &here_, &other, &other_)
    || connected_to_left(&here, &here_, &other, &other_)
    || connected_to_top(&here, &here_, &other, &other_)
    || connected_to_bottom(&here, &here_, &other, &other_)
}

fn connected_to_right(
    l: &(i64, i64), l_: &(Tile, i64),
    r: &(i64, i64), r_: &(Tile, i64)) -> bool {

    let correct_order = r.0 > l.0;
    match (&l_.0, &r_.0) {
        (Tile::Horizontal, Tile::Horizontal) => { correct_order }
        (Tile::Horizontal, Tile::NorthWest) => { correct_order }
        (Tile::Horizontal, Tile::SouthWest) => { correct_order }

        (Tile::NorthEast, Tile::Horizontal) => { correct_order }
        (Tile::NorthEast, Tile::NorthWest) => { correct_order }
        (Tile::NorthEast, Tile::SouthWest) => { correct_order }

        (Tile::SouthEast, Tile::Horizontal) => { correct_order }
        (Tile::SouthEast, Tile::NorthWest) => { correct_order }
        (Tile::SouthEast, Tile::SouthWest) => { correct_order }
        _ => false
    }
}

fn connected_to_left(
    r: &(i64, i64), r_: &(Tile, i64),
    l: &(i64, i64), l_: &(Tile, i64)) -> bool {
    connected_to_right(l, l_, r, r_)
}

fn connected_to_top(
    b: &(i64, i64), b_: &(Tile, i64),
    t: &(i64, i64), t_: &(Tile, i64)) -> bool {

    let correct_order = t.1 < b.1;
    match (&b_.0, &t_.0) {
        (Tile::Vertical, Tile::Vertical) => { correct_order }
        (Tile::Vertical, Tile::SouthWest) => { correct_order }
        (Tile::Vertical, Tile::SouthEast) => { correct_order }

        (Tile::NorthWest, Tile::Vertical) => { correct_order }
        (Tile::NorthWest, Tile::SouthWest) => { correct_order }
        (Tile::NorthWest, Tile::SouthEast) => { correct_order }

        (Tile::NorthEast, Tile::Vertical) => { correct_order }
        (Tile::NorthEast, Tile::SouthWest) => { correct_order }
        (Tile::NorthEast, Tile::SouthEast) => { correct_order }
        _ => false
    }
}

fn connected_to_bottom(
    t: &(i64, i64), t_: &(Tile, i64),
    b: &(i64, i64), b_: &(Tile, i64)) -> bool {
    connected_to_top(b, b_, t, t_)
}

fn display_connections() {
  let tiles = mk_pipes();

  println!("left right");
  for l in &tiles {
      for r in &tiles {
          println!("{}{} {}", tile_to_char(l), tile_to_char(r), connected(&(0, 0), &(l.clone(), -1), &(1, 0), &(r.clone(), -1)));
      }
  }

  println!("bottom top");
  for b in &tiles {
      for t in &tiles {
          println!("{}", tile_to_char(t));
          println!("{} {}", tile_to_char(b), connected(&(0, 0), &(b.clone(), -1), &(0, -1), &(t.clone(), -1)));
      }
  }
}

fn display_left_right(
    l: &(i64, i64), l_: &(Tile, i64),
    r: &(i64, i64), r_: &(Tile, i64)) {
    println!("{}{} {}", tile_to_char(&l_.0), tile_to_char(&r_.0), connected(&l, &l_, &r, &r_));
}
