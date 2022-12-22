use itertools::Itertools;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, 4)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    solve(input, 14)
}

fn solve(input: &str, message_size: usize) -> usize {
    input
        .as_bytes()
        .windows(message_size)
        .find_position(|window| window.iter().all_unique())
        .unwrap()
        .0
        + message_size
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLES: [(&str, usize, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    #[test]
    fn sample1() {
        for (input, result, _) in EXAMPLES {
            assert_eq!(part1(input), result);
        }
    }

    #[test]
    fn sample2() {
        for (input, _, result) in EXAMPLES {
            assert_eq!(part2(input), result);
        }
    }

    static INPUT: &str = include_str!("../input/2022/day6.txt");
    #[test]
    fn sample3() {
        assert_eq!(part1(INPUT), 1655);
    }

    #[test]
    fn sample4() {
        assert_eq!(part2(INPUT), 2665);
    }
}
