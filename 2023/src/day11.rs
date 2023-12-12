use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

pub fn input_generator(input: &str) -> (Vec<Pos>, Vec<i64>, Vec<i64>) {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let width = grid[0].len();
    let height = grid.len();

    let cols = (0..width)
        .filter(|x| grid.iter().all(|l| l[*x] == '.'))
        .map(|x| x as i64)
        .collect_vec();

    let rows = (0..height)
        .filter(|y| grid[*y].iter().all(|c| *c == '.'))
        .map(|y| y as i64)
        .collect_vec();

    let galaxies = grid
        .into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(|(x, c)| {
                    if c == '.' {
                        None
                    } else {
                        Some(Pos {
                            x: x as i64,
                            y: y as i64,
                        })
                    }
                })
                .collect_vec()
        })
        .flatten()
        .collect_vec();

    (galaxies, rows, cols)
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> i64 {
    solve(input, 2)
}

pub fn solve(input: &str, expansion_ratio: i64) -> i64 {
    let (galaxies, rows, cols) = input_generator(input);

    galaxies
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let (min_x, max_x) = (i64::min(a.x, b.x), i64::max(a.x, b.x));
            let (min_y, max_y) = (i64::min(a.y, b.y), i64::max(a.y, b.y));
            let dx = max_x - min_x
                + (expansion_ratio - 1)
                    * cols.iter().filter(|c| **c >= min_x && **c < max_x).count() as i64;
            let dy = max_y - min_y
                + (expansion_ratio - 1)
                    * rows.iter().filter(|c| **c >= min_y && **c < max_y).count() as i64;

            dx + dy
        })
        .sum()
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> i64 {
    solve(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 374);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(EXAMPLE, 10), 1030);
        assert_eq!(solve(EXAMPLE, 100), 8410);
    }

    static INPUT: &str = include_str!("../input/2023/day11.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 9233514);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 363293506944);
    }
}
