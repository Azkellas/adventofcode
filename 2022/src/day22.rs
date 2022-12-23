use std::collections::BTreeMap;
use itertools::Itertools;
use regex::Regex;
#[derive(Debug)]
enum Tile {
    Floor,
    Wall
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,     derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::Mul,
)]
struct Pos {
    x: i64,
    y: i64,
}

fn split_keep(r: &Regex, text: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(r) {
        if last != index {
            result.push(text[last..index].to_owned());
        }
        result.push(matched.to_owned());
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(text[last..].to_owned());
    }
    result
}

fn input_generator(input: &str) -> (BTreeMap<Pos, Tile>, Vec<String>) {
    let mut map = BTreeMap::new();
    let (map_data, commands) = input.split("\n\n").collect_tuple().unwrap();
    for (y, line) in map_data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {map.insert(Pos{x: x as i64, y: y as i64}, Tile::Floor);},
                '#' => {map.insert(Pos{x: x as i64, y: y as i64}, Tile::Wall);},
                ' ' => (),
                _ => unreachable!(),
            }
        }
    }
    
    let re = Regex::new(r"(\d+)").unwrap();
    let res = split_keep(&re, commands);
    (map, res)
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    East,
    South,
    West,
    North
}
impl Direction {
    fn go_left(&self) -> Self {
        match *self {
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::North => Self::West,
        }
    }
    fn go_right(&self) -> Self {
        match *self {
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::North => Self::East,
        }
    }
    fn get_delta(&self) -> Pos {
        match *self {
            Self::East => Pos{x: 1, y: 0},
            Self::South => Pos{x: 0, y: 1},
            Self::West => Pos{x: -1, y: 0},
            Self::North => Pos{x: 0, y: -1},
        }
    }
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> i64 {
    let (map, commands) = input_generator(input);

    let mut rowdims = vec![];
    let mut y = 0;
    loop {
        let min = map.iter()
            .map(|(k,_)| k)
            .filter(|&pos| pos.y == y)
            .map(|k| k.x)
            .min();
        let max = map.iter()
            .map(|(k,_)| k)
            .filter(|&pos| pos.y == y)
            .map(|k| k.x)
            .max();

        if let (Some(mi), Some(ma)) = (min, max) { 
            rowdims.push((mi, ma));
        }
        else {
            break;
        }
        y += 1;
    }
    let mut coldims = vec![];
    let mut x = 0;
    loop {
        let min = map.keys()
            .filter(|&pos| pos.x == x)
            .map(|k| k.y)
            .min();
        let max = map.keys()
            .filter(|&pos| pos.x == x)
            .map(|k| k.y)
            .max();

        if let (Some(mi), Some(ma)) = (min, max) { 
            coldims.push((mi, ma));
        }
        else {
            break;
        }
        x += 1;
    }

    let mut pos = Pos{x: rowdims[0].0, y: 0};
    let mut dir = Direction::East; 

    for command in commands {
        match command.as_str() {
            "L" => {dir = dir.go_left()},
            "R" => {dir = dir.go_right()},
            _ => {
                let steps = command.parse::<i64>().unwrap();
                for _ in 0..steps {
                    let next = pos + dir.get_delta();
                    if let Some(tile) = map.get(&next) {
                        match tile {
                            Tile::Floor => pos = next,
                            Tile::Wall => break,
                        }
                    }
                    else {
                        let mut next = pos;
                        match dir {
                            Direction::East => next.x = rowdims[next.y as usize].0,
                            Direction::West => next.x = rowdims[next.y as usize].1,
                            Direction::North => next.y = coldims[next.x as usize].1,
                            Direction::South => next.y = coldims[next.x as usize].0,
                        };
                        let tile = map.get(&next).unwrap();
                        match tile {
                            Tile::Floor => pos = next,
                            Tile::Wall => break,
                        }
                    }
                }
            }
        }
    }

    let (row, col, facing) = (pos.y + 1, pos.x + 1, dir as i64);
    1000 * row + 4 * col + facing
}

fn get_face(pos: &Pos, size: i64) -> usize {
    if size == 50 {
        // input
        match (pos.x / size, pos.y / size) {
            (2, 0) => 1,
            (1, 0) => 2,
            (1, 1) => 3,
            (1, 2) => 4,
            (0, 2) => 5,
            (0, 3) => 6,
            _ => unreachable!()
        }
    }
    else {
        // example
        match (pos.x / size, pos.y / size) {
            (2, 0) => 1,
            (0, 1) => 2,
            (1, 1) => 3,
            (2, 1) => 4,
            (2, 2) => 5,
            (3, 2) => 6,
            _ => unreachable!()
        }
    }
}

fn get_top_left(face: usize, size: i64) -> Pos {
    if size == 50 {
        // input
        match face {
            1 => Pos{x: 2 * size, y: 0 * size},
            2 => Pos{x: 1 * size, y: 0 * size},
            3 => Pos{x: 1 * size, y: 1 * size},
            4 => Pos{x: 1 * size, y: 2 * size},
            5 => Pos{x: 0 * size, y: 2 * size},
            6 => Pos{x: 0 * size, y: 3 * size},
            _ => unreachable!()
        }
    }
    else {
        // example
        match face {
            1 => Pos{x: 2 * size, y: 0 * size},
            2 => Pos{x: 0 * size, y: 1 * size},
            3 => Pos{x: 1 * size, y: 1 * size},
            4 => Pos{x: 2 * size, y: 1 * size},
            5 => Pos{x: 2 * size, y: 2 * size},
            6 => Pos{x: 3 * size, y: 2 * size},
            _ => unreachable!()
        }
    }

}
fn wrap(pos: &Pos, (face, dir): (usize, Direction), size: i64) -> (Pos, (usize, Direction)) {
    let mut toplefts = (1..=6).map(|face| get_top_left(face, size)).collect_vec();
    toplefts.insert(0, Pos{x:0, y:0});

    let delta = *pos - toplefts[face];

    if size == 50 {
        // input
        match (face, dir) {
            (1, Direction::East) => (toplefts[4] + Pos{x:size-1, y:size -1 - delta.y}, (4, Direction::West)),
            (4, Direction::East) => (toplefts[1] + Pos{x:size-1, y:size -1 - delta.y}, (1, Direction::West)),

            (1, Direction::North) => (toplefts[6] + Pos{x:delta.x, y:size-1}, (6, Direction::North)),
            (6, Direction::South) => (toplefts[1] + Pos{x:delta.x, y:0}, (1, Direction::South)),

            (1, Direction::South) => (toplefts[3] + Pos{x:size-1, y:delta.x}, (3, Direction::West)),
            (3, Direction::East) => (toplefts[1] + Pos{x:delta.y, y:size-1}, (1, Direction::North)),

            (2, Direction::North) => (toplefts[6] + Pos{x:0, y:delta.x}, (6, Direction::East)),
            (6, Direction::West) => (toplefts[2] + Pos{x:delta.y, y:0}, (2, Direction::South)),

            (3, Direction::West) => (toplefts[5] + Pos{x:delta.y, y:0}, (5, Direction::South)),
            (5, Direction::North) => (toplefts[3] + Pos{x:0, y:delta.x}, (3, Direction::East)),

            (4, Direction::South) => (toplefts[6] + Pos{x:size-1, y:delta.x}, (6, Direction::West)),
            (6, Direction::East) => (toplefts[4] + Pos{x:delta.y, y:size-1}, (4, Direction::North)),

            (2, Direction::West) => (toplefts[5] + Pos{x:0, y:size-1 - delta.y}, (5, Direction::East)),
            (5, Direction::West) => (toplefts[2] + Pos{x:0, y:size-1 - delta.y}, (2, Direction::East)),

            _ => unreachable!()
        }
    }
    else {
        // example
        match (face, dir) {
            (1, Direction::East) => (toplefts[6] + Pos{x:size-1, y:size - delta.y}, (6, Direction::West)),
            (6, Direction::East) => (toplefts[1] + Pos{x:size-1, y:size - delta.y}, (1, Direction::West)),

            (1, Direction::West) => (toplefts[3] + Pos{x:delta.y, y:0}, (3, Direction::South)),
            (3, Direction::North) => (toplefts[1] + Pos{x:0, y:delta.x}, (1, Direction::East)),

            (1, Direction::North) => (toplefts[2] + Pos{x:size-1-delta.x, y:0}, (2, Direction::South)),
            (2, Direction::North) => (toplefts[1] + Pos{x:size-1-delta.x, y:0}, (1, Direction::South)),

            (4, Direction::East) => (toplefts[6] + Pos{x:size-1 - delta.y, y:0}, (6, Direction::South)),
            (6, Direction::North) => (toplefts[4] + Pos{x:size-1, y:size-1 - delta.x}, (4, Direction::West)),

            (2, Direction::West) => (toplefts[6] + Pos{x:size-1 - delta.y, y:size-1}, (6, Direction::North)),
            (6, Direction::South) => (toplefts[2] + Pos{x:0, y:size-1 - delta.x}, (2, Direction::East)),

            (2, Direction::South) => (toplefts[5] + Pos{x:size-1-delta.x, y:size-1}, (5, Direction::North)),
            (5, Direction::South) => (toplefts[2] + Pos{x:size-1-delta.x, y:size-1}, (2, Direction::North)),

            (3, Direction::South) => (toplefts[5] + Pos{x:0, y:size-1 - delta.x}, (5, Direction::East)),
            (5, Direction::West) => (toplefts[3] + Pos{x:size-1 - delta.y, y:size-1}, (3, Direction::North)),

            _ => unreachable!()
        }
    }
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> i64 {
    let (map, commands) = input_generator(input);
    let size = f64::sqrt((map.len() / 6) as f64) as i64;


    let start_x = map.keys()
        .filter(|&pos| pos.y == 0)
        .map(|k| k.x)
        .min()
        .unwrap();
    let mut pos = Pos{x: start_x, y: 0};
    let mut dir = Direction::East;

    for command in commands {
        match command.as_str() {
            "L" => {dir = dir.go_left();},
            "R" => {dir = dir.go_right();},
            _ => {
                let steps = command.parse::<i64>().unwrap();
                for _ in 0..steps {
                    let next = pos + dir.get_delta();
                    if let Some(tile) = map.get(&next) {
                        match tile {
                            Tile::Floor => pos = next,
                            Tile::Wall => break,
                        }
                    }
                    else {
                        let face = get_face(&pos, size);
                        let (next_pos, (_next_face, next_dir)) = wrap(&pos, (face, dir), size);
                        let tile = map.get(&next_pos).unwrap();
                        match tile {
                            Tile::Floor => { pos = next_pos; dir = next_dir; },
                            Tile::Wall => break,
                        }
                    }
                }
            }
        }
    }

    let (row, col, facing) = (pos.y + 1, pos.x + 1, dir as i64);
    1000 * row + 4 * col + facing
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str =
"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn sample1() {
        assert_eq!(part1(EXAMPLE), 6032);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(EXAMPLE), 5031);
    }

    static INPUT: &str = include_str!("../input/2022/day22.txt");
    #[test]
    fn sample3() {
        assert_eq!(part1(INPUT), 164014);
    }

    #[test]
    fn sample4() {
        assert_eq!(part2(INPUT), 47525);
    }
}
