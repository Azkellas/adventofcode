use std::collections::{BTreeSet, VecDeque};

use itertools::iproduct;
use scan_fmt::*;

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
    z: i32,
}

impl Point {
    fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
}

pub fn input_generator(input: &str) -> BTreeSet<Point> {
    input
        .lines()
        .map(|line| scan_fmt!(line, "{},{},{}", i32, i32, i32))
        .map(|opt| opt.unwrap())
        .map(|(x, y, z)| Point { x, y, z })
        .collect::<BTreeSet<Point>>()
}

fn compute_perimeter(cubes: &BTreeSet<Point>) -> usize {
    let total = cubes.len() * 6;
    let hidden = iproduct!(cubes, cubes)
        .filter(|(cube1, cube2)| {
            let delta = **cube1 - **cube2;
            let adelta = delta.abs();
            adelta.x + adelta.y + adelta.z == 1
        })
        .count();

    total - hidden
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> usize {
    let cubes = input_generator(input);
    compute_perimeter(&cubes)
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> usize {
    let cubes = input_generator(input);
    let max_coord = cubes
        .iter()
        .map(|p| i32::max(p.x, i32::max(p.y, p.z)))
        .max()
        .unwrap();

    let mut total = compute_perimeter(&cubes);
    let mut seen = cubes.clone();

    let deltas = [
        Point { x: -1, y: 0, z: 0 },
        Point { x: 1, y: 0, z: 0 },
        Point { x: 0, y: -1, z: 0 },
        Point { x: 0, y: 1, z: 0 },
        Point { x: 0, y: 0, z: -1 },
        Point { x: 0, y: 0, z: 1 },
    ];
    iproduct!(0..max_coord, 0..max_coord, 0..max_coord).for_each(|(x, y, z)| {
        let start = Point { x, y, z };
        if !seen.contains(&start) {
            let mut queue = VecDeque::new();
            queue.push_back(start);
            seen.insert(start);
            let mut new_cubes = BTreeSet::new();
            let mut is_inside = true;
            while let Some(point) = queue.pop_front() {
                new_cubes.insert(point);
                if point.x < 0
                    || point.y < 0
                    || point.z < 0
                    || point.x >= max_coord
                    || point.y >= max_coord
                    || point.z >= max_coord
                {
                    is_inside = false;
                    continue;
                }
                for delta in &deltas {
                    let neigh = point + *delta;
                    if !seen.contains(&neigh) {
                        seen.insert(neigh);
                        queue.push_back(neigh);
                    }
                }
            }

            if is_inside {
                total -= compute_perimeter(&new_cubes);
            }
        }
    });

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn sample1() {
        assert_eq!(part1(EXAMPLE), 64);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(EXAMPLE), 58);
    }

    static INPUT: &str = include_str!("../input/2022/day18.txt");
    #[test]
    fn sample3() {
        assert_eq!(part1(INPUT), 3650);
    }

    #[test]
    fn sample4() {
        assert_eq!(part2(INPUT), 2118);
    }
}
