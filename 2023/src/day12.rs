use itertools::Itertools;
use memoize::memoize;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Line {
    data: Vec<char>,
    groups: Vec<usize>,
}

pub fn input_generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            let (data, groups) = l.split_once(' ').unwrap();
            let groups = groups.split(',').map(|s| s.parse().unwrap()).collect_vec();
            Line {
                data: data.chars().collect_vec(),
                groups,
            }
        })
        .collect_vec()
}

#[memoize]
pub fn validate(line: Line) -> usize {
    // no more groups to match
    if line.groups.is_empty() {
        if line.data.iter().any(|c| *c == '#') {
            return 0;
        } else {
            return 1;
        }
    }
    // find biggest group
    let (idx, size) = line
        .groups
        .iter()
        .enumerate()
        .max_by_key(|(_, &size)| size)
        .unwrap();

    let (left, right) = line.groups.split_at(idx);
    let (left, mut right) = (left.to_vec(), right.to_vec());
    right.remove(0);

    let min_idx = left.iter().sum::<usize>() + left.len();
    let max_idx = line.data.len() - right.iter().sum::<usize>() - right.len() - size;

    let mut total = 0;
    for idx in min_idx..=max_idx {
        if !line.data[idx..(idx + size)].iter().any(|c| *c == '.') {
            let mut line_left = line.clone();
            line_left.data = line.data[..idx].to_vec();
            line_left.groups = left.clone();
            if !line_left.data.is_empty() {
                match line_left.data.last().unwrap() {
                    '#' => continue,
                    '.' => {}
                    '?' => *line_left.data.last_mut().unwrap() = '.',
                    _ => unreachable!(),
                }
            }
            let mut line_right = line.clone();
            line_right.data = line.data[(idx + size)..].to_vec();
            line_right.groups = right.clone();
            if !line_right.data.is_empty() {
                match line_right.data.first().unwrap() {
                    '#' => continue,
                    '.' => {}
                    '?' => line_right.data[0] = '.',
                    _ => unreachable!(),
                }
            }
            total += validate(line_left) * validate(line_right);
        }
    }

    total
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    input_generator(input).into_iter().map(validate).sum()
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    input_generator(input)
        .into_par_iter()
        .map(|mut l| {
            let data = l.data.clone();
            l.data.push('?');
            l.data.append(&mut data.clone());
            l.data.push('?');
            l.data.append(&mut data.clone());
            l.data.push('?');
            l.data.append(&mut data.clone());
            l.data.push('?');
            l.data.append(&mut data.clone());

            let groups = l.groups.clone();
            l.groups.append(&mut groups.clone());
            l.groups.append(&mut groups.clone());
            l.groups.append(&mut groups.clone());
            l.groups.append(&mut groups.clone());

            validate(l)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 21);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 525152);
    }

    static INPUT: &str = include_str!("../input/2023/day12.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 7490);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 65607131946466);
    }
}
