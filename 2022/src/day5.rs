use itertools::Itertools;
use scan_fmt::*;

pub fn input_generator(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut preset: Vec<&str> = vec![];
    let mut moves: Vec<&str> = vec![];
    let mut found_blank = false;
    for line in input.lines() {
        if line.trim() == "" {
            found_blank = true;
        }
        else {
            match found_blank {
                true => moves.push(line),
                false => preset.push(line)
            }
        }
    }

    // parse preset
    let mut columns = vec![];
    columns.push(vec![]); // index 0
    preset.reverse();
    let stack_count = preset[0].split(" ").filter_map(|s| s.parse::<i32>().ok()).max().unwrap();

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
    let moves = moves.iter().map(|line| scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize).unwrap()).collect_vec();
    (columns, moves)
}

#[aoc(day5, part1)]
pub fn part1<'a>(input: &str) -> String {
    let (mut columns, moves) = input_generator(input);

    for (count, from, to) in moves {
        for _ in 0..count {
            let c = columns[from].pop().unwrap();
            columns[to].push(c);
        }
    }

    let mut res = "".to_owned();
    for col in columns.iter_mut().skip(1) {
        res.push(col.pop().unwrap());
    }
    res
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> String {
    let (mut columns, moves) = input_generator(input);

    for (count, from, to) in moves {
        let mut temp = vec![];
        for _ in 0..count {
            let c = columns[from].pop().unwrap();
            temp.push(c);
        }
        for _ in 0..count {
            columns[to].push(temp.pop().unwrap());
        }

        // eprintln!("{:?}", &columns);
    }

    let mut res = "".to_owned();
    for col in columns.iter_mut().skip(1) {
        res.push(col.pop().unwrap());
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str =
"    [D]    
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
}
