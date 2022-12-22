use scan_fmt::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Number(i64),
    Add(String, String),
    Div(String, String),
    Mul(String, String),
    Sub(String, String),
}

impl Operation {
    fn is_number(&self) -> bool {
        match self {
            Self::Number(_) => true,
            _ => false,
        }
    }

    fn get_ops(&self) -> (&String, &String) {
        match self {
            Self::Add(a, b) => (a, b),
            Self::Sub(a, b) => (a, b),
            Self::Div(a, b) => (a, b),
            Self::Mul(a, b) => (a, b),
            _ => unreachable!(),
        }
    }

    fn compute(&self, a: i64, b: i64) -> Self {
        match self {
            Self::Add(_, _) => Self::Number(a + b),
            Self::Sub(_, _) => Self::Number(a - b),
            Self::Div(_, _) => Self::Number(a / b),
            Self::Mul(_, _) => Self::Number(a * b),
            _ => unreachable!(),
        }
    }
}
fn input_generator(input: &str) -> HashMap<String, Operation> {
    let mut monkeys = HashMap::new();
    for line in input.lines() {
        let num_parse = scan_fmt!(line, "{}: {d}", String, i64);
        if let Ok((id, val)) = num_parse {
            monkeys.insert(id, Operation::Number(val));
        } else {
            let op_parse = scan_fmt!(line, "{}: {} {} {}", String, String, String, String);
            let (id, lhs, op, rhs) = op_parse.unwrap();
            match op.as_str() {
                "+" => {
                    monkeys.insert(id, Operation::Add(lhs, rhs));
                }
                "-" => {
                    monkeys.insert(id, Operation::Sub(lhs, rhs));
                }
                "*" => {
                    monkeys.insert(id, Operation::Mul(lhs, rhs));
                }
                "/" => {
                    monkeys.insert(id, Operation::Div(lhs, rhs));
                }
                _ => unreachable!(),
            }
        }
    }

    monkeys
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> i64 {
    let mut monkeys = input_generator(input);

    loop {
        if let Operation::Number(res) = monkeys.get("root").unwrap() {
            return *res;
        }

        let mut newmonkeys = HashMap::new();
        for (id, operation) in &monkeys {
            if let Operation::Number(_) = operation {
                continue;
            }

            let (lhs, rhs) = operation.get_ops();
            let lhs = monkeys.get(lhs).unwrap();
            let rhs = monkeys.get(rhs).unwrap();
            match (lhs, rhs) {
                (Operation::Number(a), Operation::Number(b)) => {
                    newmonkeys.insert(id.clone(), operation.compute(*a, *b));
                }
                _ => (),
            }
        }
        for (id, op) in newmonkeys {
            monkeys.insert(id, op);
        }
    }
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> i64 {
    let mut monkeys = input_generator(input);

    let mut deps = vec![];
    let mut node = "humn";
    while node != "root" {
        deps.push(node.to_owned());
        for (id, op) in &monkeys {
            if !op.is_number() {
                let (lhs, rhs) = op.get_ops();
                if lhs.as_str() == node || rhs.as_str() == node {
                    node = id.as_str();
                    break;
                }
            }
        }
    }

    let backup = monkeys.clone();
    let mut target;

    loop {
        let operation = monkeys.get("root").unwrap();
        let (lhs, rhs) = operation.get_ops();
        let oplhs = monkeys.get(lhs).unwrap();
        let oprhs = monkeys.get(rhs).unwrap();
        if oplhs.is_number() && oprhs.is_number() {
            if lhs != deps.last().unwrap().as_str() {
                if let Operation::Number(t) = oplhs {
                    target = *t;
                    break;
                }
            }
            if rhs != deps.last().unwrap().as_str() {
                if let Operation::Number(t) = oprhs {
                    target = *t;
                    break;
                }
            }
        }

        let mut newmonkeys = HashMap::new();
        for (id, operation) in &monkeys {
            if let Operation::Number(_) = operation {
                continue;
            }

            let (lhs, rhs) = operation.get_ops();
            let lhs = monkeys.get(lhs).unwrap();
            let rhs = monkeys.get(rhs).unwrap();
            match (lhs, rhs) {
                (Operation::Number(a), Operation::Number(b)) => {
                    newmonkeys.insert(id.clone(), operation.compute(*a, *b));
                }
                _ => (),
            }
        }
        for (id, op) in newmonkeys {
            monkeys.insert(id, op);
        }
    }

    // eprintln!("target = {target}");

    for (idx, node) in deps.iter().enumerate().rev() {
        if node.as_str() == "humn" {
            return target;
        }
        let op = backup.get(node).unwrap();
        let (lhs, rhs) = op.get_ops();
        let Operation::Number(reslhs) = monkeys.get(lhs).unwrap() else { unreachable!() };
        let Operation::Number(resrhs) = monkeys.get(rhs).unwrap() else { unreachable!() };
        if lhs.as_str() == deps[idx - 1] {
            match op {
                Operation::Add(_, _) => target -= resrhs,
                Operation::Sub(_, _) => target += resrhs,
                Operation::Mul(_, _) => target /= resrhs,
                Operation::Div(_, _) => target *= resrhs,
                _ => unreachable!(),
            }
        }
        if rhs.as_str() == deps[idx - 1] {
            match op {
                Operation::Add(_, _) => target -= reslhs,
                Operation::Sub(_, _) => target = reslhs - target,
                Operation::Mul(_, _) => target /= reslhs,
                Operation::Div(_, _) => target = reslhs / target,
                _ => unreachable!(),
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn sample1() {
        assert_eq!(part1(EXAMPLE), 152);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(EXAMPLE), 301);
    }

    static INPUT: &str = include_str!("../input/2022/day21.txt");
    #[test]
    fn sample3() {
        assert_eq!(part1(INPUT), 70674280581468);
    }

    #[test]
    fn sample4() {
        assert_eq!(part2(INPUT), 3243420789721);
    }
}
