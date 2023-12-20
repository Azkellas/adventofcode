use std::{
    collections::{HashMap, VecDeque},
    fmt::{Display, Formatter},
};

use aoc_runner_derive::aoc;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Kind {
    Button,
    Broadcast,
    FlipFlop { state: State },
    Conjunction { inputs: HashMap<String, Pulse> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    On,
    Off,
}
impl State {
    fn flip(&mut self) {
        *self = match self {
            State::On => State::Off,
            State::Off => State::On,
        };
    }

    fn get_signal(&self) -> Pulse {
        match self {
            State::On => Pulse::High,
            State::Off => Pulse::Low,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::High => write!(f, "high"),
            Pulse::Low => write!(f, "low"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Module {
    name: String,
    kind: Kind,
    outputs: Vec<String>,
}

#[derive(Clone)]
struct Signal {
    from: String,
    to: String,
    pulse: Pulse,
}

impl Display for Signal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{}-> {}", self.from, self.pulse, self.to)
    }
}

impl Module {
    // returns a list of signal to emits
    fn receive_signal(&mut self, signal: Signal) -> Vec<Signal> {
        let mut signals = Vec::new();
        match &mut self.kind {
            Kind::Button => unreachable!(),
            Kind::Broadcast => {
                for output in &self.outputs {
                    signals.push(Signal {
                        from: self.name.clone(),
                        to: output.clone(),
                        pulse: signal.pulse.clone(),
                    });
                }
            }
            Kind::FlipFlop { ref mut state } => {
                if signal.pulse == Pulse::Low {
                    state.flip();
                    let new_pulse = state.get_signal();
                    for output in &self.outputs {
                        signals.push(Signal {
                            from: self.name.clone(),
                            to: output.clone(),
                            pulse: new_pulse.clone(),
                        });
                    }
                }
            }
            Kind::Conjunction { ref mut inputs } => {
                inputs.insert(signal.from.clone(), signal.pulse.clone());

                let new_pulse = if inputs.values().all(|p| p == &Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                for output in &self.outputs {
                    signals.push(Signal {
                        from: self.name.clone(),
                        to: output.clone(),
                        pulse: new_pulse.clone(),
                    });
                }
            }
        }

        signals
    }
}
fn input_generator(input: &str) -> HashMap<String, Module> {
    let mut modules = input
        .lines()
        .map(|line| {
            let (mut name, outputs) = line.split_once(" -> ").unwrap();
            let kind;
            if name == "broadcaster" {
                kind = Kind::Broadcast;
            } else {
                match name.chars().next().unwrap() {
                    '%' => kind = Kind::FlipFlop { state: State::Off },
                    '&' => {
                        kind = Kind::Conjunction {
                            inputs: HashMap::new(),
                        }
                    }
                    _ => unreachable!("unexpected {name}"),
                }
                name = &name[1..];
            }
            let name = name.to_string();
            let outputs = outputs.split(", ").map(|s| s.to_string()).collect();
            (
                name.clone(),
                Module {
                    name,
                    kind,
                    outputs,
                },
            )
        })
        .collect::<HashMap<String, Module>>();

    modules.insert(
        "button".to_owned(),
        Module {
            name: "button".to_string(),
            kind: Kind::Button,
            outputs: vec!["broadcaster".to_string()],
        },
    );

    let modules_clone = modules.clone();
    for (name, module) in modules_clone {
        for output in &module.outputs {
            if let Some(output) = modules.get_mut(output) {
                if let Kind::Conjunction { inputs } = &mut output.kind {
                    inputs.insert(name.clone(), Pulse::Low);
                }
            }
        }
    }

    modules
}

#[aoc(day20, part1)]
fn part1(input: &str) -> usize {
    let mut modules = input_generator(input);

    let starting_modules = modules.clone();

    let mut highs = 0;
    let mut lows = 0;
    let mut step = 0;
    while step < 1000 {
        if step > 0 && modules == starting_modules {
            // println!("cycle detected at {}", step);
            let cycle = step;
            let remaining = 1000 - step;
            let cycles_done = remaining / cycle;
            let to_do = remaining % cycle;
            highs *= cycles_done + 1;
            lows *= cycles_done + 1;
            step = 1000 - to_do;
            println!("cycles done: {}", cycles_done);
            println!("to_do: {}", to_do);
            if to_do == 0 {
                break;
            }
        }

        // start step
        let mut signals = VecDeque::new();
        signals.push_back(Signal {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            pulse: Pulse::Low,
        });

        while let Some(signal) = signals.pop_front() {
            // println!("{signal}");
            if signal.pulse == Pulse::High {
                highs += 1;
            } else {
                lows += 1;
            }
            if let Some(module) = modules.get_mut(&signal.to) {
                let new_signals = module.receive_signal(signal);
                for new_signal in new_signals {
                    signals.push_back(new_signal);
                }
            }
        }

        step += 1;
    }

    println!("highs: {}, lows: {}", highs, lows);
    lows * highs
}

#[aoc(day20, part2)]
fn part2(input: &str) -> usize {
    let mut modules = input_generator(input);

    let target_modules = ["zs", "nt", "ff", "th"];
    let mut periods: Vec<Option<usize>> = vec![None; target_modules.len()];

    let mut step = 0;
    while periods.iter().any(|p| p.is_none()) {
        step += 1;
        // start step
        let mut signals = VecDeque::new();
        signals.push_back(Signal {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            pulse: Pulse::Low,
        });

        while let Some(signal) = signals.pop_front() {
            if target_modules.contains(&signal.from.as_str()) && signal.pulse == Pulse::Low {
                let idx = target_modules
                    .iter()
                    .position(|s| s == &signal.from)
                    .unwrap();

                if periods[idx].is_none() {
                    periods[idx] = Some(step);
                }
            }
            if let Some(module) = modules.get_mut(&signal.to) {
                let new_signals = module.receive_signal(signal);
                for new_signal in new_signals {
                    signals.push_back(new_signal);
                }
            }
        }
    }

    periods
        .iter()
        .map(|p| p.unwrap())
        .fold(1, num::integer::lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_1: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    static EXAMPLE_2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_1), 32000000);
        assert_eq!(part1(EXAMPLE_2), 11687500);
    }

    static INPUT: &str = include_str!("../input/2023/day20.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 821985143);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 240853834793347);
    }
}
