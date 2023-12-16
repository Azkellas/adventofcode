use std::collections::VecDeque;

use bitmask_enum::bitmask;
use itertools::Itertools;

#[bitmask]
#[bitmask_config(vec_debug)]
enum Direction {
    None,
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Map {
    grid: Vec<Vec<char>>,
    width: i32,
    height: i32,
}

impl Map {
    fn get(&self, x: i32, y: i32) -> char {
        self.grid[y as usize][x as usize]
    }

    fn is_inside(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }
}

fn input_generator(input: &str) -> Map {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    Map {
        grid,
        width,
        height,
    }
}

fn solve(map: &Map, (x, y): (i32, i32), dir: Direction) -> usize {
    let mut seen = vec![vec![Direction::None; map.width as usize]; map.height as usize];

    let mut queue = VecDeque::new();

    queue.push_back((x, y, dir));

    while let Some((x, y, dir)) = queue.pop_front() {
        if map.is_inside(x, y) && seen[y as usize][x as usize] & dir != 0 {
            continue;
        }

        if map.is_inside(x, y) {
            seen[y as usize][x as usize] |= dir;
        }

        match dir {
            Direction::North => {
                if y > 0 {
                    match map.get(x, y - 1) {
                        '-' => {
                            queue.push_back((x, y - 1, Direction::West));
                            queue.push_back((x, y - 1, Direction::East));
                        }
                        '/' => queue.push_back((x, y - 1, Direction::East)),
                        '\\' => queue.push_back((x, y - 1, Direction::West)),
                        _ => queue.push_back((x, y - 1, Direction::North)),
                    }
                }
            }
            Direction::South => {
                if y + 1 < map.height {
                    match map.get(x, y + 1) {
                        '-' => {
                            queue.push_back((x, y + 1, Direction::West));
                            queue.push_back((x, y + 1, Direction::East));
                        }
                        '/' => queue.push_back((x, y + 1, Direction::West)),
                        '\\' => queue.push_back((x, y + 1, Direction::East)),
                        _ => queue.push_back((x, y + 1, Direction::South)),
                    }
                }
            }
            Direction::East => {
                if x + 1 < map.width {
                    match map.get(x + 1, y) {
                        '|' => {
                            queue.push_back((x + 1, y, Direction::North));
                            queue.push_back((x + 1, y, Direction::South));
                        }
                        '/' => queue.push_back((x + 1, y, Direction::North)),
                        '\\' => queue.push_back((x + 1, y, Direction::South)),
                        _ => queue.push_back((x + 1, y, Direction::East)),
                    }
                }
            }
            Direction::West => {
                if x > 0 {
                    match map.get(x - 1, y) {
                        '|' => {
                            queue.push_back((x - 1, y, Direction::North));
                            queue.push_back((x - 1, y, Direction::South));
                        }
                        '/' => queue.push_back((x - 1, y, Direction::South)),
                        '\\' => queue.push_back((x - 1, y, Direction::North)),
                        _ => queue.push_back((x - 1, y, Direction::West)),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    // debug seen
    // for row in seen.iter() {
    //     for dir in row.iter() {
    //         print!("{} ", if *dir != Direction::None { '#' } else { '.' });
    //     }
    //     println!();
    // }

    seen.iter()
        .map(|row| row.iter().filter(|&&d| d != Direction::None).count())
        .sum()
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    let map = input_generator(input);
    solve(&map, (-1, 0), Direction::East)
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    let map = input_generator(input);
    let mut best_score = 0;

    for y in 0..map.height {
        best_score = best_score.max(solve(&map, (-1, y), Direction::East));
        best_score = best_score.max(solve(&map, (map.width, y), Direction::West));
    }
    for x in 0..map.width {
        best_score = best_score.max(solve(&map, (x, -1), Direction::South));
        best_score = best_score.max(solve(&map, (x, map.height), Direction::North));
    }

    best_score
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 46);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 51);
    }

    static INPUT: &str = include_str!("../input/2023/day16.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 7951);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 8148);
    }
}
