use std::collections::HashMap;

#[aoc(day17, part1)]
pub fn part1(input: &str) -> usize {
    let tiles = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    let width = 7;
    let mut map = vec![];
    map.push(vec![true; width]);

    let count = 2022;

    let mut wind_idx = 0;

    for step in 0..count {
        let tile_id = step % 5;
        let tile = &tiles[tile_id];
        let mut pos = (2, map.len() + 3);

        loop {
            let wind_dir = input.as_bytes()[wind_idx];
            wind_idx += 1;
            if wind_idx == input.as_bytes().len() {
                wind_idx = 0;
            }

            // wind
            let mut possible = true;
            for elt in tile {
                let mut elt_pos = (pos.0 + elt.0, pos.1 + elt.1);
                if elt_pos.0 == 0 && wind_dir == b'<' {
                    possible = false;
                    break;
                }
                if wind_dir == b'<' {
                    elt_pos.0 -= 1;
                } else {
                    elt_pos.0 += 1;
                }
                // elt_pos.0 += if wind_dir == b'<' { -(1 as i32) } else { 1 } as usize;
                if elt_pos.0 >= width {
                    possible = false;
                    break;
                }
                if elt_pos.1 < map.len() && map[elt_pos.1][elt_pos.0] {
                    possible = false;
                    break;
                }
            }
            if possible {
                if wind_dir == b'<' {
                    pos.0 -= 1;
                } else {
                    pos.0 += 1;
                }
            }

            // gravity
            let mut possible = true;
            for elt in tile {
                let mut elt_pos = (pos.0 + elt.0, pos.1 + elt.1);
                elt_pos.1 -= 1;
                if elt_pos.1 < map.len() && map[elt_pos.1][elt_pos.0] {
                    possible = false;
                    break;
                }
            }
            if possible {
                pos.1 -= 1;
            } else {
                // eprintln!("adding tile {tile_id} at height {}", pos.1);
                for elt in tile {
                    let elt_pos = (pos.0 + elt.0, pos.1 + elt.1);
                    while elt_pos.1 >= map.len() {
                        map.push(vec![false; width]);
                    }
                    map[elt_pos.1][elt_pos.0] = true;
                }
                break;
            }
        }
        // // debug
        // if step < 10 {
        //     for y in (1..map.len()).rev() {
        //         eprintln!("{y}|{}|", map[y].iter().map(|b| if *b { '#' } else {'.' }).collect::<String>());
        //     }
        //     eprintln!("+-------+");
        //     eprintln!()
        // }
    }

    map.len() - 1
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> u64 {
    let tiles = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    let width = 7;
    let mut map = vec![];
    map.push(vec![true; width]);

    let mut wind_idx = 0;

    let mut seen: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut step = 0;
    loop {
        let tile_id = step % 5;
        let tile = &tiles[tile_id];
        let mut pos = (2, map.len() + 3);

        let key = (wind_idx, tile_id);
        let value = (step, map.len());

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
                    let turns: u64 = 1_000_000_000_000;
                    let turns_remaining = turns - step as u64;
                    let plays = turns_remaining / delta_step as u64;
                    let to_play = turns_remaining % delta_step as u64;
                    if to_play == 0 {
                        return map.len() as u64 + plays * delta_height as u64 - 1;
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
                let mut elt_pos = (pos.0 + elt.0, pos.1 + elt.1);
                if elt_pos.0 == 0 && wind_dir == b'<' {
                    possible = false;
                    break;
                }
                if wind_dir == b'<' {
                    elt_pos.0 -= 1;
                } else {
                    elt_pos.0 += 1;
                }
                if elt_pos.0 >= width {
                    possible = false;
                    break;
                }
                if elt_pos.1 < map.len() && map[elt_pos.1][elt_pos.0] {
                    possible = false;
                    break;
                }
            }
            if possible {
                if wind_dir == b'<' {
                    pos.0 -= 1;
                } else {
                    pos.0 += 1;
                }
            }

            // gravity
            let mut possible = true;
            for elt in tile {
                let mut elt_pos = (pos.0 + elt.0, pos.1 + elt.1);
                elt_pos.1 -= 1;
                if elt_pos.1 < map.len() && map[elt_pos.1][elt_pos.0] {
                    possible = false;
                    break;
                }
            }
            if possible {
                pos.1 -= 1;
            } else {
                // eprintln!("adding tile {tile_id} at height {}", pos.1);
                for elt in tile {
                    let elt_pos = (pos.0 + elt.0, pos.1 + elt.1);
                    while elt_pos.1 >= map.len() {
                        map.push(vec![false; width]);
                    }
                    map[elt_pos.1][elt_pos.0] = true;
                }
                break;
            }
        }

        step += 1;
        if step == 100_000 {
            break;
        }
    }

    unreachable!()
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
}
