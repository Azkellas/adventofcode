use itertools::Itertools;
use scan_fmt::*;

pub struct Move(usize, usize, usize);
pub fn input_generator(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let mut preset: Vec<&str> = vec![];
    let mut moves: Vec<&str> = vec![];
    let mut found_blank = false;
    for line in input.lines() {
        if line.trim() == "" {
            found_blank = true;
        } else {
            match found_blank {
                true => moves.push(line),
                false => preset.push(line),
            }
        }
    }

    // parse preset
    let mut columns = vec![];
    columns.push(vec![]); // index 0
    preset.reverse();
    let stack_count = preset[0]
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .max()
        .unwrap();

    for idx in 1..=stack_count {
        let mut column = vec![];
        let char_idx = preset[0].find(idx.to_string().as_str()).unwrap();
        for line in preset.iter().skip(1) {
            let char = line.chars().nth(char_idx).unwrap();
            match char {
                ' ' => break,
                _ => column.push(char),
            }
        }
        columns.push(column);
    }

    // parse moves
    let moves = moves
        .iter()
        .map(|line| {
            scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize)
                .map(|(a, b, c)| Move(a, b, c))
                .unwrap()
        })
        .collect_vec();
    (columns, moves)
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> String {
    let (mut columns, moves) = input_generator(input);

    for Move(count, from, to) in moves {
        let start_idx = columns[from].len() - count;
        let temp: Vec<char> = columns[from].drain(start_idx..).collect();
        columns[to].extend(temp.iter().rev());
    }

    columns
        .iter()
        .skip(1)
        .map(|col| col.last().unwrap())
        .collect()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> String {
    let (mut columns, moves) = input_generator(input);

    for Move(count, from, to) in moves {
        let start_idx = columns[from].len() - count;
        let temp: Vec<char> = columns[from].drain(start_idx..).collect();
        columns[to].extend(temp);
    }

    columns
        .iter()
        .skip(1)
        .map(|col| col.last().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    #[test]
    fn sample1() {
        assert_eq!(part1(EXAMPLE), "CMZ");
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(EXAMPLE), "MCD");
    }

    static INPUT: &str = include_str!("../input/2022/day5.txt");
    #[test]
    fn sample3() {
        assert_eq!(part1(INPUT), "MQSHJMWNH");
    }

    #[test]
    fn sample4() {
        assert_eq!(part2(INPUT), "LLWJRBHVZ");
    }
}
