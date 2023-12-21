use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Part {
    Part1 { steps: usize },
    Part2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
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
        self.x >= 0 && self.x < width as i32 && self.y >= 0 && self.y < height as i32
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub grid: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
    pub start: Pos,
}

impl Grid {
    pub fn is_valid(&self, pos: &Pos) -> bool {
        pos.is_in_bounds(self.width, self.height)
    }

    pub fn at(&self, pos: &Pos) -> char {
        self.grid[pos.y as usize][pos.x as usize]
    }
}

pub fn input_generator(input: &str) -> Grid {
    let mut grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let width = grid[0].len();
    let height = grid.len();

    let start = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == 'S')
                .map(move |(x, _)| Pos {
                    x: x as i32,
                    y: y as i32,
                })
        })
        .next()
        .unwrap();

    grid[start.y as usize][start.x as usize] = '.';
    Grid {
        grid,
        width,
        height,
        start,
    }
}

fn solve(part: Part, grid: &Grid) -> usize {
    let mut bfs = vec![vec![None; grid.height]; grid.width];

    bfs[grid.start.y as usize][grid.start.x as usize] = Some(0);

    let mut queue = VecDeque::new();
    queue.push_back(grid.start);
    while let Some(pos) = queue.pop_front() {
        let step = bfs[pos.y as usize][pos.x as usize].unwrap();
        let neighbours = [pos.up(), pos.down(), pos.left(), pos.right()];
        for neighbour in neighbours {
            if grid.is_valid(&neighbour)
                && grid.at(&neighbour) == '.'
                && bfs[neighbour.y as usize][neighbour.x as usize].map_or(true, |v| v > step + 1)
            {
                bfs[neighbour.y as usize][neighbour.x as usize] = Some(step + 1);
                queue.push_back(neighbour);
            }
        }
    }

    // // debug grid
    // for row in grid.grid.iter() {
    //     for col in row.iter() {
    //         print!("{}", col);
    //     }
    //     println!();
    // }

    // // debug bfs
    // for row in bfs.iter() {
    //     for col in row.iter() {
    //         if *col == usize::MAX {
    //             print!(" ");
    //         } else {
    //             print!("{}", col);
    //         }
    //     }
    //     println!();
    // }

    match part {
        Part::Part1 { steps } => bfs
            .iter()
            .flatten()
            .flatten()
            .filter(|&&s| s <= steps && s % 2 == 0)
            .count(),
        Part::Part2 => {
            let bfs = bfs.into_iter().flatten().flatten().collect_vec();

            let even_corners = bfs.iter().filter(|v| **v % 2 == 0 && **v > 65).count();
            let odd_corners = bfs.iter().filter(|v| **v % 2 == 1 && **v > 65).count();

            let even_full = bfs.iter().filter(|v| **v % 2 == 0).count();
            let odd_full = bfs.iter().filter(|v| **v % 2 == 1).count();

            // This is 202300 but im writing it out here to show the process
            let n = ((26501365 - (grid.width / 2)) / grid.height) as usize;
            assert_eq!(n, 202300);

            let even = n * n;
            let odd = (n + 1) * (n + 1);

            odd * odd_full + even * even_full - ((n + 1) * odd_corners) + (n * even_corners)
        }
    }
}

#[aoc(day21, part1)]
fn part1(input: &str) -> usize {
    let grid = input_generator(input);

    solve(Part::Part1 { steps: 64 }, &grid)
}

#[aoc(day21, part2)]
fn part2(input: &str) -> usize {
    let grid = input_generator(input);

    solve(Part::Part2, &grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn example_part1() {
        assert_eq!(
            solve(Part::Part1 { steps: 6 }, &input_generator(EXAMPLE)),
            16
        );
    }

    static INPUT: &str = include_str!("../input/2023/day21.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 3598);
    }

    #[test]
    fn input_part2() {
        // steps 26501365
        assert_eq!(part2(INPUT), 601441063166538);
    }
}
