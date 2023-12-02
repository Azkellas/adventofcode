use itertools::Itertools;
use scan_fmt::*;

#[derive(Debug)]
pub struct Hand {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    minimal_set: Hand,
    _hands: Vec<Hand>,
}

pub fn input_generator(games: &str) -> Vec<Game> {
    games
        .lines()
        .into_iter()
        .map(|line| {
            let (game_id, hands) = line.split_once(": ").unwrap();
            let id = scan_fmt!(game_id, "Game {d}", u32).unwrap();
            let hands = hands
                .split(';')
                .map(|hand| {
                    let colors = hand.split(',').collect_vec();
                    let mut hand = Hand {
                        red: 0,
                        blue: 0,
                        green: 0,
                    };
                    for color in colors {
                        let color_parse = scan_fmt!(color, "{d} {}", u32, String);
                        let (count, color) = color_parse.unwrap();
                        match color.as_str() {
                            "red" => hand.red = count,
                            "blue" => hand.blue = count,
                            "green" => hand.green = count,
                            _ => unreachable!(),
                        }
                    }
                    hand
                })
                .collect_vec();

            let minimal_set = hands.iter().fold(
                Hand {
                    red: 0,
                    blue: 0,
                    green: 0,
                },
                |mut acc, hand| {
                    acc.red = acc.red.max(hand.red);
                    acc.blue = acc.blue.max(hand.blue);
                    acc.green = acc.green.max(hand.green);
                    acc
                },
            );

            Game {
                id,
                minimal_set,
                _hands: hands,
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(games: &str) -> u32 {
    let games = input_generator(games);
    games
        .iter()
        .filter(|game| {
            game.minimal_set.red <= 12
                && game.minimal_set.green <= 13
                && game.minimal_set.blue <= 14
        })
        .map(|game| game.id)
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(games: &str) -> u32 {
    let games = input_generator(games);
    games
        .iter()
        .map(|game| game.minimal_set.red * game.minimal_set.green * game.minimal_set.blue)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 8);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 2286);
    }

    static INPUT: &str = include_str!("../input/2023/day2.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 1853);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 72706);
    }
}
