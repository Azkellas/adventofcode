use std::cmp::Ordering;
use std::str::FromStr;
use std::num::ParseIntError;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum List {
    Int(usize),
    List(Vec<List>),
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Int(lhs), Self::Int(rhs)) => lhs.cmp(rhs),
            (Self::Int(lhs), rhs) => {
                let lhs = Self::List(vec![Self::Int(*lhs)]);
                lhs.cmp(rhs)
            }
            (lhs, Self::Int(rhs)) => {
                let rhs = Self::List(vec![Self::Int(*rhs)]);
                lhs.cmp(&rhs)
            },
            (Self::List(lhs), Self::List(rhs)) => {
                let (lhs_len, rhs_len) = (lhs.len(), rhs.len());
                let min = usize::min(lhs_len, rhs_len);
                for i in 0..min {
                    match lhs[i].cmp(&rhs[i]) {
                        Ordering::Equal => continue,
                        ord => return ord,
                    }
                }
                lhs_len.cmp(&rhs_len)
            },
        }
    }
}
impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for List { }

impl FromStr for List {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input == "[]" {
            return Ok(Self::List(vec![]));
        }

        let mut depth = 0;
        let bytes = input.as_bytes();
        let mut split_idx = vec![];
        for (i, byte) in bytes.iter().enumerate() {
            match byte {
                b'[' => depth += 1,
                b']' => depth -= 1,
                b',' if depth == 1 => split_idx.push(i),
                _ => ()
            }
        }

        if split_idx.is_empty() {
            return if bytes[0] == b'[' {
                // contains sublist
                let rec = Self::from_str(&input[1..input.len()-1])?;
                Ok(Self::List(vec![rec]))
            }
            else {
                // mere int
                input.parse::<usize>().map(Self::Int)
            }
        }

        // lists of lists
        split_idx.insert(0, 0);
        split_idx.push(bytes.len() - 1);

        let mut lists = vec![];
        for i in 0..split_idx.len() - 1 {
            let (start, end) = (split_idx[i] + 1, split_idx[i+1]);
            let rec = Self::from_str(&input[start..end])?;
            lists.push(rec);
        }
        Ok(Self::List(lists))
    }
}

pub fn input_generator(input: &str) -> Result<Vec<List>, ParseIntError> {
    input
        .lines()
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(List::from_str)
        .collect::<Result<Vec<_>, _>>()
}


#[must_use]
#[aoc(day13, part1)]
pub fn part1(input: &str) -> usize {
    input_generator(input)
        .unwrap()
        .iter()
        .tuples()
        .enumerate()
        .filter(|(_, (line1, line2))| line1 < line2)
        .map(|(idx, _)| idx + 1)
        .sum()
}

#[must_use]
#[aoc(day13, part2)]
pub fn part2(input: &str) -> usize {
    let mut lists = input_generator(input).unwrap();

    let key1 = List::from_str("[[2]]").unwrap();
    let key2 = List::from_str("[[6]]").unwrap();
    lists.push(key1.clone());
    lists.push(key2.clone());
    lists.sort();

    lists.iter().positions(|l| *l == key1 || *l == key2).map(|idx| idx + 1).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn sample1() {
        assert_eq!(part1(EXAMPLE), 13);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(EXAMPLE), 140);
    }
}
