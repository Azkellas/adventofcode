use itertools::Itertools;
use range_collections::{range_set::RangeSetRange, RangeSet, RangeSet2};
use scan_fmt::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

impl Pos {
    pub fn left(&self) -> Pos {
        Pos {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Pos {
        Pos {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn up(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn down(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn is_in_bounds(&self, width: usize, height: usize) -> bool {
        self.x >= 0 && self.x < width as i64 && self.y >= 0 && self.y < height as i64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    dir: Direction,
    len: i64,
}

fn input_generator(part: Part, input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|l| {
            let (dir, len, color) = scan_fmt!(l, "{} {} (#{})", String, i64, String).unwrap();
            match part {
                Part::Part1 => Move {
                    dir: match dir.as_str() {
                        "R" => Direction::East,
                        "L" => Direction::West,
                        "U" => Direction::North,
                        "D" => Direction::South,
                        _ => unreachable!(),
                    },
                    len,
                },
                Part::Part2 => {
                    let dir = color.chars().last().unwrap();
                    // remove last char
                    let color = &color[..color.len() - 1];
                    // parse as hexa
                    let len = i64::from_str_radix(color, 16).unwrap();
                    Move {
                        dir: match dir {
                            '0' => Direction::East,
                            '1' => Direction::South,
                            '2' => Direction::West,
                            '3' => Direction::North,
                            _ => unreachable!(),
                        },
                        len,
                    }
                }
            }
        })
        .collect_vec()
}

#[aoc(day18, part1)]
fn part1(input: &str) -> usize {
    let moves = input_generator(Part::Part1, input);

    let width = moves
        .iter()
        .filter(|m| m.dir == Direction::East || m.dir == Direction::West)
        .map(|m| m.len)
        .sum::<i64>()
        + 1;
    let height = moves
        .iter()
        .filter(|m| m.dir == Direction::North || m.dir == Direction::South)
        .map(|m| m.len)
        .sum::<i64>()
        + 1;

    let mut grid = vec![vec![false; width as usize]; height as usize];
    let mut pos = Pos {
        x: width / 2,
        y: height / 2,
    };
    grid[pos.x as usize][pos.y as usize] = true;

    for m in moves.iter() {
        for _ in 0..m.len {
            match m.dir {
                Direction::North => pos = pos.up(),
                Direction::South => pos = pos.down(),
                Direction::East => pos = pos.right(),
                Direction::West => pos = pos.left(),
            }
            grid[pos.x as usize][pos.y as usize] = true;
        }
    }

    let mut score = grid.iter().flatten().filter(|&&b| b).count();

    let get_next_start = |ff: &Vec<Vec<bool>>| {
        #[allow(clippy::needless_range_loop)]
        for y in 0..height {
            for x in 0..width {
                if !ff[y as usize][x as usize] {
                    return Some(Pos { x, y });
                }
            }
        }
        None
    };

    while let Some(start) = get_next_start(&grid) {
        let mut found_exit = false;
        let mut queue = vec![start];
        let mut seens = vec![];
        while let Some(pos) = queue.pop() {
            if pos.x == 0 || pos.x + 1 == width || pos.y == 0 || pos.y + 1 == height {
                found_exit = true;
            }

            if grid[pos.y as usize][pos.x as usize] {
                continue;
            }

            seens.push(pos);
            grid[pos.y as usize][pos.x as usize] = true;

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let x = pos.x + dx;
                let y = pos.y + dy;
                if !(Pos { x, y }.is_in_bounds(width as usize, height as usize)) {
                    continue;
                }
                if grid[y as usize][x as usize] {
                    continue;
                }
                queue.push(Pos { x, y });
            }
        }

        if !found_exit {
            score += seens.len();
        }
    }

    score
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SouthWall {
    pos: Pos,
    len: i64,
}

#[aoc(day18, part2)]
fn part2(input: &str) -> i64 {
    let moves = input_generator(Part::Part2, input);
    let mut walls = vec![];

    let mut pos = Pos { x: 0, y: 0 };
    let mut boundary_size = 0;
    for m in moves {
        match m.dir {
            Direction::North => {
                pos.y -= m.len;
                walls.push(SouthWall { pos, len: m.len });
            }
            Direction::South => {
                walls.push(SouthWall { pos, len: m.len });
                pos.y += m.len;
            }
            Direction::East => pos.x += m.len,
            Direction::West => pos.x -= m.len,
        }
        boundary_size += m.len;
    }
    assert_eq!(pos, Pos { x: 0, y: 0 });
    walls.sort_by(|a, b| a.pos.x.cmp(&b.pos.x));

    let leftest_x = walls[0].pos.x;

    let mut a: RangeSet2<i64> = RangeSet::from(walls[0].pos.y..walls[0].pos.y + walls[0].len);

    let mut score = 0;
    let mut current_x = leftest_x;

    for wall in walls.iter().skip(1) {
        if current_x != wall.pos.x {
            for x in a.iter() {
                match x {
                    RangeSetRange::Range(x) => {
                        score += (x.end - x.start) * (wall.pos.x - current_x);
                    }
                    RangeSetRange::RangeFrom(_) => {
                        unreachable!()
                    }
                }
            }
            current_x = wall.pos.x;
        }

        a ^= RangeSet::from(wall.pos.y..wall.pos.y + wall.len);
    }

    score + boundary_size / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 62);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 952408144115);
    }

    static INPUT: &str = include_str!("../input/2023/day18.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 42317);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 83605563360288);
    }
}
