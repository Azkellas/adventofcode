use itertools::{iproduct, Itertools};

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

    let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    iproduct!(0..grid.len(), 0..grid[0].len())
        .filter(|&(y, x)| {
            let coord_height = grid[y][x];
            deltas.iter().any(|&(dx, dy)| {
                (1..)
                    .into_iter()
                    .map_while(|i| {
                        let nx = x.checked_add_signed(dx * i)?;
                        let ny = y.checked_add_signed(dy * i)?;
                        grid.get(ny)?.get(nx)
                    })
                    .all(|&height| height < coord_height)
            })
        })
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

    let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    iproduct!(0..grid.len(), 0..grid[0].len())
        .map(|(y, x)| {
            let coord_height = grid[y][x];
            deltas
                .iter()
                .map(|&(dx, dy)| {
                    let line = (1..).into_iter().map_while(|i| {
                        let nx = x.checked_add_signed(dx * i)?;
                        let ny = y.checked_add_signed(dy * i)?;
                        grid.get(ny)?.get(nx)
                    });

                    let mut score = 0;
                    for height in line {
                        score += 1;
                        if *height >= coord_height {
                            break;
                        }
                    }
                    score
                })
                .product::<usize>()
        })
        .max()
        .unwrap()
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

    static INPUT: &str = include_str!("../input/2022/day8.txt");
    #[test]
    fn sample3() {
        assert_eq!(part1(INPUT), 1801);
    }

    #[test]
    fn sample4() {
        assert_eq!(part2(INPUT), 209880);
    }
}
