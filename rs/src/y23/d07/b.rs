use std::cmp::Ordering;
use std::collections::HashMap;
use util::*;

#[derive(PartialEq, Eq)]
struct Card(char);

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = "J123456789TQKA";
        ord.find(self.0).partial_cmp(&ord.find(other.0))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    Triple,
    FullHouse,
    Quadruple,
    Quintuple,
}

impl std::str::FromStr for HandType {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars_counts: HashMap<char, i32> =
            s.to_owned().chars().fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            });
        let j_count = if let Some(j_count) = chars_counts.remove(&'J') {
            j_count
        } else {
            0
        };
        if j_count == 5 {
            return Ok(Self::Quintuple);
        }
        let mut counts: Vec<&mut i32> = chars_counts.values_mut().sorted().rev().collect();
        *counts[0] += j_count;
        Ok(match counts.len() {
            5 => Self::HighCard,
            4 => Self::OnePair,
            3 => match counts[0] {
                3 => Self::Triple,
                2 => Self::TwoPair,
                _ => unreachable!(),
            },
            2 => match counts[0] {
                4 => Self::Quadruple,
                3 => Self::FullHouse,
                _ => unreachable!(),
            },
            1 => Self::Quintuple,
            _ => unreachable!(),
        })
    }
}

struct Hand {
    pub bid: i32,
    hand: String,
    hand_type: HandType,
}

impl Hand {
    pub fn new(input: &str) -> Self {
        let (hand, bid) = input.ssplit_once(" ");

        Self {
            bid: bid.uinto(),
            hand: hand.to_owned(),
            hand_type: hand.uinto(),
        }
    }

    pub fn into_vec_cards(&self) -> Vec<Card> {
        self.hand.chars().map(|c| Card(c.clone())).collect_vec()
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
            && self.hand.chars().sorted().collect::<String>()
                == other.hand.chars().sorted().collect::<String>()
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.into_vec_cards().partial_cmp(&other.into_vec_cards()),
            i => Some(i),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[aoc_proc::aoc_run(23-07b)]
pub fn run(input: impl AsRef<str>) -> Result<i32, BoxError> {
    let sum = input
        .as_ref()
        .lines()
        .map(|line| Hand::new(line))
        .sorted()
        .enumerate()
        .fold(0, |val, (i, hand)| val + (i as i32 + 1) * hand.bid);
    Ok(sum)
}
