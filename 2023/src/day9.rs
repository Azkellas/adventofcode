use itertools::Itertools;

pub struct Entry {
    sequences: Vec<Vec<i64>>,
}
pub fn input_generator(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect_vec()
        })
        .map(|mut sequence| {
            let mut sequences = vec![];
            while sequence.iter().any(|n| *n != 0) {
                sequences.push(sequence.clone());
                sequence = sequence
                    .into_iter()
                    .tuple_windows()
                    .map(|(p, q)| q - p)
                    .collect_vec();
            }
            Entry { sequences }
        })
        .collect_vec()
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> i64 {
    input_generator(input)
        .into_iter()
        .map(|entry| {
            entry
                .sequences
                .into_iter()
                .map(|v| *v.last().unwrap())
                .sum::<i64>()
        })
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> i64 {
    let input = input_generator(input);
    input
        .into_iter()
        .map(|entry| {
            entry
                .sequences
                .into_iter()
                .enumerate()
                .map(|(i, v)| v[0] * i64::pow(-1, (i % 2 == 1) as u32))
                .sum::<i64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 114);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 2);
    }

    static INPUT: &str = include_str!("../input/2023/day9.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 1939607039);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 1041);
    }
}
