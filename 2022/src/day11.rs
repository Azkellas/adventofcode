use itertools::Itertools;
use regex::Regex;
use scan_fmt::*;

#[derive(Debug)]
enum Operation {
    Add(usize),
    Mul(usize),
    Square,
}

#[derive(Debug)]
pub struct Monkey {
    _id: usize,
    operation: Operation,
    holding: Vec<usize>,
    test: usize,
    target: [usize; 2],
    inspected: usize,
}

pub fn input_generator(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| {
            let lines = monkey.split('\n').collect_vec();
            let [id, starting, operation, test, iftrue, iffalse] = lines[0..6] else {
                unreachable!()
            };
            let id = scan_fmt!(id, "Monkey {d}:", usize).unwrap();
            let (operation, val) =
                scan_fmt!(operation, "Operation: new = old {} {}:", char, String).unwrap();
            let operation = match (operation, val.as_str()) {
                ('+', _) => Operation::Add(val.parse::<usize>().unwrap()),
                ('*', "old") => Operation::Square,
                ('*', _) => Operation::Mul(val.parse::<usize>().unwrap()),
                _ => unreachable!(),
            };

            let test = scan_fmt!(test, "Test: divisible by {d}:", usize).unwrap();
            let iftrue = scan_fmt!(iftrue, "If true: throw to monkey {d}", usize).unwrap();
            let iffalse = scan_fmt!(iffalse, "If false: throw to monkey {d}", usize).unwrap();

            let holding = Regex::new(r"\d+")
                .unwrap()
                .find_iter(starting)
                .filter_map(|digits| digits.as_str().parse().ok())
                .collect_vec();

            Monkey {
                _id: id,
                operation,
                holding,
                test,
                target: [iffalse, iftrue],
                inspected: 0,
            }
        })
        .collect()
}

fn _debug_monkeys(round: usize, monkeys: &[Monkey]) {
    eprintln!("Round {round}:");
    for monkey in monkeys {
        eprintln!("Monkey {}: {}", monkey._id, monkey.holding.iter().join(", "));
    }
    eprintln!();
}
#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let mut monkeys = input_generator(input);

    // debug_monkeys(0, &monkeys);

    for _round in 1..=20 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].holding.clone();
            monkeys[i].holding = vec![];
            monkeys[i].inspected += items.len();

            for mut item in items {
                item = match monkeys[i].operation {
                    Operation::Mul(val) => item * val,
                    Operation::Add(val) => item + val,
                    Operation::Square => item * item,
                };
                item /= 3;
                let test = item % monkeys[i].test == 0;
                let targets = monkeys[i].target;
                monkeys[targets[test as usize]].holding.push(item);
            }
        }
        // debug_monkeys(round, &monkeys);
    }

    monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let mut monkeys = input_generator(input);
    let gcd: usize = monkeys.iter().map(|m| m.test).product();

    // debug_monkeys(0, &monkeys);

    for _round in 1..=10000 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].holding.clone();
            monkeys[i].holding = vec![];
            monkeys[i].inspected += items.len();

            for mut item in items {
                item = match monkeys[i].operation {
                    Operation::Mul(val) => item * val,
                    Operation::Add(val) => item + val,
                    Operation::Square => item * item,
                };
                // item /= 3;
                item %= gcd;

                let test = item % monkeys[i].test == 0;
                let targets = monkeys[i].target;
                monkeys[targets[test as usize]].holding.push(item);
            }
        }
        // if round % 100 == 0 {
        //     debug_monkeys(round, &monkeys);
        // }
    }

    monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 10605);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 2713310158);
    }

    static INPUT: &str = include_str!("../input/2022/day11.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 121450);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 28244037010);
    }
}
