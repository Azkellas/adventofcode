use std::collections::HashMap;

use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Noop,
    GreaterThan,
    LessThan,
}

impl Operator {
    fn apply(&self, lhs: usize, rhs: usize) -> bool {
        match self {
            Operator::Noop => true,
            Operator::GreaterThan => lhs > rhs,
            Operator::LessThan => lhs < rhs,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Filter {
    index: usize,
    value: usize,
    operator: Operator,
    result: String,
}

impl Filter {
    fn apply(&self, part: &Part) -> bool {
        self.operator.apply(part[self.index], self.value)
    }
}

type Part = [usize; 4];

fn input_generator(input: &str) -> (HashMap<String, Vec<Filter>>, Vec<Part>) {
    let input = input.replace("\r\n", "\n");
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|line| {
            let (name, filters) = scan_fmt!(line, "{}{{{}}}", String, String).unwrap();

            let filters = filters
                .split(',')
                .map(|filter| {
                    if !filter.contains(':') {
                        return Filter {
                            index: 0,
                            value: 0,
                            operator: Operator::Noop,
                            result: filter.to_string(),
                        };
                    }

                    let index = match filter.chars().next().unwrap() {
                        'x' => 0,
                        'm' => 1,
                        'a' => 2,
                        's' => 3,
                        _ => unreachable!(),
                    };
                    let operator = match filter.chars().nth(1).unwrap() {
                        '>' => Operator::GreaterThan,
                        '<' => Operator::LessThan,
                        _ => unreachable!(),
                    };

                    let (value, result) = scan_fmt!(filter, "{}:{}", String, String).unwrap();
                    let value = value[2..].parse::<usize>().unwrap();

                    Filter {
                        index,
                        value,
                        operator,
                        result,
                    }
                })
                .collect_vec();
            (name, filters)
        })
        .collect_vec();

    let mut workflows_map = HashMap::new();
    for (name, filters) in workflows {
        workflows_map.insert(name, filters);
    }

    let parts = parts
        .lines()
        .map(|line| {
            let (x, m, a, s) = scan_fmt!(
                line,
                "{{x={d},m={d},a={d},s={d}}}",
                usize,
                usize,
                usize,
                usize
            )
            .unwrap();
            [x, m, a, s]
        })
        .collect_vec();

    (workflows_map, parts)
}

#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
    let (workflows, parts) = input_generator(input);

    parts
        .into_iter()
        .filter(|part| {
            // println!("{part:?}");
            let mut workflow_id = "in";
            loop {
                // println!("  {workflow_id}");
                if workflow_id == "R" {
                    return false;
                }
                if workflow_id == "A" {
                    return true;
                }

                let filters = workflows.get(workflow_id).unwrap();
                for filter in filters {
                    if filter.apply(part) {
                        // println!("    - {filter:?}");
                        workflow_id = &filter.result;
                        break;
                    }
                }
            }
        })
        .map(|part| part.into_iter().sum::<usize>())
        .sum()
}

#[aoc(day19, part2)]
fn part2(input: &str) -> usize {
    let (workflows, _) = input_generator(input);

    let mut nodes = vec![([1usize..=4000, 1..=4000, 1..=4000, 1..=4000], ("in", 0))];

    let score_ranges = |range: [RangeInclusive<usize>; 4]| {
        range
            .into_iter()
            .map(|r: RangeInclusive<usize>| r.count())
            .product::<usize>()
    };

    let mut score = 0;
    while let Some(node) = nodes.pop() {
        let (ranges, (wid, fidx)) = node;

        // println!("{ranges:?} {wid} {fidx}");

        if wid == "A" {
            score += score_ranges(ranges);
            continue;
        }
        if wid == "R" {
            continue;
        }

        let filter: &Filter = &workflows.get(wid).unwrap()[fidx];
        if filter.operator == Operator::Noop {
            nodes.push((ranges, (&filter.result, 0)));
            continue;
        }

        if !ranges[filter.index].contains(&filter.value) {
            // outside of range, push to next in workflow
            nodes.push((ranges, (wid, fidx + 1)));
        } else {
            // need to split in two
            let mut ranges1 = ranges.clone();
            let mut ranges2 = ranges.clone();
            if filter.operator == Operator::GreaterThan {
                ranges1[filter.index] = *ranges[filter.index].start()..=filter.value;
                ranges2[filter.index] = (filter.value + 1)..=*ranges[filter.index].end();
                nodes.push((ranges1, (wid, fidx + 1)));
                nodes.push((ranges2, (&filter.result, 0)));
            } else {
                ranges1[filter.index] = *ranges[filter.index].start()..=(filter.value - 1);
                ranges2[filter.index] = filter.value..=*ranges[filter.index].end();
                nodes.push((ranges1, (&filter.result, 0)));
                nodes.push((ranges2, (wid, fidx + 1)));
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 19114);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 167409079868000);
    }

    static INPUT: &str = include_str!("../input/2023/day19.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 386787);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 131029523269531);
    }
}
