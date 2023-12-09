use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Strength {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Hand {
    hand: Vec<usize>,
    bid: usize,
    strength: Strength,
}

impl Hand {
    pub fn new(part: Part, hand: Vec<usize>, bid: usize) -> Self {
        let strength = {
            let jokers = (part == Part::Part2) as usize * hand.iter().filter(|&&c| c == 0).count();

            let mut count = hand
                .iter()
                .counts()
                .into_iter()
                .filter(|(&c, _)| part == Part::Part1 || c != 0)
                .map(|(_, i)| i)
                .sorted_by(|a, b| b.cmp(a))
                .collect_vec();

            if count.is_empty() {
                count.push(0);
            }
            count[0] += jokers;

            match count.as_slice() {
                [5] => Strength::FiveOfAKind,
                [4, 1] => Strength::FourOfAKind,
                [3, 2] => Strength::FullHouse,
                [3, 1, 1] => Strength::ThreeOfAKind,
                [2, 2, 1] => Strength::TwoPair,
                [2, 1, 1, 1] => Strength::Pair,
                [1, 1, 1, 1, 1] => Strength::HighCard,
                _ => unreachable!(),
            }
        };
        Self {
            hand,
            bid,
            strength,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.strength.cmp(&other.strength) {
            Ordering::Equal => self.hand.cmp(&other.hand),
            ord => ord,
        }
    }
}

pub fn input_generator(part: Part, input: &str) -> Vec<Hand> {
    let lines = input.lines();

    let strengthes = match part {
        Part::Part1 => "23456789TJQKA",
        Part::Part2 => "J23456789TQKA",
    }
    .chars()
    .collect_vec();

    lines
        .into_iter()
        .map(|s| {
            let (hand, bid) = s.split_once(' ').unwrap();
            let bid = bid.parse().unwrap();
            let hand = hand
                .chars()
                .map(|c| strengthes.iter().position(|&s| s == c).unwrap())
                .collect_vec();
            Hand::new(part, hand, bid)
        })
        .collect_vec()
}

fn sort_and_score(hands: &[Hand]) -> usize {
    hands
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    sort_and_score(&input_generator(Part::Part1, input))
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    sort_and_score(&input_generator(Part::Part2, input))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 6440);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 5905);
    }

    static INPUT: &str = include_str!("../input/2023/day7.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 253603890);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 253630098);
    }
}
