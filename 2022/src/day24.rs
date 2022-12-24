use keyed_priority_queue::{Entry, KeyedPriorityQueue};
use std::{cmp::Reverse, collections::BTreeSet};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::Mul,
)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    fn get_delta(&self) -> Position {
        match *self {
            Self::East => Position { x: 1, y: 0 },
            Self::South => Position { x: 0, y: 1 },
            Self::West => Position { x: -1, y: 0 },
            Self::North => Position { x: 0, y: -1 },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Wind {
    pos: Position,
    dir: Direction,
}

#[derive(Clone, Debug)]
struct Map {
    width: i32,
    height: i32,
    start: Position,
    end: Position,
    winds: BTreeSet<Wind>,
}

impl Map {
    fn _debug(&self) {
        println!("from {:?} to {:?}", self.start, self.end);
        for y in 0..self.height {
            for x in 0..self.width {
                if x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1 {
                    print!(
                        "{}",
                        match self.is_free(&Position { x, y }) {
                            true => '.',
                            false => '#',
                        }
                    );
                } else {
                    print!(
                        "{}",
                        match self.is_free(&Position { x, y }) {
                            true => '.',
                            false => '~',
                        }
                    );
                }
            }
            println!();
        }
        println!();
    }
    fn is_free(&self, pos: &Position) -> bool {
        if pos.x == 0 || pos.x == self.width - 1 || pos.y == 0 || pos.y == self.height - 1 {
            return *pos == self.start || *pos == self.end;
        }

        !self.winds.contains(&Wind {
            pos: *pos,
            dir: Direction::North,
        }) && !self.winds.contains(&Wind {
            pos: *pos,
            dir: Direction::South,
        }) && !self.winds.contains(&Wind {
            pos: *pos,
            dir: Direction::West,
        }) && !self.winds.contains(&Wind {
            pos: *pos,
            dir: Direction::East,
        })
    }

    fn get_next_turn(&self) -> Self {
        let winds: BTreeSet<Wind> = self
            .winds
            .iter()
            .map(|wind| {
                let mut new_pos = wind.pos + wind.dir.get_delta();
                if new_pos.x == 0 {
                    new_pos.x = self.width - 2;
                }
                if new_pos.x == self.width - 1 {
                    new_pos.x = 1;
                }
                if new_pos.y == 0 {
                    new_pos.y = self.height - 2;
                }
                if new_pos.y == self.height - 1 {
                    new_pos.y = 1;
                }
                Wind {
                    pos: new_pos,
                    dir: wind.dir,
                }
            })
            .collect();

        Self {
            width: self.width,
            height: self.height,
            start: self.start,
            end: self.end,
            winds,
        }
    }
}
fn input_generator(input: &str) -> Map {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().chars().count() as i32;
    let start_x = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .position(|c| c == '.')
        .unwrap() as i32;
    let end_x = input
        .lines()
        .last()
        .unwrap()
        .chars()
        .position(|c| c == '.')
        .unwrap() as i32;

    let mut winds: BTreeSet<Wind> = BTreeSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Position {
                x: x as i32,
                y: y as i32,
            };
            match c {
                '>' => winds.insert(Wind {
                    pos,
                    dir: Direction::East,
                }),
                '<' => winds.insert(Wind {
                    pos,
                    dir: Direction::West,
                }),
                '^' => winds.insert(Wind {
                    pos,
                    dir: Direction::North,
                }),
                'v' => winds.insert(Wind {
                    pos,
                    dir: Direction::South,
                }),
                _ => continue,
            };
        }
    }

    let start = Position { x: start_x, y: 0 };
    let end = Position {
        x: end_x,
        y: height - 1,
    };
    Map {
        width,
        height,
        start,
        end,
        winds,
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    turn: usize,
    pos: Position,
}

fn solve(map: &Map, start: &Position, end: &Position) -> (usize, Map) {
    let mut queue: KeyedPriorityQueue<State, Reverse<usize>> = KeyedPriorityQueue::new();
    queue.push(
        State {
            turn: 0,
            pos: *start,
        },
        Reverse(0),
    );
    let mut maps = vec![map.clone()];

    let directions = [
        Direction::South,
        Direction::East,
        Direction::West,
        Direction::North,
    ];
    while let Some((State { turn, pos }, _)) = queue.pop() {
        if pos == *end {
            return (turn, maps[turn].clone());
        }

        while turn + 1 >= maps.len() {
            maps.push(maps[maps.len() - 1].get_next_turn());
        }

        let map = &maps[turn + 1];
        if map.is_free(&pos) {
            let new_state = State {
                turn: turn + 1,
                pos,
            };
            if let Entry::Vacant(_) = queue.entry(new_state) {
                queue.push(new_state, Reverse(turn + 1));
            }
        }
        for dir in directions {
            let neigh = pos + dir.get_delta();
            if map.is_free(&neigh) {
                let new_state = State {
                    turn: turn + 1,
                    pos: neigh,
                };
                if let Entry::Vacant(_) = queue.entry(new_state) {
                    queue.push(new_state, Reverse(turn + 1));
                }
            }
        }
    }

    unreachable!()
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> usize {
    let map = input_generator(input);
    solve(&map, &map.start, &map.end).0
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> usize {
    let map = input_generator(input);
    let (score1, map) = solve(&map, &map.start, &map.end);
    let (score2, map) = solve(&map, &map.end, &map.start);
    let (score3, _) = solve(&map, &map.start, &map.end);
    score1 + score2 + score3
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 18);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 54);
    }

    static INPUT: &str = include_str!("../input/2022/day24.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 221);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 739);
    }
}
