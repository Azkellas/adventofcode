use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|c| c.trim().parse::<i32>().unwrap()).sum())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(elves: &[i32]) -> i32 {
    *elves.iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(elves: &[i32]) -> i32 {
    elves.iter().sorted().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn example_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 24000);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 45000);
    }

    static INPUT: &str = include_str!("../input/2022/day1.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 69912);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 208180);
    }

}
