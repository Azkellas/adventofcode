use itertools::Itertools;

pub fn input_generator(input: &str) -> (Vec<usize>, Vec<usize>) {
    let lines = input.lines();

    let (times, distances) = lines
        .into_iter()
        .map(|s| {
            s.split_once(": ")
                .unwrap()
                .1
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();

    (times, distances)
}

pub fn get_range(time: usize, distance: usize) -> usize {
    let discriminant = time as f64 * time as f64 - 4.0 * (distance as f64);
    let mut t1 = (time as f64 - discriminant.sqrt()) / 2.0;
    let mut t2 = (time as f64 + discriminant.sqrt()) / 2.0;

    t1 = if t1 as usize as f64 == t1 {
        t1 + 1.0
    } else {
        t1.ceil()
    };
    t2 = if t2 as usize as f64 == t2 {
        t2 - 1.0
    } else {
        t2.floor()
    };

    (t2 - t1 + 1.0) as usize
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let (times, distances) = input_generator(input);

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| get_range(time, distance))
        .product()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let (times, distances) = input_generator(input);

    let (time, distance) = [times, distances]
        .into_iter()
        .map(|v| {
            v.iter()
                .map(|t| t.to_string())
                .join("")
                .parse::<usize>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    get_range(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 288);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 71503);
    }

    static INPUT: &str = include_str!("../input/2023/day6.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 345015);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 42588603);
    }
}
