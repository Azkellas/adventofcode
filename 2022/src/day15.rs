use itertools::Itertools;
use scan_fmt::*;

pub struct Pair {
    sensor: (i32, i32),
    beacon: (i32, i32),
    dst: usize,
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Interval {
    from: i32,
    to: i32,
    value: i32,
}

fn distance(lhs: (i32, i32), rhs: (i32, i32)) -> i32 {
    i32::abs(lhs.0 - rhs.0) + i32::abs(lhs.1 - rhs.1)
}

fn valid_point((x, y): (i32, i32), size: i32) -> bool {
    0 <= x && x < size && 0 <= y && y < size
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> (Vec<Pair>, bool) {
    let test = input.lines().count() < 15;

    let pairs = input
        .lines()
        .map(|line| {
            let (sx, sy, bx, by) = scan_fmt!(
                line,
                "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                i32,
                i32,
                i32,
                i32
            )
            .unwrap();
            let dst = (i32::abs(sx - bx) + i32::abs(sy - by)) as usize;
            Pair {
                beacon: (bx, by),
                sensor: (sx, sy),
                dst,
            }
        })
        .collect_vec();

    (pairs, test)
}

#[must_use]
#[aoc(day15, part1)]
pub fn part1((pairs, test): &(Vec<Pair>, bool)) -> i32 {
    // hack to work with both example and input
    let row = if *test { 10 } else { 2_000_000 };

    let raw_intervals: Vec<Interval> = pairs
        .iter()
        .filter(|pair| i32::abs(row - pair.sensor.1) <= pair.dst as i32)
        .map(|pair| {
            let dy = i32::abs(row - pair.sensor.1);
            Interval {
                from: pair.sensor.0 - (pair.dst as i32 - dy),
                to: pair.sensor.0 + (pair.dst as i32 - dy),
                value: 1,
            }
        })
        .collect();

    let mut intervals: Vec<Interval> = vec![];
    for ni in raw_intervals {
        let interval_count = intervals.len();
        for idx in 0..interval_count {
            let i = &intervals[idx];
            if i.from <= ni.from && ni.to <= i.to {
                intervals.push(Interval {
                    from: ni.from,
                    to: ni.to,
                    value: -i.value,
                });
            } else if ni.from <= i.from && i.to <= ni.to {
                intervals.push(Interval {
                    from: i.from,
                    to: i.to,
                    value: -i.value,
                });
            } else {
                let interval = Interval {
                    from: i32::max(i.from, ni.from),
                    to: i32::min(i.to, ni.to),
                    value: -i.value,
                };
                if interval.from <= interval.to {
                    intervals.push(interval);
                }
            }
        }
        intervals.push(ni);
    }

    let beacons = pairs
        .iter()
        .map(|pair| pair.beacon)
        .unique()
        .filter(|(_, y)| *y == row)
        .count();
    let visibles: i32 = intervals
        .iter()
        .map(|interval| interval.value * (interval.to - interval.from + 1))
        .sum();
    visibles - beacons as i32
}

#[must_use]
#[aoc(day15, part2)]
pub fn part2((pairs, test): &(Vec<Pair>, bool)) -> i64 {
    // hack to work with both example and input
    let limit_size = if *test { 20 } else { 4_000_000 };

    for Pair {
        sensor,
        beacon: _,
        dst,
    } in pairs
    {
        // top-right
        let dst = 1 + *dst as i32;
        let mut pt = (sensor.0, sensor.1 - dst);
        while valid_point(pt, limit_size) && pt.0 <= sensor.0 + dst {
            let seen = pairs
                .iter()
                .any(|pair| distance(pair.sensor, pt) <= pair.dst as i32);
            if !seen {
                return 4_000_000 * pt.0 as i64 + pt.1 as i64;
            }
            pt.0 += 1;
            pt.1 += 1;
        }

        // top-left
        let mut pt = (sensor.0, sensor.1 - dst);
        while valid_point(pt, limit_size) && pt.0 >= sensor.0 - dst {
            let seen = pairs
                .iter()
                .any(|pair| distance(pair.sensor, pt) <= pair.dst as i32);
            if !seen {
                return 4_000_000 * pt.0 as i64 + pt.1 as i64;
            }
            pt.0 -= 1;
            pt.1 += 1;
        }

        // bottom-right
        let mut pt = (sensor.0, sensor.1 + dst);
        while valid_point(pt, limit_size) && pt.0 <= sensor.0 + dst {
            let seen = pairs
                .iter()
                .any(|pair| distance(pair.sensor, pt) <= pair.dst as i32);
            if !seen {
                return 4_000_000 * pt.0 as i64 + pt.1 as i64;
            }
            pt.0 += 1;
            pt.1 -= 1;
        }

        // bottom-left
        let mut pt = (sensor.0, sensor.1 + dst);
        while valid_point(pt, limit_size) && pt.0 >= sensor.0 - dst {
            let seen = pairs
                .iter()
                .any(|pair| distance(pair.sensor, pt) <= pair.dst as i32);
            if !seen {
                return 4_000_000 * pt.0 as i64 + pt.1 as i64;
            }
            pt.0 -= 1;
            pt.1 -= 1;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 26);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 56000011);
    }
}
