use nom::{
  bytes::complete::tag,
  bytes::complete::take_while1,
  character::complete::alphanumeric1,
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
use std::cmp::Ordering;
use std::fs;

fn main() {
    //let content = fs::read_to_string("../example.txt").expect("XXX");
    let content = fs::read_to_string("../input.txt").expect("XXX");
    let result = parse_hands(&content).finish();
    match result {
        Ok((_, hands)) => {
            let mut typed_hands: Vec<TypedHand> = hands.iter().map(typed_hand).collect();
            order_hands(&mut typed_hands);
            let mut total = 0;
            for (idx, hand) in typed_hands.iter().enumerate() {
                let rank = (idx + 1) as u64;
                println!("{:?} {}\n  {:?}",
                         hand,
                         rank,
                         count_cards(&hand.cards));
                total += rank * hand.bid;
            }
            println!("Part one: {}", total);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

#[derive(Debug)]
pub struct Hand {
    cards: Vec<u8>,
    bid: u64,
}

#[derive(Debug)]
pub struct TypedHand {
    cards: Vec<u8>,
    bid: u64,
    typ: HandType,
}

#[derive(Debug)]
enum HandType {
    Five(u8),
    Four(u8),
    ThreeTwo(u8, u8),
    Three(u8),
    TwoTwo(u8, u8),
    Two(u8),
    Zero,
}

fn num_type(typ: &HandType) -> u8 {
    match typ {
        HandType::Five(_) => 7,
        HandType::Four(_) => 6,
        HandType::ThreeTwo(_, _) => 5,
        HandType::Three(_) => 4,
        HandType::TwoTwo(_, _) => 3,
        HandType::Two(_) => 2,
        HandType::Zero => 1,
    }
}

// From lowest value to highest.
fn order_hands(hands: &mut Vec<TypedHand>) {
    hands.sort_by(compare_hands);
}

fn compare_hands(a: &TypedHand, b: &TypedHand) -> std::cmp::Ordering {
    match num_type(&a.typ).cmp(&num_type(&b.typ)) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        Ordering::Equal => {
            a.cards.cmp(&b.cards)
        }
    }
}

fn count_cards(cards: &Vec<u8>) -> Vec<u8> {
    let mut count = vec![0; 15];
    for card in cards {
        count[*card as usize] += 1;
    }
    count
}

fn typed_hand(hand: &Hand) -> TypedHand {
    TypedHand { cards: hand.cards.clone(), bid: hand.bid, typ: typed_cards(&hand.cards) }
}

fn typed_cards(cards: &Vec<u8>) -> HandType {
    let mut count = count_cards(cards);
    // Mke the hand "better" my modifying the `count` vector.
    // This does not change the underlying cards, which is good
    // for the way we need to compare them in part two.
    make_better(&mut count);
    match count.iter().enumerate().find(|&(idx, &val)| val == 5) {
        Some((idx, _)) => {
            return HandType::Five(idx.try_into().unwrap());
        }
        None => {
        }
    }
    match count.iter().enumerate().find(|&(idx, &val)| val == 4) {
        Some((idx, _)) => {
            return HandType::Four(idx.try_into().unwrap());
        }
        None => {
        }
    }
    match count.iter().enumerate().find(|&(idx, &val)| val == 3) {
        Some((idx, _)) => {
            let idx_ = idx;
            match count.iter().enumerate().find(|&(idx, &val)| val == 2) {
                Some((idx, _)) => {
                    return HandType::ThreeTwo(
                        idx_.try_into().unwrap(),
                        idx.try_into().unwrap());
                }
                None => {
                    return HandType::Three(idx.try_into().unwrap());
                }
            }
        }
        None => {
        }
    }
    match count.iter().enumerate().find(|&(idx, &val)| val == 2) {
        Some((idx, _)) => {
            let idx_ = idx;
            match count.iter().skip(idx_ + 1).enumerate().find(|&(idx, &val)| val == 2) {
                Some((idx, _)) => {
                    return HandType::TwoTwo(
                        idx_.try_into().unwrap(),
                        (idx + idx_ + 1).try_into().unwrap());
                }
                None => {
                    return HandType::Two(idx.try_into().unwrap());
                }
            }
        }
        None => {
        }
    }
    HandType::Zero
}

fn make_better(count: &mut Vec<u8>) {
    let highest_count = {
        count.iter().enumerate().max_by_key(|&(_, &val)| val).map(|(idx, &val)| (idx, val))
    };
    match highest_count {
        Some((idx, val)) => {
            if idx == (card_to_nbr('J') as usize) {
                // If the card with the highest count is J,
                // change it to whatever else card.
                count[idx] = 0;
                let second_highest = {
                    count.iter().enumerate().max_by_key(|&(_, &val)| val).map(|(idx, &val)| (idx, val))
                };
                match second_highest {
                    Some((idx_, val_)) => {
                        if val == 0 { // means that we had 5 J.
                            count[idx] = val;
                        } else
                        {
                            count[idx_] += val;
                        }
                    }
                    None => {
                        panic!("XXX");
                    }
                }
            } else {
                // Else, change J to the that card.
                count[idx] += count[card_to_nbr('J') as usize];
                count[card_to_nbr('J') as usize] = 0;
            }
        }
        None => {
            panic!("XXX");
        }
    }
}

fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, hands) = many1(parse_hand)(input)?;
    Ok((input, hands))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, cards) = parse_cards(input)?;
    let (input, _) = space1(input)?;
    let (input, bid) = parse_number(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Hand {cards, bid}))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, cards) = alphanumeric1(input)?;
    let cards = cards.chars().map(card_to_nbr).collect();
    Ok((input, cards))
}

fn card_to_nbr(card: char) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        // 'J' => 11, // Fort part one.
        'J' => 1, // For part two.
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("XXXX"),
    }
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
