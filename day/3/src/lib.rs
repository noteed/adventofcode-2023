use std::fs;


#[derive(Debug)]
pub struct Number {
  value: i32,
  x1: i32,
  x2: i32,
  y: i32,
}

pub fn run(filename: String) {
    println!("Opening file '{}'...", filename);
    let content = fs::read_to_string(filename).expect("XXX");

    // Construct a list of numbers, including their coordinates.
    let mut numbers = vec![];
    for (y, line) in content.lines().enumerate() {
        let mut in_number = false;
        let mut number = "".to_string();
        let mut x1 = 0;
        let mut x2 = 0;
        for (x, character) in line.chars().enumerate() {
            println!("Character at {},{}: {}", x, y, character);
            if character.is_digit(10) {
                if !in_number {
                    x1 = x;
                }
                in_number = true;
                number += &character.to_string();
            } else {
                if in_number {
                    x2 = x - 1;
                    let value = number.parse().unwrap();
                    let n = Number { value,
                      x1: (x1 + 1).try_into().unwrap(),
                      x2: (x2 + 1).try_into().unwrap(),
                      y: (y + 1).try_into().unwrap(),
                    };
                    numbers.push(n);
                }
                in_number = false;
                number = "".to_string();
            }
        }
        if in_number {
            x2 = line.len();
            let value = number.parse().unwrap();
            let n = Number { value,
              x1: (x1 + 1).try_into().unwrap(),
              x2: (x2 + 1).try_into().unwrap(),
              y: (y + 1).try_into().unwrap(),
            };
            numbers.push(n);
        }
    }

    // Construct a list of symbol coordinates.
    let mut symbols = vec![];
    for (y, line) in content.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if (!character.is_digit(10)) && (character != '.') {
                symbols.push(((x + 1).try_into().unwrap(), (y + 1).try_into().unwrap()));
            }
        }
    }

    for s in &symbols {
        println!("{:?}", s);
    }
    let mut total = 0;
    for n in &numbers {
        println!("{:?} {:?}", n, is_part_number(&n, &symbols));
        if is_part_number(&n, &symbols) {
            total += n.value;
        }
    }
    println!("Part one: {:?}", total);

    let mut total = 0;
    for (x, y) in &symbols {
        let mut potential_gear = 0;
        let mut potential_gear_ratio = 1;
        for n in &numbers {
            if touch(&n, (&x, &y)) {
                potential_gear += 1;
                if potential_gear <= 2 {
                    potential_gear_ratio *= n.value;
                }
            }
        }
        if potential_gear == 2 {
            total += potential_gear_ratio;
        }
    }
    println!("Part two: {:?}", total);
}

fn is_part_number(n: &Number, symbols: &Vec<(i32, i32)>) -> bool {
    for (x, y) in symbols {
        if touch(&n, (x, y)) {
            return true;
        }
    }
    false
}

fn touch(n: &Number, (x, y): (&i32, &i32)) -> bool {
    &(n.y-1) <= y && y <= &(n.y+1) && &(n.x1-1) <= x && x <= &(n.x2+1)
}
