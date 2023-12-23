use std::collections::{BTreeMap, BTreeSet, VecDeque};

use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
}

impl Grid {
    pub fn is_valid(&self, pos: &Pos) -> bool {
        pos.is_in_bounds(self.width, self.height)
    }

    pub fn at(&self, pos: &Pos) -> char {
        self.grid[pos.y as usize][pos.x as usize]
    }

    pub fn is_walkable(&self, pos: &Pos) -> bool {
        pos.is_in_bounds(self.width, self.height) && self.at(pos) != '#'
    }
}

fn input_generator(input: &str) -> (Grid, Pos, Pos) {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let width = grid[0].len();
    let height = grid.len();

    let start_x = grid[0].iter().position(|c| *c == '.').unwrap();
    let end_x = grid[height - 1].iter().position(|c| *c == '.').unwrap();
    (
        Grid {
            grid,
            width,
            height,
        },
        Pos {
            x: start_x as i32,
            y: 0,
        },
        Pos {
            x: end_x as i32,
            y: (height - 1) as i32,
        },
    )
}

fn backtrack(grid: &Grid, pos: &Pos, target: &Pos, path: &mut BTreeSet<Pos>) -> usize {
    if pos == target {
        return path.len();
    }

    let mut max_dist = 0;
    let neighbours = [pos.up(), pos.down(), pos.right(), pos.left()];

    for neighbour in neighbours {
        if grid.is_valid(&neighbour) && !path.contains(&neighbour) {
            let char = grid.at(&neighbour);
            let valid_dir = match char {
                '.' => true,
                '>' if neighbour == pos.right() => true,
                '<' if neighbour == pos.left() => true,
                '^' if neighbour == pos.up() => true,
                'v' if neighbour == pos.down() => true,
                _ => false,
            };

            if valid_dir {
                path.insert(neighbour);
                max_dist = max_dist.max(backtrack(grid, &neighbour, target, path));
                path.remove(&neighbour);
            }
        }
    }

    max_dist
}

#[aoc(day23, part1)]
fn part1(input: &str) -> usize {
    let (grid, start, end) = input_generator(input);
    backtrack(&grid, &start, &end, &mut BTreeSet::from([start])) - 1
}

fn link_portal(grid: &Grid, portals: &BTreeSet<Pos>, portal: &Pos) -> BTreeSet<(Pos, usize)> {
    let mut res = BTreeSet::new();

    let mut bfs = vec![vec![None; grid.height]; grid.width];

    bfs[portal.y as usize][portal.x as usize] = Some(0);

    let mut queue = VecDeque::new();
    queue.push_back(*portal);
    while let Some(pos) = queue.pop_front() {
        let step = bfs[pos.y as usize][pos.x as usize].unwrap();
        if portals.contains(&pos) && pos != *portal {
            res.insert((pos, step));
            continue;
        }

        let neighbours = [pos.up(), pos.down(), pos.left(), pos.right()];
        for neighbour in neighbours {
            if grid.is_walkable(&neighbour)
                && bfs[neighbour.y as usize][neighbour.x as usize].map_or(true, |v| v > step + 1)
            {
                bfs[neighbour.y as usize][neighbour.x as usize] = Some(step + 1);
                queue.push_back(neighbour);
            }
        }
    }

    res
}

fn backtrack_p2(
    portals: &BTreeMap<Pos, BTreeSet<(Pos, usize)>>,
    pos: &Pos,
    target: &Pos,
    path: &mut BTreeSet<Pos>,
    length: usize,
) -> usize {
    if pos == target {
        return length;
    }

    let mut res = 0;
    for (portal, dist) in &portals[pos] {
        if !path.contains(portal) {
            path.insert(*portal);
            res = res.max(backtrack_p2(portals, portal, target, path, length + dist));
            path.remove(portal);
        }
    }

    res
}

#[aoc(day23, part2)]
fn part2(input: &str) -> usize {
    let (grid, start, end) = input_generator(input);

    let mut portals: BTreeSet<Pos> = grid
        .grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|&(x, c)| {
                    if *c == '#' {
                        return false;
                    }
                    let pos = Pos {
                        x: x as i32,
                        y: y as i32,
                    };
                    let neighbours = [pos.up(), pos.down(), pos.right(), pos.left()];
                    let valids = neighbours
                        .iter()
                        .filter(|neigh| grid.is_walkable(&neigh))
                        .count();

                    valids > 2
                })
                .map(move |(x, _)| Pos {
                    x: x as i32,
                    y: y as i32,
                })
                .collect_vec()
        })
        .collect();

    portals.insert(start);
    portals.insert(end);

    let portals: BTreeMap<Pos, BTreeSet<(Pos, usize)>> = portals
        .iter()
        .map(|portal| (*portal, link_portal(&grid, &portals, portal)))
        .collect();

    backtrack_p2(&portals, &start, &end, &mut BTreeSet::from([start]), 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 94);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 154);
    }

    static INPUT: &str = include_str!("../input/2023/day23.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 2018);
    }

    #[test]
    fn input_part2() {
        // steps 26501365
        assert_eq!(part2(INPUT), 6406);
    }
}
