use scan_fmt::*;

pub fn input_generator(input: &str) -> impl Iterator<Item = (i32, i32, i32, i32)> + '_ {
    input
        .lines()
        .into_iter()
        .map(|line| scan_fmt!(line, "{d}-{d},{d}-{d}", i32, i32, i32, i32).unwrap())
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    input_generator(input)
        .filter(|(a1, a2, b1, b2)| a1 <= b1 && a2 >= b2 || b1 <= a1 && b2 >= a2)
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    input_generator(input)
        .filter(|(a1, a2, b1, b2)| i32::max(0, i32::min(*a2, *b2) + 1 - i32::max(*a1, *b1)) > 0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    #[test]
    fn sample1() {
        assert_eq!(part1(EXAMPLE), 2);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(EXAMPLE), 4);
    }
}
