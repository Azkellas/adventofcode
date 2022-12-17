use itertools::Itertools;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() + 1)
                .collect_vec()
        })
        .collect_vec();
    let mut bools = vec![vec![false; grid[0].len()]; grid.len()];

    for y in 0..grid.len() {
        let mut max = 0;
        for x in 0..grid[0].len() {
            if grid[y][x] > max {
                bools[y][x] = true;
                max = grid[y][x];
            }
        }
        let mut max = 0;
        for x in (0..grid[0].len()).rev() {
            if grid[y][x] > max {
                bools[y][x] = true;
                max = grid[y][x];
            }
        }
    }

    for x in 0..grid[0].len() {
        let mut max = 0;
        for y in 0..grid.len() {
            if grid[y][x] > max {
                bools[y][x] = true;
                max = grid[y][x];
            }
        }
        let mut max = 0;
        for y in (0..grid.len()).rev() {
            if grid[y][x] > max {
                bools[y][x] = true;
                max = grid[y][x];
            }
        }
    }
    bools
        .into_iter()
        .flat_map(|v| v.into_iter())
        .filter(|b| *b)
        .count()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() + 1)
                .collect_vec()
        })
        .collect_vec();
    // let mut scores = vec![vec![0; grid[0].len()]; grid.len()];
    let mut max_score = 0;

    for x in 0..grid[0].len() {
        // eprintln!("{} < {}", x, grid[0].len());
        for y in 0..grid.len() {
            let mut scores = (0, 0, 0, 0);
            let tree_height = grid[y][x];
            for dx in x + 1..grid[y].len() {
                if grid[y][dx] >= tree_height {
                    scores.0 = dx - x;
                    break;
                }
            }
            if scores.0 == 0 {
                scores.0 = grid[y].len() - 1 - x;
            }

            for dx in (0..x).rev() {
                if grid[y][dx] >= tree_height {
                    scores.1 = x - dx;
                    break;
                }
            }
            if scores.1 == 0 {
                scores.1 = x;
            }

            for dy in y + 1..grid.len() {
                if grid[dy][x] >= tree_height {
                    scores.2 = dy - y;
                    break;
                }
            }
            if scores.2 == 0 {
                scores.2 = grid.len() - 1 - y;
            }

            for dy in (0..y).rev() {
                if grid[dy][x] >= tree_height {
                    scores.3 = y - dy;
                    break;
                }
            }
            if scores.3 == 0 {
                scores.3 = y;
            }

            let score = scores.0 * scores.1 * scores.2 * scores.3;
            // eprintln!("{x} {y} {}: {scores:?}", grid[y][x]);
            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn sample1() {
        assert_eq!(part1(EXAMPLE), 21);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(EXAMPLE), 8);
    }
}
