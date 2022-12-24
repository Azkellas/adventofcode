use itertools::Itertools;
use regex::Regex;
use std::collections::BTreeMap;

#[derive(Debug)]
struct Blueprint {
    id: usize,
    costs: [[usize; 4]; 4],
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
struct Data {
    robots: [usize; 4],
    resources: [usize; 4],
}

fn backtrace(
    blueprint: &Blueprint,
    turns_left: usize,
    data: &mut Data,
    cache: &mut BTreeMap<[usize; 4], Vec<(usize, [usize; 4])>>,
    seen: &mut usize,
    max_at: &mut Vec<usize>,
    late_game: usize,
) -> usize {
    *seen += 1;

    {
        let vec = cache.entry(data.robots).or_insert_with(|| vec![]);

        for (turns, resources) in vec {
            if *turns >= turns_left && (0..4).all(|i| resources[i] >= data.resources[i]) {
                return 0;
            };
        }
    }

    let mut res = data.resources[3] + data.robots[3] * turns_left;

    if turns_left == 0 || max_at[turns_left] > data.resources[3] + 2 {
        return res;
    }
    max_at[turns_left] = max_at[turns_left].max(data.resources[3]);

    for robot_id in (0..4).rev() {
        let cost = blueprint.costs[robot_id];
        let mut availables = [turns_left, turns_left, turns_left, turns_left];
        for i in 0..4 {
            if data.robots[i] >= 20 {
                continue;
            } else if data.resources[i] >= cost[i] {
                availables[i] = 1;
            } else if data.robots[i] > 0 {
                let r = data.robots[i];
                let c = cost[i] - data.resources[i];
                availables[i] = 1 + c / r + ((c % r != 0) as usize);
            }
        }
        let required = *availables.iter().max().unwrap();
        if required < turns_left && !(robot_id <= 1 && turns_left < late_game) {
            for i in 0..4 {
                data.resources[i] += required * data.robots[i];
                data.resources[i] -= cost[i];
            }
            data.robots[robot_id] += 1;
            res = res.max(backtrace(
                blueprint,
                turns_left - required,
                data,
                cache,
                seen,
                max_at,
                late_game,
            ));
            data.robots[robot_id] -= 1;
            for i in 0..4 {
                data.resources[i] += cost[i];
                data.resources[i] -= required * data.robots[i];
                // data.resources[i] -= required * data.robots[i] - cost[i];
            }
        }
    }

    {
        let vec = cache.entry(data.robots).or_insert_with(|| vec![]);
        vec.push((turns_left, data.resources));
    }

    res
}

fn input_generator(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| {
            let re = Regex::new(r"(\d+)").unwrap();
            let numbs = re
                .captures_iter(line)
                .map(|cap| cap.get(0).unwrap().as_str().parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let id = numbs[0];
            let ore_cost = numbs[1];
            let clay_cost = numbs[2];
            let (obs_cost_ore, obs_cost_clay) = (numbs[3], numbs[4]);
            let (geode_cost_ore, geode_cost_obs) = (numbs[5], numbs[6]);
            Blueprint {
                id,
                costs: [
                    [ore_cost, 0, 0, 0],
                    [clay_cost, 0, 0, 0],
                    [obs_cost_ore, obs_cost_clay, 0, 0],
                    [geode_cost_ore, 0, geode_cost_obs, 0],
                ],
            }
        })
        // .inspect(|b| {dbg!(b);})
        .collect_vec()
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let blueprints = input_generator(input);
    let turns = 24;

    blueprints
        .iter()
        .map(|blueprint| {
            let mut cache: BTreeMap<[usize; 4], Vec<(usize, [usize; 4])>> = BTreeMap::new();
            let mut data = Data {
                resources: [0, 0, 0, 0],
                robots: [1, 0, 0, 0],
            };
            let mut seen = 0;
            let mut max_at = vec![0; turns + 1];
            let res = backtrace(
                blueprint,
                turns,
                &mut data,
                &mut cache,
                &mut seen,
                &mut max_at,
                8,
            );
            assert_eq!(data.resources, [0, 0, 0, 0]);
            assert_eq!(data.robots, [1, 0, 0, 0]);
            res * blueprint.id
            // assert_eq!(data.resources, [0,0,0,0]);
        })
        .sum()
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    let blueprints = input_generator(input);
    let turns = 32;

    blueprints
        .iter()
        .take(3)
        .map(|blueprint| {
            let mut cache: BTreeMap<[usize; 4], Vec<(usize, [usize; 4])>> = BTreeMap::new();
            let mut data = Data {
                resources: [0, 0, 0, 0],
                robots: [1, 0, 0, 0],
            };
            let mut seen = 0;
            let mut max_at = vec![0; turns + 1];
            let res = backtrace(
                blueprint,
                turns,
                &mut data,
                &mut cache,
                &mut seen,
                &mut max_at,
                15,
            );
            assert_eq!(data.resources, [0, 0, 0, 0]);
            assert_eq!(data.robots, [1, 0, 0, 0]);
            res
            // assert_eq!(data.resources, [0,0,0,0]);
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 33);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 3472);
    }

    static INPUT: &str = include_str!("../input/2022/day19.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 1192);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 14725);
    }
}
