use regex::Regex;
use std::collections::HashSet;


pub struct Instruction {
    pub turn: String,
    pub steps: i32
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"([RL])([0-9]+)").unwrap();

    let mut instructions = vec![];
    for cap in re.captures_iter(input) {
        let (turn, steps) = (cap[1].to_owned(), cap[2].parse::<i32>().unwrap());
        instructions.push(Instruction{turn, steps});
    }
    
    instructions
}

const NORTH: i32 = 0;
const EAST: i32  = 1;
const SOUTH: i32 = 2;
const WEST: i32  = 3;

#[aoc(day1, part1)]
pub fn part1(instructions: &[Instruction]) -> i32 {
    let mut dir = NORTH;
    let (mut x, mut y) = (0, 0);

    for inst in instructions {
        match inst.turn.as_str() {
            "R" => dir += 1,
            "L" => dir -= 1,
            _ => unreachable!(),
        };
        if dir > WEST {
            dir = NORTH;
        }
        if dir < NORTH {
            dir = WEST;
        }

        match dir {
            NORTH => y -= inst.steps,
            EAST  => x += inst.steps,
            SOUTH => y += inst.steps,
            WEST  => x -= inst.steps,
            _ => unreachable!()
        }
    }
    i32::abs(x) + i32::abs(y)
}

#[aoc(day1, part2)]
pub fn part2(instructions: &[Instruction]) -> i32 {
    let mut dir = NORTH;
    let (mut x, mut y) = (0, 0);

    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    let mut count = 0;
    for inst in instructions {
        count += 1;
        match inst.turn.as_str() {
            "R" => dir += 1,
            "L" => dir -= 1,
            _ => unreachable!(),
        };
        if dir > WEST {
            dir = NORTH;
        }
        if dir < NORTH {
            dir = WEST;
        }

        let delta = match dir {
            NORTH => ( 0, -1),
            EAST  => ( 1,  0),
            SOUTH => ( 0,  1),
            WEST  => (-1,  0),
            // _ => (0, 0)
            _ => unreachable!()
        };

        for _ in 0..inst.steps {
            x += delta.0;
            y += delta.1;
            if seen.contains(&(x, y)) {
                eprintln!("Found after {count} steps");
                return i32::abs(x) + i32::abs(y);
            }
            seen.insert((x, y));
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator("R2, L3")), 5);
    }

    #[test]
    fn sample2() {
        assert_eq!(part1(&input_generator("R2, R2, R2")), 2);
    }

    #[test]
    fn sample3() {
        assert_eq!(part1(&input_generator("R5, L5, R5, R3")), 12);
    }

    #[test]
    fn sample4() {
        assert_eq!(part2(&input_generator("R8, R4, R4, R8")), 4);
    }

}
