use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|c| c.trim().parse::<i32>().unwrap()).sum())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(elves: &Vec<i32>) -> i32 {
    *elves.iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(elves: &Vec<i32>) -> i32 {
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
    fn sample1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 24000);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 45000);
    }
}
