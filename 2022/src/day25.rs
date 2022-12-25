
#[aoc(day25, part1)]
pub fn part1(input: &str) -> String {
    let mut total: i64 = input.lines().map(|line| {
        line.chars().fold(0, |acc, c| {
            acc * 5 + match c {
                '-' => -1,
                '=' => -2,
                _ => c.to_digit(10).unwrap() as i64
            }
        })
    }).sum();

    let mut res = "".to_owned();
    while total > 0 {
        let rem = total % 5;
        let (r, c) = match rem {
            0 | 1 | 2 => (0, char::from_digit(rem as u32 , 10).unwrap()),
            3 => (1, '='),
            4 => (1, '-'),
            _ => unreachable!()
        };
        total /= 5;
        total += r;
        res.insert(0, c);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), "2=-1=0");
    }

    static INPUT: &str = include_str!("../input/2022/day25.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), "2=2-1-010==-0-1-=--2");
    }
}
