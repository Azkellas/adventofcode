use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, 4)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    solve(input, 14)
}

fn solve(input: &str, message_size: usize) -> usize {
    for i in 0..input.len() {
        let set: HashSet<&u8> = HashSet::from_iter(input.as_bytes()[i..i+message_size].iter());
        if set.len() == message_size {
            return i + message_size;
        }
    }
    unreachable!();
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
}
