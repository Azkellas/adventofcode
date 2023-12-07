use itertools::Itertools;
use std::ops::Range;

#[derive(Debug)]
pub struct Mapping {
    dest_start: usize,
    source_start: usize,
    range_length: usize,
}

fn intersect_ranges(a: &Range<usize>, b: &Range<usize>) -> Option<Range<usize>> {
    let min = a.start.max(b.start);
    let max = a.end.min(b.end);
    if min < max {
        Some(min..max)
    } else {
        None
    }
}

impl Mapping {
    fn map_source(&self, value: usize) -> Option<usize> {
        let range = self.source_start..(self.source_start + self.range_length);
        if range.contains(&value) {
            Some(self.dest_start + (value - self.source_start))
        } else {
            None
        }
    }

    fn map_source_range(&self, in_range: &Range<usize>) -> Option<(Range<usize>, Range<usize>)> {
        let range = self.source_start..(self.source_start + self.range_length);
        let new_source_range = intersect_ranges(&range, in_range);
        new_source_range.map(|range| {
            (
                (self.dest_start + (range.start - self.source_start))
                    ..(self.dest_start + (range.end - self.source_start)),
                range,
            )
        })
    }
}

#[derive(Debug)]
pub struct Map {
    _name: String,
    mappings: Vec<Mapping>,
}

pub fn input_generator(almanac: &str) -> (Vec<usize>, Vec<Map>) {
    let almanac = almanac.replace("\r\n", "\n");
    let (seeds, maps) = almanac.split_once("\n\n").unwrap();
    let seeds = seeds
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect_vec();

    let maps = maps
        .split("\n\n")
        .map(|map| {
            let (name, mappings) = map.split_once(":\n").unwrap();
            let mappings = mappings
                .lines()
                .map(|mapping| {
                    let (dest_start, source_start, range_length) = mapping
                        .split_whitespace()
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap();

                    Mapping {
                        dest_start,
                        source_start,
                        range_length,
                    }
                })
                .collect_vec();

            Map {
                _name: name.to_owned(),
                mappings,
            }
        })
        .collect_vec();

    (seeds, maps)
}

#[aoc(day5, part1)]
pub fn part1(almanac: &str) -> usize {
    let (seeds, maps) = input_generator(almanac);

    seeds
        .into_iter()
        .map(|seed| {
            maps.iter().fold(seed, |acc, map| {
                map.mappings
                    .iter()
                    .find_map(|mapping| mapping.map_source(acc))
                    .unwrap_or(acc)
            })
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn part2(almanac: &str) -> usize {
    let (seeds, maps) = input_generator(almanac);

    let ranges = seeds
        .into_iter()
        .tuples()
        .map(|(range_start, range_size)| Range {
            start: range_start,
            end: range_start + range_size,
        })
        .collect_vec();

    maps.into_iter()
        .fold(ranges.clone(), |acc, map| {
            let data = acc
                .clone()
                .into_iter()
                .flat_map(|range| {
                    map.mappings
                        .iter()
                        .map(|mapping| mapping.map_source_range(&range))
                        .collect_vec()
                })
                .filter(Option::is_some)
                .collect_vec();

            let mut found = data
                .iter()
                .map(|range| range.clone().unwrap().0)
                .collect_vec();

            let valid = data
                .iter()
                .map(|range| range.clone().unwrap().1)
                .collect_vec();

            let mut not_found = vec![];
            let mut to_be_broken: Vec<Range<usize>> = acc;
            while let Some(next) = to_be_broken.pop() {
                let mut found_break = false;
                for range in valid.iter() {
                    if next.contains(&range.start) {
                        found_break = true;
                        let left = next.start..(usize::min(range.start, next.end));
                        if !left.is_empty() {
                            to_be_broken.push(left);
                        }
                    }
                    if next.contains(&(range.end - 1)) {
                        found_break = true;
                        let right: Range<usize> = (usize::max(range.end, next.start))..next.end;
                        if !right.is_empty() {
                            to_be_broken.push(right);
                        }
                    }

                    if found_break {
                        break;
                    }
                }
                if !found_break {
                    not_found.push(next);
                }
            }

            found.append(&mut not_found);
            found
        })
        .into_iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 35);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 46);
    }

    static INPUT: &str = include_str!("../input/2023/day5.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 340994526);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 52210644);
    }
}
