use nalgebra::Vector2;
use scan_fmt::*;
use std::collections::HashSet;

fn solve(input: &str, length: usize) -> usize {
    let mut seen: HashSet<Vector2<i32>> = HashSet::new();
    let mut rope: Vec<Vector2<i32>> = vec![Vector2::<i32>::zeros(); length];
    seen.insert(rope[length - 1]);

    input
        .lines()
        .into_iter()
        .map(|line| scan_fmt!(line, "{} {}\n", char, i32).unwrap())
        .for_each(|(d, c)| {
            let delta: Vector2<i32> = match d {
                'U' => Vector2::<i32>::new(0, -1),
                'L' => Vector2::<i32>::new(-1, 0),
                'D' => Vector2::<i32>::new(0, 1),
                'R' => Vector2::<i32>::new(1, 0),
                _ => unreachable!(),
            };

            for _ in 0..c {
                rope[0] += delta;
                for i in 0..length - 1 {
                    let v: Vector2<i32> = rope[i] - rope[i + 1];
                    if v.amax() >= 2 {
                        rope[i + 1] += v.map(i32::signum);
                    }
                }
                seen.insert(rope[length - 1]);
            }
        });

    seen.len()
}
#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, 2)
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    solve(input, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let example = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(part1(example), 13);
    }

    #[test]
    fn example_part2() {
        let example = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(part2(example), 36);
    }

    static INPUT: &str = include_str!("../input/2022/day9.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 6175);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 2578);
    }
}
