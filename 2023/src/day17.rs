use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Move {
    x: i32,
    y: i32,
    direction: Direction,
    last_turn: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct HeapMove(Move, i32);

impl Ord for HeapMove {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1).reverse()
    }
}

impl PartialOrd for HeapMove {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Map {
    grid: Vec<Vec<i32>>,
    width: i32,
    height: i32,
}

impl Map {
    fn get(&self, x: i32, y: i32) -> i32 {
        self.grid[y as usize][x as usize]
    }

    fn is_inside(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }
}

fn input_generator(input: &str) -> Map {
    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect_vec();
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    Map {
        grid,
        width,
        height,
    }
}

fn solve(map: &Map, test_direction: fn(&Direction, &Direction, i32) -> bool) -> i32 {
    let mut heap = BinaryHeap::new();
    heap.push(HeapMove(
        Move {
            x: 0,
            y: 0,
            direction: Direction::East,
            last_turn: 0,
        },
        0,
    ));

    let mut seens = HashMap::new();

    let delta_north = (0, -1, Direction::North);
    let delta_south = (0, 1, Direction::South);
    let delta_east = (1, 0, Direction::East);
    let delta_west = (-1, 0, Direction::West);

    let deltas_north = [&delta_west, &delta_east, &delta_north];
    let deltas_south = [&delta_west, &delta_east, &delta_south];
    let deltas_east = [&delta_north, &delta_south, &delta_east];
    let deltas_west = [&delta_north, &delta_south, &delta_west];

    while let Some(HeapMove(mve, cost)) = heap.pop() {
        // end.
        if mve.x == map.width - 1 && mve.y == map.height - 1 {
            return cost;
        }

        // worse solution.
        if let Some(old_cost) = seens.get(&mve) {
            if cost >= *old_cost {
                continue;
            }
        }
        seens.insert(mve.clone(), cost);

        let deltas = match mve.direction {
            Direction::North => &deltas_north,
            Direction::South => &deltas_south,
            Direction::East => &deltas_east,
            Direction::West => &deltas_west,
        };

        for delta in deltas {
            let (dx, dy, dir) = delta;
            let (x, y) = (mve.x + dx, mve.y + dy);
            if map.is_inside(x, y) && test_direction(dir, &mve.direction, mve.last_turn) {
                heap.push(HeapMove(
                    Move {
                        x,
                        y,
                        direction: dir.clone(),
                        last_turn: if *dir == mve.direction {
                            mve.last_turn + 1
                        } else {
                            0
                        },
                    },
                    cost + map.get(x, y),
                ));
            }
        }
    }

    unreachable!("no solution found")
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> i32 {
    let map = input_generator(input);

    let valid_dir = |new_dir: &Direction, dir: &Direction, steps: i32| new_dir != dir || steps < 2;

    solve(&map, valid_dir)
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> i32 {
    let map: Map = input_generator(input);

    let valid_dir = |new_dir: &Direction, dir: &Direction, steps: i32| {
        if new_dir == dir {
            steps < 9
        } else {
            steps > 2
        }
    };

    solve(&map, valid_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 102);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 94);
    }

    static INPUT: &str = include_str!("../input/2023/day17.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 953);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 1180);
    }
}
