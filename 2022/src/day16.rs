use itertools::Itertools;
use scan_fmt::*;
use std::collections::{HashMap, BTreeSet};

#[derive(Debug)]
struct Valve {
    id: usize,
    flow_rate: usize,
    neighbours: Vec<usize>,
}

#[derive(Debug)]
struct Data {
    pressure: usize,
    pressure_per_turn: usize,
    left_to_open: usize,
    map: Vec<Vec<usize>>,
    open: Vec<bool>,
    current_max: usize,
    max_flow: usize,
}

fn go_to(data: &mut Data, dist: usize) {
    data.pressure += dist * data.pressure_per_turn;
}
fn come_back(data: &mut Data, dist: usize) {
    data.pressure -= dist * data.pressure_per_turn;
}
fn open_valve(data: &mut Data, valve: &Valve) {
    data.open[valve.id] = true;
    data.left_to_open -= 1;
    data.pressure += data.pressure_per_turn;
    data.pressure_per_turn += valve.flow_rate;
}
fn close_valve(data: &mut Data, valve: &Valve) {
    data.pressure_per_turn -= valve.flow_rate;
    data.pressure -= data.pressure_per_turn;
    data.open[valve.id] = false;
    data.left_to_open += 1;
}

fn backtrace(valves: &Vec<Valve>, turns_left: usize, position: usize, data: &mut Data) -> usize {
    // println!("at pos {} with {} turns", position, turns_left);
    if data.current_max >= data.pressure + turns_left * data.max_flow {
        return data.current_max;
    }

    if turns_left == 0 {
        // eprintln!("  | best = {}", data.pressure);
        data.current_max = data.pressure;
        return data.current_max;
    }
    if data.left_to_open == 0 {
        // eprintln!("  | all open = {}", data.pressure + turns_left * data.pressure_per_turn);
        data.current_max = data.pressure + turns_left * data.pressure_per_turn;
        return data.current_max;
    }

    let mut max = data.pressure + turns_left * data.pressure_per_turn;
    let valve = &valves[position];

    if !data.open[position] {
        open_valve(data, valve);
        // eprintln!("  |  | open valve = {} at turn {}", position, turns_left);
        max = max.max(backtrace(valves, turns_left - 1, position, data));
        close_valve(data, valve);
    } else {
        for neighbour in 0..valves.len() {
            if neighbour != position
                && !data.open[neighbour]
                && data.map[position][neighbour] < turns_left
            {
                let dist = data.map[position][neighbour];
                go_to(data, dist);
                // eprintln!("  |  | move from {} to {} with dist {}", position, neighbour, data.map[position][neighbour]);
                max = max.max(backtrace(valves, turns_left - dist, neighbour, data));
                come_back(data, dist);
            }
        }
    }

    max
}

fn input_generator(input: &str) -> (usize, Vec<Valve>, Data) {
    let mut valves_ids: HashMap<String, usize> = HashMap::new();
    let mut valves_rev_ids = HashMap::new();
    // let mut valves = vec![];

    let mut valves = input
        .lines()
        .into_iter()
        .map(|line| {
            let (sid, flow_rate, neighbours) = scan_fmt!(
                line,
                "Valve {} has flow rate={}; tunnel{*[s]} lead{*[s]} to valve{*[s]} {/.*/}",
                String,
                usize,
                String
            )
            .unwrap();
            let neighbours = neighbours.split(", ").map(|s| s.to_owned()).collect_vec();

            // create ids if needed
            let len = valves_ids.len();
            valves_ids.entry(sid.clone()).or_insert(len);
            for n in &neighbours {
                let len = valves_ids.len();
                valves_ids.entry(n.clone()).or_insert(len);
            }

            let id = *valves_ids.get(&sid).unwrap();
            let neighbours = neighbours
                .into_iter()
                .map(|n| *valves_ids.get(&n).unwrap())
                .collect_vec();

            valves_rev_ids.insert(id, sid);
            Valve {
                id,
                flow_rate,
                neighbours,
            }
        })
        .collect_vec();

    valves.sort_by_key(|valve| valve.id);

    let len = valves.len();
    let mut fw = vec![vec![usize::MAX / 3; len]; len];
    for valve in &valves {
        fw[valve.id][valve.id] = 0;
        for neigh in &valve.neighbours {
            fw[valve.id][*neigh] = 1;
        }
    }
    for k in 0..len {
        for i in 0..len {
            for j in 0..len {
                if fw[i][j] > fw[i][k] + fw[k][j] {
                    fw[i][j] = fw[i][k] + fw[k][j];
                }
            }
        }
    }

    for valve in &valves {
        assert_eq!(fw[valve.id][valve.id], 0);
        for neigh in &valve.neighbours {
            assert_eq!(fw[valve.id][*neigh], 1);
        }
    }

    for i in 0..len {
        for j in 0..len {
            assert!(fw[i][j] <= len);
        }
    }

    let mut data = Data {
        pressure: 0,
        pressure_per_turn: 0,
        left_to_open: len,
        map: fw,
        open: vec![false; len],
        current_max: 0,
        max_flow: valves.iter().map(|v| v.flow_rate).sum::<usize>(),
    };

    for position in 0..len {
        if valves[position].flow_rate == 0 {
            data.open[position] = true;
            data.left_to_open -= 1;
        }
    }

    let start_id = *valves_ids.get("AA").unwrap();
    (start_id, valves, data)
}
#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    let (start_id, valves, mut data) = input_generator(input);
    let turns_left = 30;
    backtrace(&valves, turns_left, start_id, &mut data)
}

fn powerset<T>(s: &[T]) -> Vec<BTreeSet<T>>
where
    T: Clone + PartialEq + Eq + PartialOrd + Ord,
{
    (0..2usize.pow(s.len() as u32))
        .map(|i| {
            s.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, element)| element.clone())
                .collect()
        })
        .collect()
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    let (start_id, valves, mut data) = input_generator(input);
    let turns_left = 26;
    let mut valids = vec![];
    for valve in &valves {
        if valve.flow_rate > 0 {
            valids.push(valve.id);
        }
    }

    let sets = powerset(&valids);
    let mut scores = vec![0; sets.len()];
    println!("{:?}", &valids);
    println!("{}", sets.len());
    for (idx, set) in sets.iter().enumerate() {
        if idx % 1000 == 0 {
            eprintln!("{idx}");
        }
        data.left_to_open = 0;
        data.current_max = 0;
        data.max_flow = 0;
        for i in data.open.iter_mut() {
            *i = true;
        }
        for i in set {
            // eprint!("{i} ");
            data.open[*i] = false;
            data.left_to_open += 1;
            data.max_flow += valves[*i].flow_rate;
        }
        scores[idx] = backtrace(&valves, turns_left, start_id, &mut data);

        let start_open = data.left_to_open;
        assert_eq!(data.pressure, 0);
        assert_eq!(data.pressure_per_turn, 0);
        assert_eq!(data.left_to_open, start_open);
        assert_eq!(data.open.iter().filter(|b| !**b).count(), start_open);
    }

    let mut max = 0;
    for (i, set1) in sets.iter().enumerate() {
        for (j, set2) in sets.iter().skip(i+1).enumerate() {
            if set1.is_disjoint(set2) && scores[i] + scores[j] > max {
                max = scores[i] + scores[j];
            }
        }
    }
    // eprintln!("\n{:?}", &scores);
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    #[test]
    fn sample1() {
        assert_eq!(part1(EXAMPLE), 1651);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(EXAMPLE), 1707);
    }
}
