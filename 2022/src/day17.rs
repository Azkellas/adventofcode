use std::collections::{HashMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,     derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::Mul,
)]
struct Pos {
    x: i64,
    y: i64,
}

fn solve(input: &str, steps: usize) -> i64 {
    let tiles = vec![
        vec![Pos{x:0, y:0}, Pos{x:1, y:0}, Pos{x:2, y:0}, Pos{x:3, y:0}],
        vec![Pos{x:1, y:0}, Pos{x:0, y:1}, Pos{x:1, y:1}, Pos{x:2, y:1}, Pos{x:1, y:2}],
        vec![Pos{x:0, y:0}, Pos{x:1, y:0}, Pos{x:2, y:0}, Pos{x:2, y:1}, Pos{x:2, y:2}],
        vec![Pos{x:0, y:0}, Pos{x:0, y:1}, Pos{x:0, y:2}, Pos{x:0, y:3}],
        vec![Pos{x:0, y:0}, Pos{x:1, y:0}, Pos{x:0, y:1}, Pos{x:1, y:1}],
    ];

    let width = 7;
    let mut map = BTreeSet::new();
    for x in 0..7 {
        map.insert(Pos{x, y: 0});
    }

    let mut seen: HashMap<(usize, usize), Vec<(usize, i64)>> = HashMap::new();

    let mut wind_idx = 0;

    for step in 0..steps {
        let tile_id = step % 5;
        let tile = &tiles[tile_id];
        let height = map.iter().map(|v| v.y).max().unwrap();
        let mut pos = Pos{x: 2, y: height + 4};

        let key = (wind_idx, tile_id);
        let value = (step, height);

        if seen.contains_key(&key) {
            let vals = seen.get_mut(&key).unwrap();
            if vals.len() == 1 {
                vals.push(value);
            } else {
                let len = vals.len();
                let prev_delta = (
                    vals[len - 1].0 - vals[len - 2].0,
                    vals[len - 1].1 - vals[len - 2].1,
                );
                let delta = (value.0 - vals[len - 1].0, value.1 - vals[len - 1].1);
                if delta == prev_delta {
                    let delta_height = delta.1;
                    let delta_step = delta.0;
                    let turns_remaining = steps - step;
                    let plays = turns_remaining / delta_step;
                    let to_play = turns_remaining % delta_step;
                    if to_play == 0 {
                        // eprintln!("found recursion after {step} steps");
                        return map.iter().map(|v| v.y).max().unwrap() + plays as i64 * delta_height;
                    }
                } else {
                    vals.push(value);
                }
            }
        } else {
            seen.insert(key, vec![value]);
        }

        loop {
            let wind_dir = input.as_bytes()[wind_idx];
            wind_idx += 1;
            if wind_idx == input.as_bytes().len() {
                wind_idx = 0;
            }

            // wind
            let mut possible = true;
            for elt in tile {
                let mut elt_pos = pos + *elt;
                elt_pos.x += if wind_dir == b'<' { -1 } else { 1 };

                if elt_pos.x < 0 || elt_pos.x >= width {
                    possible = false;
                    break;
                }
                if map.contains(&elt_pos) {
                    possible = false;
                    break;
                }
            }
            if possible {
                pos.x += if wind_dir == b'<' { -1 } else { 1 };
            }

            // gravity
            let mut possible = true;
            for elt in tile {
                let mut elt_pos = pos + *elt;
                elt_pos.y -= 1;
                if map.contains(&elt_pos) {
                    possible = false;
                    break;
                }
            }
            if possible {
                pos.y -= 1;
            }
            else {
                for elt in tile {
                    let elt_pos = pos + *elt;
                    map.insert(elt_pos);
                }
                break;
            }
        }
    }

    map.iter().map(|v| v.y).max().unwrap()
}
#[aoc(day17, part1)]
pub fn part1(input: &str) -> i64 {
    solve(input, 2022)
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> i64 {
    solve(input, 1_000_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn sample1() {
        assert_eq!(part1(EXAMPLE), 3068);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(EXAMPLE), 1514285714288);
    }
    
    static INPUT: &str = include_str!("../input/2022/day17.txt");
    #[test]
    fn sample3() {
        assert_eq!(part1(INPUT), 3157);
    }

    #[test]
    fn sample4() {
        assert_eq!(part2(INPUT), 1581449275319);
    }
}
