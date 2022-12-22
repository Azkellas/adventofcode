use itertools::Itertools;
use std::collections::BTreeSet;

#[derive(
    Debug,
    Clone,
    Copy,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::Mul,
)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}

#[must_use]
#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> BTreeSet<Point> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            line.split(" -> ")
                .map(|s| {
                    s.split(',')
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .map(|(x, y)| Point { x, y })
                .collect::<Vec<Point>>()
        })
        .flat_map(|line| {
            line.into_iter()
                .tuple_windows()
                .map(|(from, to)| {
                    let delta = to - from;
                    let step = delta.signum();
                    let steps = i32::max(delta.x.abs(), delta.y.abs());
                    (0..=steps).map(move |i| from + step * i)
                })
                .flatten()
        })
        .collect::<BTreeSet<Point>>()
}

#[must_use]
pub fn drop_sand(mut rocks: BTreeSet<Point>) -> usize {
    let start_point = Point { x: 500, y: 0 };
    let max_y = rocks.iter().map(|s| s.y).max().unwrap();
    let deltas = [
        Point { x: 0, y: 1 },
        Point { x: -1, y: 1 },
        Point { x: 1, y: 1 },
    ];
    let mut steps = 0;

    'spawn: loop {
        if rocks.contains(&start_point) {
            break;
        }

        let mut sand = start_point;
        loop {
            if sand.y == max_y {
                break 'spawn;
            }

            if !deltas.iter().any(|delta| {
                let found = !rocks.contains(&(sand + *delta));
                if found {
                    sand += *delta;
                }
                found
            }) {
                rocks.insert(sand);
                break;
            }
        }

        steps += 1;
    }

    steps
}

#[must_use]
#[aoc(day14, part1)]
pub fn part1(rocks: &BTreeSet<Point>) -> usize {
    drop_sand(rocks.clone())
}

#[must_use]
#[aoc(day14, part2)]
pub fn part2(rocks: &BTreeSet<Point>) -> usize {
    let mut rocks = rocks.clone();

    let max_y = rocks.iter().map(|s| s.y).max().unwrap();
    for x in 0..1000 {
        rocks.insert(Point { x, y: max_y + 2 });
    }
    drop_sand(rocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 24);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 93);
    }

    static INPUT: &str = include_str!("../input/2022/day14.txt");
    #[test]
    fn sample3() {
        assert_eq!(part1(&input_generator(INPUT)), 768);
    }

    #[test]
    fn sample4() {
        assert_eq!(part2(&input_generator(INPUT)), 26686);
    }
}
