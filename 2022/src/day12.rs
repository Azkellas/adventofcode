use std::collections::VecDeque;

use itertools::iproduct;

pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

fn is_good_move(
    grid: &Vec<Vec<char>>,
    dists: &Vec<Vec<usize>>,
    (x, y): (usize, usize),
    (nx, ny): (usize, usize),
) -> bool {
    let (width, height) = (grid[0].len(), grid.len());
    let is_in_bound = nx < width && ny < height;
    let is_new = is_in_bound && dists[y][x] + 1 < dists[ny][nx];
    let is_accessible = is_in_bound && grid[ny][nx] as u8 <= grid[y][x] as u8 + 1;
    is_in_bound && is_new && is_accessible
}

pub fn find_steps(grid: &Vec<Vec<char>>, starts: &[(usize, usize)], end: (usize, usize)) -> usize {
    let (width, height) = (grid[0].len(), grid.len());
    let mut dists: Vec<Vec<usize>> = vec![vec![usize::MAX - 10; width]; height];

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    for start in starts {
        queue.push_back(*start);
        dists[start.1][start.0] = 0;
    }

    let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while let Some((x, y)) = queue.pop_front() {
        deltas
            .iter()
            .filter_map(|&(dx, dy)| {
                let nx = x.checked_add_signed(dx)?;
                let ny = y.checked_add_signed(dy)?;
                Some((nx, ny))
            })
            .for_each(|(nx, ny)| {
                if is_good_move(grid, &dists, (x, y), (nx, ny)) {
                    dists[ny][nx] = dists[y][x] + 1;
                    queue.push_back((nx, ny));
                }
            });
    }

    dists[end.1][end.0]
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let mut grid = input_generator(input);
    let mut target = (usize::MAX, usize::MAX);
    let mut start = (usize::MAX, usize::MAX);
    let (width, height) = (grid[0].len(), grid.len());

    iproduct!(0..width, 0..height).for_each(|(x, y)| match grid[y][x] {
        'S' => {
            grid[y][x] = 'a';
            start = (x, y);
        }
        'E' => {
            target = (x, y);
            grid[y][x] = 'z';
        }
        _ => (),
    });

    find_steps(&grid, &[start], target)
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    let mut grid = input_generator(input);
    let (width, height) = (grid[0].len(), grid.len());
    let mut target = (usize::MAX, usize::MAX);
    let mut starts = vec![];

    iproduct!(0..width, 0..height).for_each(|(x, y)| match grid[y][x] {
        'S' | 'a' => {
            grid[y][x] = 'a';
            starts.push((x, y));
        }
        'E' => {
            target = (x, y);
            grid[y][x] = 'z';
        }
        _ => (),
    });

    find_steps(&grid, &starts, target)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn sample1() {
        assert_eq!(part1(EXAMPLE), 31);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(EXAMPLE), 29);
    }

    static INPUT: &str = include_str!("../input/2022/day12.txt");
    #[test]
    fn sample3() {
        assert_eq!(part1(INPUT), 408);
    }

    #[test]
    fn sample4() {
        assert_eq!(part2(INPUT), 399);
    }
}
