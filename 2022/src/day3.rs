use itertools::Itertools;
use std::collections::HashSet;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let (left, right) = line.trim().split_at(line.len() / 2);
            let left: HashSet<char> = HashSet::from_iter(left.chars());
            let right: HashSet<char> = HashSet::from_iter(right.chars());
            let c = HashSet::intersection(&left, &right).next().unwrap();
            match c.is_ascii_lowercase() {
                true => (*c as u8 - b'a' + 1) as i32,
                false => (*c as u8 - b'A' + 27) as i32,
            }
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .into_iter()
        .map(|l| HashSet::from_iter(l.chars()))
        .tuples()
        .map(
            |(l1, l2, l3): (HashSet<char>, HashSet<char>, HashSet<char>)| {
                let c = HashSet::intersection(&l1, &l2)
                    .find(|c| l3.contains(c))
                    .unwrap();
                match c.is_ascii_lowercase() {
                    true => (*c as u8 - b'a' + 1) as i32,
                    false => (*c as u8 - b'A' + 27) as i32,
                }
            },
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 157);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 70);
    }

    static INPUT: &str = include_str!("../input/2022/day3.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 7428);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 2650);
    }
}
