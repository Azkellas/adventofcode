use std::str;
use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r"(\d+) +(\d+) +(\d+)").unwrap();

    let mut invalid = 0;
    let mut seen = 0;
    for cap in re.captures_iter(input) {
        // cap.iter().skip(1).map(|s| s.unwrap());
        let mut v = cap.iter().skip(1).map(|s| s.unwrap().as_str().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        v.sort();
        let [a, b, c] = v[0..3] else {
            unreachable!();
        };

        // eprintln!("{} {} {}", a,b,c);

        seen += 1;
        if c >= a + b {
            invalid += 1;
        }
    }

    eprintln!("seen {seen} lines");
    return seen - invalid;

}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let re = Regex::new(r"(\d+) +(\d+) +(\d+)").unwrap();

    let mut valid = 0;
    let mut data: Vec<Vec<i32>> = vec![];
    for cap in re.captures_iter(input) {
        // cap.iter().skip(1).map(|s| s.unwrap());
        let v = cap.iter().skip(1).map(|s| s.unwrap().as_str().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        data.push(v[..3].to_vec());
    }

    for i in 0..data.len()/3 {
        for c in 0..3 {
            let (a, b, c) = (data[3*i][c], data[3*i+1][c], data[3*i+2][c]);
            let mut vv = vec![a, b, c];
            vv.sort();
            if vv[2] < vv[1] + vv[0] {
                valid += 1;
            }
        }
    }
    valid

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sample1() {
        assert_eq!(part1("5 10 25"), 1);
    }


    #[test]
    fn sample2() {
        // assert_eq!(part2(EXAMPLE), "5DB3");
    }

}
