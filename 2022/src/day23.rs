use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,     derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::Mul,
)]
struct Pos {
    x: i64,
    y: i64,
}


#[derive(Copy, Clone, Debug)]
enum Direction {
    East,
    South,
    West,
    North
}
impl Direction {
    fn get_delta(&self) -> Pos {
        match *self {
            Self::East => Pos{x: 1, y: 0},
            Self::South => Pos{x: 0, y: 1},
            Self::West => Pos{x: -1, y: 0},
            Self::North => Pos{x: 0, y: -1},
        }
    }

    fn get_deltas(&self) -> Vec<Pos> {
        match *self {
            Self::East => (-1..=1).map(|y| Pos{x: 1, y}).collect(),
            Self::South => (-1..=1).map(|x| Pos{x, y: 1}).collect(),
            Self::West => (-1..=1).map(|y| Pos{x: -1, y}).collect(),
            Self::North => (-1..=1).map(|x| Pos{x, y: -1}).collect(),
        }
    }
}

fn input_generator(input: &str) -> BTreeSet<Pos> {
    let mut map = BTreeSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert(Pos{x: x as i64, y: y as i64});
            }
        }
    }
    map
}

#[derive(PartialEq, Eq)]
enum Puzzle {
    Part1,
    Part2,
}

fn _debug(elves: &BTreeSet<Pos>, step: i64) {
    // debug
    let min_x = elves.iter().map(|p| p.x).min().unwrap();
    let max_x = elves.iter().map(|p| p.x).max().unwrap();
    let min_y = elves.iter().map(|p| p.y).min().unwrap();
    let max_y = elves.iter().map(|p| p.y).max().unwrap();
    println!("step: {step}");
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if elves.contains(&Pos{x, y}) {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn solve(input: &str, part: Puzzle) -> i64 {
    let mut elves = input_generator(input);


    let dirs = [Direction::North, Direction::South, Direction::West, Direction::East];
    let all_deltas: BTreeSet<Pos> = dirs.iter().flat_map(|d| d.get_deltas()).collect();

    let step_count = match part {
        Puzzle::Part1 => 10,
        Puzzle::Part2 => i64::MAX,
    };

    for step in 0..step_count {
        let mut moves = vec![];
        for elf in &elves {
            let neighbours = all_deltas.iter().filter(|&delta| elves.contains(&(*elf + *delta))).count();
            if neighbours > 0 {
                for i in 0..4 {
                    let dir = dirs[(step as usize + i) % 4];
                    let deltas = dir.get_deltas();
                    if deltas.iter().all(|&delta| !elves.contains(&(*elf + delta))) {
                        moves.push((*elf, *elf + dir.get_delta()));
                        break;
                    }
                }
            }
        }

        if part == Puzzle::Part2 && moves.is_empty() {
            return step + 1;
        }

        for (from, to) in &moves {
            let count = moves.iter().filter(|(_, t)| t == to).count();
            if count == 1 {
                elves.remove(from);
                elves.insert(*to);
            }
        }
        
        // debug(&elves, step + 1);
    }


    // solve part 1
    let min_x = elves.iter().map(|p| p.x).min().unwrap();
    let max_x = elves.iter().map(|p| p.x).max().unwrap();
    let min_y = elves.iter().map(|p| p.y).min().unwrap();
    let max_y = elves.iter().map(|p| p.y).max().unwrap();
    (max_x + 1 - min_x) * (max_y + 1 - min_y) - elves.len() as i64

}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> i64 {
    solve(input, Puzzle::Part1)
}


#[aoc(day23, part2)]
pub fn part2(input: &str) -> i64 {
    solve(input, Puzzle::Part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "`
..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    #[test]
    fn sample1() {
        assert_eq!(part1(EXAMPLE), 110);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(EXAMPLE), 20);
    }

    static INPUT: &str = include_str!("../input/2022/day23.txt");
    #[test]
    fn sample3() {
        assert_eq!(part1(INPUT), 4049);
    }

    #[test]
    fn sample4() {
        assert_eq!(part2(INPUT), 1021);
    }
}
