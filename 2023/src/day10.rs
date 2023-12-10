use itertools::Itertools;

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

    pub fn set_start(&mut self, c: char) {
        self.grid[self.start.y as usize][self.start.x as usize] = c;
    }

    pub fn get_neighbours(&self, pos: &Pos) -> Option<(Pos, Pos)> {
        match self.at(pos) {
            '-' => Some((Pos::left(pos), Pos::right(pos))),
            '|' => Some((Pos::up(pos), Pos::down(pos))),
            'J' => Some((Pos::left(pos), Pos::up(pos))),
            'F' => Some((Pos::right(pos), Pos::down(pos))),
            '7' => Some((Pos::left(pos), Pos::down(pos))),
            'L' => Some((Pos::up(pos), Pos::right(pos))),
            _ => None,
        }
        .filter(|(p1, p2)| self.is_valid(p1) && self.is_valid(p2))
    }

    pub fn get_next_position(&self, pos: &Pos, prev: &Pos) -> Option<Pos> {
        self.get_neighbours(pos).and_then(|(neigh1, neigh2)| {
            match *prev {
                p if p == neigh1 => Some(neigh2),
                p if p == neigh2 => Some(neigh1),
                p if p == *pos => Some(neigh1), // arbitrary choice at start point
                _ => None,
            }
        })
    }
}

pub fn input_generator(input: &str) -> Grid {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
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

    Grid {
        grid,
        width,
        height,
        start,
    }
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    let input_grid: Grid = input_generator(input);

    let mut max_length = 0;
    for c in ['-', '|', 'L', 'J', 'F', '7'] {
        let mut grid: Grid = input_grid.clone();
        grid.set_start(c);

        let mut pos = grid.start;
        let mut previous_pos = grid.start;
        let mut length = 0;
        while pos != grid.start || length == 0 {
            let next_pos = grid.get_next_position(&pos, &previous_pos);
            if let Some(next_pos) = next_pos {
                previous_pos = pos;
                pos = next_pos;
                length += 1;
            } else {
                break;
            }
        }
        if pos == grid.start {
            length /= 2;
            // println!("{}: {}", c, length);
            max_length = max_length.max(length);
        }
    }

    max_length
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    let input_grid: Grid = input_generator(input);

    let mut max_length = 0;
    let mut max_seens = vec![];
    let mut max_grid = input_grid.clone();

    for c in ['-', '|', 'L', 'J', 'F', '7'] {
        let mut grid = input_grid.clone();
        grid.set_start(c);

        let mut pos = grid.start;
        let mut previous_pos = grid.start;
        let mut length = 0;
        let mut seens = vec![];
        while pos != grid.start || length == 0 {
            seens.push(pos);
            let next_pos = grid.get_next_position(&pos, &previous_pos);
            if let Some(next_pos) = next_pos {
                previous_pos = pos;
                pos = next_pos;
                length += 1;
            } else {
                break;
            }
        }
        if pos == grid.start {
            length /= 2;
            println!("{}: {}", c, length);
            if length > max_length {
                max_length = length;
                max_seens = seens;
                max_grid = grid;
            }
        }
    }

    let zoom_width = 2 * max_grid.width - 1;
    let zoom_height = 2 * max_grid.height - 1;
    let mut walls = vec![vec![false; zoom_width]; zoom_height];
    let mut insides = vec![vec![false; zoom_width]; zoom_height];
    let mut outsides = vec![vec![false; zoom_width]; zoom_height];

    for pos in max_seens {
        let x = pos.x as usize;
        let y = pos.y as usize;
        walls[2 * y][2 * x] = true;
        #[allow(clippy::match_on_vec_items)]
        match max_grid.grid[y][x] {
            '-' => {
                walls[2 * y][2 * x + 1] = true;
                walls[2 * y][2 * x - 1] = true;
            }
            '|' => {
                walls[2 * y + 1][2 * x] = true;
                walls[2 * y - 1][2 * x] = true;
            }
            'J' => {
                walls[2 * y - 1][2 * x] = true;
                walls[2 * y][2 * x - 1] = true;
            }
            'F' => {
                walls[2 * y + 1][2 * x] = true;
                walls[2 * y][2 * x + 1] = true;
            }
            '7' => {
                walls[2 * y + 1][2 * x] = true;
                walls[2 * y][2 * x - 1] = true;
            }
            'L' => {
                walls[2 * y - 1][2 * x] = true;
                walls[2 * y][2 * x + 1] = true;
            }
            _ => {}
        }
    }

    let mut floodfill = walls.clone();

    let get_next_start = |ff: &Vec<Vec<bool>>| {
        #[allow(clippy::needless_range_loop)]
        for y in 0..zoom_height {
            for x in 0..zoom_width {
                if !ff[y][x] {
                    return Some(Pos {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
        None
    };

    while let Some(start) = get_next_start(&floodfill) {
        let mut found_exit = false;
        let mut queue = vec![start];
        let mut seens = vec![];
        while let Some(pos) = queue.pop() {
            if pos.x == 0
                || pos.x + 1 == zoom_width as i32
                || pos.y == 0
                || pos.y + 1 == zoom_height as i32
            {
                found_exit = true;
            }

            if floodfill[pos.y as usize][pos.x as usize] {
                continue;
            }

            seens.push(pos);
            floodfill[pos.y as usize][pos.x as usize] = true;

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let x = pos.x + dx;
                let y = pos.y + dy;
                if !(Pos { x, y }.is_in_bounds(zoom_width, zoom_height)) {
                    continue;
                }
                if floodfill[y as usize][x as usize] {
                    continue;
                }
                queue.push(Pos { x, y });
            }
        }

        for seen in seens {
            if !found_exit {
                insides[seen.y as usize][seen.x as usize] = true;
            } else {
                outsides[seen.y as usize][seen.x as usize] = true;
            }
        }
    }

    // for y in 0..zoom_height {
    //     for x in 0..zoom_width {
    //         if walls[y][x] {
    //             print!("#");
    //         } else if insides[y][x] {
    //             print!("I");
    //         } else if outsides[y][x] {
    //             print!("O");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    let mut count = 0;
    for x in 0..max_grid.width {
        for y in 0..max_grid.height {
            if insides[2 * y][2 * x] {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_1: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....";

    static EXAMPLE_2: &str = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    static EXAMPLE_3: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    static EXAMPLE_4: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    static EXAMPLE_5: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    static EXAMPLE_6: &str = "\
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    static EXAMPLE_7: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    static EXAMPLE_8: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_1), 4);
        assert_eq!(part1(EXAMPLE_2), 4);
        assert_eq!(part1(EXAMPLE_3), 8);
        assert_eq!(part1(EXAMPLE_4), 8);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_5), 4);
        assert_eq!(part2(EXAMPLE_6), 4);
        assert_eq!(part2(EXAMPLE_7), 8);
        assert_eq!(part2(EXAMPLE_8), 10);
    }

    static INPUT: &str = include_str!("../input/2023/day10.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 6931);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 357);
    }
}
