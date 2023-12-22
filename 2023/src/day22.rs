use aoc_runner_derive::aoc;
use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Brick {
    x: RangeInclusive<u32>,
    y: RangeInclusive<u32>,
    z: RangeInclusive<u32>,

    belows: Vec<usize>,
    above: Vec<usize>,
}

impl Brick {
    pub fn intersects(&self, other: &Brick) -> bool {
        (self.x.contains(other.x.start())
            || self.x.contains(other.x.end())
            || other.x.contains(self.x.start())
            || other.x.contains(self.x.end()))
            && (self.y.contains(other.y.start())
                || self.y.contains(other.y.end())
                || other.y.contains(self.y.start())
                || other.y.contains(self.y.end()))
    }
}

fn input_generator(input: &str) -> Vec<Brick> {
    let mut bricks = input
        .lines()
        .map(|line| {
            let (x1, y1, z1, x2, y2, z2) =
                scan_fmt!(line, "{},{},{}~{},{},{}", u32, u32, u32, u32, u32, u32).unwrap();
            Brick {
                x: RangeInclusive::new(x1, x2),
                y: RangeInclusive::new(y1, y2),
                z: RangeInclusive::new(z1, z2),
                belows: vec![],
                above: vec![],
            }
        })
        .collect_vec();

    bricks.sort_by(|a, b| a.z.start().cmp(b.z.start()));
    for idx in 0..bricks.len() {
        for i in 0..idx {
            if bricks[i].intersects(&bricks[idx]) {
                bricks[idx].belows.push(i);
            }
        }
    }

    bricks
}

fn fall(bricks: &mut Vec<Brick>) {
    let count = bricks.len();

    for idx in 0..count {
        let belows = bricks[idx]
            .belows
            .iter()
            .map(|&i| (i, *bricks[i].z.end()))
            .collect_vec();
        let max_z = belows.iter().map(|&(_, z)| z).max().unwrap_or(0);

        bricks[idx].z = (max_z + 1)..=(max_z + 1 + bricks[idx].z.end() - bricks[idx].z.start());

        bricks[idx].belows = belows
            .into_iter()
            .filter(|&(_, z)| z == max_z)
            .map(|(i, _)| i)
            .collect_vec();
    }

    for idx in 0..count {
        for i in bricks[idx].belows.clone() {
            bricks[i].above.push(idx);
        }
    }
    // for (idx, brick) in bricks.iter().enumerate() {
    //     println!("{idx}: {brick:?}");
    // }
}

#[aoc(day22, part1)]
fn part1(input: &str) -> usize {
    let mut bricks = input_generator(input);
    fall(&mut bricks);

    let count = bricks.len();
    let mut disintegrate = vec![true; count];
    for idx in 0..count {
        if bricks[idx].belows.len() == 1 {
            disintegrate[bricks[idx].belows[0]] = false;
        }
    }
    disintegrate.into_iter().filter(|&b| b).count()
}

#[aoc(day22, part2)]
fn part2(input: &str) -> usize {
    let mut bricks = input_generator(input);
    fall(&mut bricks);

    let count = bricks.len();

    let mut total = 0;

    for idx in 0..count {
        let mut falling = vec![false; count];
        let mut queue = vec![idx];
        while let Some(b) = queue.pop() {
            if !falling[b] {
                falling[b] = true;
                for a in &bricks[b].above {
                    if bricks[*a].belows.iter().all(|i| falling[*i]) {
                        queue.push(*a);
                    }
                }
            }
        }

        let score = falling.iter().filter(|&b| *b).count() - 1;
        // println!("{idx}: {score}");
        total += score;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 5);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 7);
    }

    static INPUT: &str = include_str!("../input/2023/day22.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 434);
    }

    #[test]
    fn input_part2() {
        // steps 26501365
        assert_eq!(part2(INPUT), 61209);
    }
}
