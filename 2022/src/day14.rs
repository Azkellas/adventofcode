use itertools::Itertools;
use std::collections::BTreeSet;

#[must_use]
#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> BTreeSet<(usize, usize)> {
    input
        .lines()
        .into_iter()
        .map(|line| line
            .split(" -> ")
            .map(|s| s
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap())
            .collect::<Vec<(usize, usize)>>())
        .map(|line| {
            let mut res = BTreeSet::new();
            for i in 0..line.len()-1 {
                let (from, to) = (line[i], line[i+1]);
                let delta = (to.0 as i32 - from.0 as i32, to.1 as i32 - from.1 as i32);
                let step = (delta.0.signum(), delta.1.signum());
                let steps = i32::max(delta.0.abs(), delta.1.abs());
                for i in 0..=steps {
                    let pt = ((from.0 as i32 + i * step.0) as usize, (from.1 as i32 + i * step.1) as usize);
                    res.insert(pt);
                }
            }
            res
        })
        .reduce(|acc, s| BTreeSet::union(&acc, &s).copied().collect()).unwrap()
}

#[must_use]
pub fn drop_sand(mut rocks: BTreeSet<(usize, usize)>) -> usize {
    let start_point = (500, 0);
    let max_y = rocks.iter().map(|s| s.1).max().unwrap();

    let mut steps = 0;
    loop {
        if rocks.contains(&start_point) {
            break;
        }

        let mut stable = false;
        let mut sand = start_point;
        loop {
            if sand.1 == max_y {
                break;
            }

            if !rocks.contains(&(sand.0, sand.1+1)) {
                sand.1 += 1;
            }
            else if !rocks.contains(&(sand.0-1, sand.1+1)) {
                sand.0 -= 1;
                sand.1 += 1;
            }
            else if !rocks.contains(&(sand.0+1, sand.1+1)) {
                sand.0 += 1;
                sand.1 += 1;
            }
            else {
                rocks.insert(sand);
                stable = true;
                break;
            }
        }

        if !stable {   
            break;
        }
        steps += 1;

    }

    steps
}


#[must_use]
#[aoc(day14, part1)]
pub fn part1(rocks: &BTreeSet<(usize, usize)>) -> usize {
    drop_sand(rocks.clone())
}

#[must_use]
#[aoc(day14, part2)]
pub fn part2(rocks: &BTreeSet<(usize, usize)>) -> usize {
    let mut rocks = rocks.clone();

    let max_y = rocks.iter().map(|s| s.1).max().unwrap();
    for x in 0..1000 {
        rocks.insert((x, max_y + 2));
    }
    drop_sand(rocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 24);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 93);
    }
}
