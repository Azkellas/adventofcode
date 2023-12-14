use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

use itertools::Itertools;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Map {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn get_score(&self) -> usize {
        let mut score = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == 'O' {
                    score += self.height - y;
                }
            }
        }
        score
    }

    fn move_north(&mut self) -> usize {
        let mut score = 0;
        for x in 0..self.width {
            let mut empty_places = VecDeque::new();
            for y in 0..self.height {
                #[allow(clippy::match_on_vec_items)]
                match self.grid[y][x] {
                    '.' => empty_places.push_back(y),
                    '#' => empty_places.clear(),
                    'O' => {
                        let next_empty_place = empty_places.pop_front();
                        if let Some(next_empty_place) = next_empty_place {
                            self.grid[next_empty_place][x] = 'O';
                            self.grid[y][x] = '.';
                            empty_places.push_back(y);

                            score += self.height - next_empty_place;
                        } else {
                            score += self.height - y;
                        }
                    }
                    _ => unreachable!(),
                };
            }
        }

        score
    }

    fn move_south(&mut self) -> usize {
        let mut score = 0;
        for x in 0..self.width {
            let mut empty_places = VecDeque::new();
            for y in (0..self.height).rev() {
                #[allow(clippy::match_on_vec_items)]
                match self.grid[y][x] {
                    '.' => empty_places.push_back(y),
                    '#' => empty_places.clear(),
                    'O' => {
                        let next_empty_place = empty_places.pop_front();
                        if let Some(next_empty_place) = next_empty_place {
                            self.grid[next_empty_place][x] = 'O';
                            self.grid[y][x] = '.';
                            empty_places.push_back(y);

                            score += self.height - next_empty_place;
                        } else {
                            score += self.height - y;
                        }
                    }
                    _ => unreachable!(),
                };
            }
        }

        score
    }

    fn move_west(&mut self) -> usize {
        let mut score = 0;
        for y in 0..self.height {
            let mut empty_places = VecDeque::new();
            for x in 0..self.width {
                #[allow(clippy::match_on_vec_items)]
                match self.grid[y][x] {
                    '.' => empty_places.push_back(x),
                    '#' => empty_places.clear(),
                    'O' => {
                        let next_empty_place = empty_places.pop_front();
                        if let Some(next_empty_place) = next_empty_place {
                            self.grid[y][next_empty_place] = 'O';
                            self.grid[y][x] = '.';
                            empty_places.push_back(x);
                        }
                        score += self.height - y;
                    }
                    _ => unreachable!(),
                };
            }
        }

        score
    }

    fn move_east(&mut self) -> usize {
        let mut score = 0;
        for y in 0..self.height {
            let mut empty_places = VecDeque::new();
            for x in (0..self.width).rev() {
                #[allow(clippy::match_on_vec_items)]
                match self.grid[y][x] {
                    '.' => empty_places.push_back(x),
                    '#' => empty_places.clear(),
                    'O' => {
                        let next_empty_place = empty_places.pop_front();
                        if let Some(next_empty_place) = next_empty_place {
                            self.grid[y][next_empty_place] = 'O';
                            self.grid[y][x] = '.';
                            empty_places.push_back(x);
                        }
                        score += self.height - y;
                    }
                    _ => unreachable!(),
                };
            }
        }

        score
    }
}

fn input_generator(input: &str) -> Map {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let width = grid[0].len();
    let height = grid.len();
    Map {
        grid,
        width,
        height,
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    input_generator(input).move_north()
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let mut map = input_generator(input);

    let mut seens = HashMap::new();
    let mut idx = 1;
    while idx <= 1_000_000_000 {
        map.move_north();
        map.move_west();
        map.move_south();
        map.move_east();

        if seens.contains_key(&map) {
            let prev_idx = seens.get(&map).unwrap();
            let cycle = idx - prev_idx;
            let remaining = (1_000_000_000 - idx) % cycle;

            for _ in 0..remaining {
                map.move_north();
                map.move_west();
                map.move_south();
                map.move_east();
            }
            return map.get_score();
            // see
        }

        seens.insert(map.clone(), idx);
        idx += 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 136);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 64);
    }

    static INPUT: &str = include_str!("../input/2023/day14.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 108889);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 104671);
    }
}
