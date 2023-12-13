use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Map {
    grid: Vec<Vec<char>>,
    transposed: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Part {
    Part1 = 0,
    Part2 = 1,
}

pub fn input_generator(input: &str) -> Vec<Map> {
    let grids = input.replace("\r\n", "\n");

    grids
        .split("\n\n")
        .map(|map| {
            let grid = map.lines().map(|l| l.chars().collect_vec()).collect_vec();
            let width = grid[0].len();
            let height = grid.len();
            let transposed = (0..grid[0].len())
                .map(|x| (0..grid.len()).map(|y| grid[y][x]).collect_vec())
                .collect_vec();
            Map {
                grid,
                transposed,
                width,
                height,
            }
        })
        .collect_vec()
}

fn test(grid: &Vec<Vec<char>>, index: usize) -> usize {
    let size = grid.len();
    let mut differences = 0;
    for i in 0..=index {
        if index + i + 1 < size {
            differences += grid[index - i]
                .iter()
                .zip(grid[index + i + 1].iter())
                .filter(|(a, b)| a != b)
                .count();
        }
    }

    differences
}

fn solve(input: &str, part: Part) -> usize {
    let grids = input_generator(input);

    grids
        .into_iter()
        .map(|map| {
            // horizontal
            for idx in 0..map.height - 1 {
                let differences = test(&map.grid, idx);
                if differences == (part as usize) {
                    return 100 * (idx + 1);
                }
            }

            // vertical
            for idx in 0..map.width - 1 {
                let differences = test(&map.transposed, idx);
                if differences == (part as usize) {
                    return idx + 1;
                }
            }

            unreachable!()
        })
        .sum()
}
#[aoc(day13, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, Part::Part1)
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> usize {
    solve(input, Part::Part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 405);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 400);
    }

    static INPUT: &str = include_str!("../input/2023/day13.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 29130);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 33438);
    }
}
