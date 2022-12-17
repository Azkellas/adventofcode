#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let line = line.as_bytes();
            let opp_play = (line[0] - b'A') as i32;
            let unknown = (line[2] - b'X') as i32;
            (opp_play, unknown)
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(i32, i32)]) -> i32 {
    input
        .iter()
        .map(|(opp_play, my_play)| {
            // hand score + result score
            (my_play + 1) + (3 * ((my_play - opp_play + 4) % 3))
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[(i32, i32)]) -> i32 {
    input
        .iter()
        .map(|(opp_play, result)| {
            let my_play = (opp_play + result + 2) % 3;
            // hand score + result score
            (my_play + 1) + (3 * ((my_play - opp_play + 4) % 3))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
A Y
B X
C Z";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 15);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 12);
    }
}
