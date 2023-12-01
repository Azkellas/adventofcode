use itertools::Itertools;

#[aoc(day1, part1)]
pub fn part1(elves: &str) -> u32 {
    elves
        .split_whitespace()
        .map(|line| {
            let digits = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
                .collect_vec();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(elves: &str) -> u32 {
    part1(
        elves
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
            .as_str(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_PART1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    static EXAMPLE_PART2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_PART1), 142);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_PART2), 281);
    }

    static INPUT: &str = include_str!("../input/2023/day1.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 54390);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 54277);
    }
}
