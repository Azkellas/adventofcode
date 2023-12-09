use itertools::Itertools;
use scan_fmt::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Part {
    Part1,
    Part2,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum NodeKind {
    Start,
    End,
    Normal,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Node {
    id: usize,
    kind: NodeKind,
    left: usize,
    right: usize,
}

fn input_generator(part: Part, input: &str) -> (String, Vec<Node>) {
    let mut lines = input.lines();
    let moves = lines.next().unwrap();

    let mut seens = BTreeMap::new();
    let mut nodes = vec![];

    lines.skip(1).for_each(|line| {
        let (from, left, right) = scan_fmt!(line, "{} = ({}, {})", String, String, String).unwrap();

        let mut kind = NodeKind::Normal;
        if part == Part::Part1 && from == "AAA" || part == Part::Part2 && from.ends_with('A') {
            kind = NodeKind::Start;
        }
        if part == Part::Part1 && from == "ZZZ" || part == Part::Part2 && from.ends_with('Z') {
            kind = NodeKind::End;
        }

        let from = seens.get(&from).copied().unwrap_or_else(|| {
            let id = seens.len();
            seens.insert(from.clone(), id);
            id
        });
        let left = seens.get(&left).copied().unwrap_or_else(|| {
            let id = seens.len();
            seens.insert(left.clone(), id);
            id
        });
        let right = seens.get(&right).copied().unwrap_or_else(|| {
            let id = seens.len();
            seens.insert(right.clone(), id);
            id
        });

        nodes.push(Node {
            id: from,
            kind,
            left,
            right,
        });
    });

    nodes.sort_by_key(|n| n.id);

    (moves.to_string(), nodes)
}

#[aoc(day8, part1)]
pub fn part1(calibration: &str) -> usize {
    let (moves, nodes) = input_generator(Part::Part1, calibration);

    let mut node = nodes.iter().find(|n| n.kind == NodeKind::Start).unwrap();
    let mut index = 0;
    let mut steps = 0;
    while node.kind != NodeKind::End {
        let char = moves.chars().nth(index).unwrap();
        match char {
            'L' => {
                node = &nodes[node.left];
            }
            'R' => {
                node = &nodes[node.right];
            }
            _ => unreachable!(),
        }
        steps += 1;
        index = (index + 1) % moves.len();
    }
    steps
}

#[aoc(day8, part2)]
pub fn part2(calibration: &str) -> usize {
    let (moves, nodes) = input_generator(Part::Part2, calibration);

    let start_nodes = nodes
        .iter()
        .filter(|n| n.kind == NodeKind::Start)
        .collect_vec();

    let mut seens = vec![];
    let mut periods = vec![];
    let mut starts = vec![];
    for start_node in start_nodes.iter() {
        let mut seen: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        let mut index = 0;
        let mut node = *start_node;
        let mut valid = true;
        loop {
            let char = moves.chars().nth(index % moves.len()).unwrap();
            match char {
                'L' => {
                    node = &nodes[node.left];
                }
                'R' => {
                    node = &nodes[node.right];
                }
                _ => unreachable!(),
            }
            index += 1;

            if node.kind == NodeKind::End {
                let val = (node.id, index % moves.len());
                if seen.contains_key(&val) {
                    break;
                }
                seen.insert(val, index);
            } else if index > nodes.len() * moves.len() {
                valid = false;
                break;
            }
        }

        if valid {
            let period = index - seen[&(node.id, index % moves.len())];
            periods.push(period);
            starts.push(seen[&(node.id, index % moves.len())]);
        } else {
            periods.push(usize::MAX);
            starts.push(usize::MAX);
        }
        seens.push(seen);
    }

    // for (idx, seen) in seens.iter_mut().enumerate() {
    //     println!(
    //         "for {}: seens = {:?}, periods = {}, starts = {}",
    //         idx,
    //         seen.keys(),
    //         periods[idx],
    //         starts[idx]
    //     );
    // }

    // we can do this because the orbits start at index 0 and are pairwise coprimes.
    periods.into_iter().fold(1, num::integer::lcm)

    // this slow solution solves it if it's not the case.
    // let mut current_steps = seens
    //     .iter()
    //     .map(|s| *s.iter().min_by_key(|(k, v)| **v).unwrap().1)
    //     .collect_vec();

    // while current_steps.iter().any(|&s| s != current_steps[0]) {
    //     let (idx, val) = current_steps
    //         .iter()
    //         .enumerate()
    //         .min_by_key(|(_, c)| **c)
    //         .unwrap();
    //     let in_warmup = seens[idx].iter().find(|(_, v)| **v > *val);
    //     if let Some((_, v)) = in_warmup {
    //         current_steps[idx] = *v;
    //     } else {
    //         current_steps[idx] += periods[idx];
    //     }
    // }
    // current_steps[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    static EXAMPLE_2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    static EXAMPLE_3: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_1), 2);
        assert_eq!(part1(EXAMPLE_2), 6);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_3), 6);
    }

    static INPUT: &str = include_str!("../input/2023/day8.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 11567);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 9858474970153);
    }
}
