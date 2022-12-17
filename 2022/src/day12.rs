use std::collections::VecDeque;

pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            line.chars().collect()
        })
        .collect()
}

pub fn find_steps(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> usize {
    let (width, height) = (grid[0].len(), grid.len());
    let mut dists: Vec<Vec<usize>> = vec![vec![usize::MAX - 10; width]; height];

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    queue.push_back(start);
    dists[start.1][start.0] = 0;

    while let Some((x, y)) = queue.pop_front() {
        if x > 0 && dists[y][x] + 1 < dists[y][x-1] && grid[y][x-1] as i32 - grid[y][x] as i32 <= 1 {
            dists[y][x-1] = dists[y][x] + 1;
            queue.push_back((x-1, y));
        }
        if x < width - 1 && dists[y][x] + 1 < dists[y][x+1] && grid[y][x+1] as i32 - grid[y][x] as i32 <= 1 {
            dists[y][x+1] = dists[y][x] + 1;
            queue.push_back((x+1, y));
        }
        if y > 0 && dists[y][x] + 1 < dists[y-1][x] && grid[y-1][x] as i32 - grid[y][x] as i32 <= 1 {
            dists[y-1][x] = dists[y][x] + 1;
            queue.push_back((x, y-1));
        }
        if y < height - 1 && dists[y][x] + 1 < dists[y+1][x] && grid[y+1][x] as i32 - grid[y][x] as i32 <= 1 {
            dists[y+1][x] = dists[y][x] + 1;
            queue.push_back((x, y+1));
        }
    }

    dists[end.1][end.0]
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let mut grid = input_generator(input);
    let mut target = (usize::MAX, usize::MAX);
    let mut start = (usize::MAX, usize::MAX);
    let (width, height) = (grid[0].len(), grid.len());

    for x in 0..width {
        for y in 0..height {
            if grid[y][x] == 'S' {
                start = (x, y);
                grid[y][x] = 'a';
            }
            if grid[y][x] == 'E' {
                target = (x, y);
                grid[y][x] = 'z';
            }
        }
    }

    find_steps(&grid, start, target)
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    let mut grid = input_generator(input);
    let (width, height) = (grid[0].len(), grid.len());
    let mut target = (usize::MAX, usize::MAX);

    for x in 0..width {
        for y in 0..height {
            if grid[y][x] == 'S' {
                grid[y][x] = 'a';
            }
            if grid[y][x] == 'E' {
                target = (x, y);
                grid[y][x] = 'z';
            }
        }
    }

    let mut min = usize::MAX;
    for x in 0..width {
        for y in 0..height {
            if grid[y][x] == 'a' {
                min = usize::min(min, find_steps(&grid, (x, y), target));
            }
        }
    }
    min
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
}
