use itertools::Itertools;
use scan_fmt::*;

#[derive(Debug)]
pub struct Game {
    id: usize,
    winning_moves: Vec<usize>,
    played_moves: Vec<usize>,
}

pub fn input_generator(games: &str) -> Vec<Game> {
    games
        .lines()
        .map(|line| {
            let (game_id, game) = line.split_once(": ").unwrap();
            let id = scan_fmt!(game_id, "Card {d}", usize).unwrap();
            let (winning_moves, played_moves) = game.split_once('|').unwrap();
            let winning_moves = winning_moves
                .split_whitespace()
                .map(|number| number.parse::<usize>().unwrap())
                .collect_vec();
            let played_moves = played_moves
                .split_whitespace()
                .map(|number| number.parse::<usize>().unwrap())
                .collect_vec();

            Game {
                id,
                winning_moves,
                played_moves,
            }
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(games: &str) -> usize {
    let games = input_generator(games);

    games
        .iter()
        .map(|game| {
            game.winning_moves
                .iter()
                .filter(|winning_move| game.played_moves.contains(winning_move))
                .count()
        })
        .filter(|count| *count > 0)
        .map(|count| usize::pow(2, (count - 1) as u32))
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(games: &str) -> usize {
    let games = input_generator(games);

    let mut cards = vec![1; games.len() + 1];
    cards[0] = 0;

    games
        .iter()
        .map(|game| {
            (
                game.id,
                game.winning_moves
                    .iter()
                    .filter(|winning_move| game.played_moves.contains(winning_move))
                    .count(),
            )
        })
        .for_each(|(id, count)| {
            for i in 1..=count {
                cards[id + i] += cards[id];
            }
        });

    cards.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 13);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 30);
    }

    static INPUT: &str = include_str!("../input/2023/day4.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 21558);
    }

    // #[test]
    // fn input_part2() {
    //     assert_eq!(part2(INPUT), 86879020);
    // }
}
