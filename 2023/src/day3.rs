#[derive(Debug)]
pub struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Range {
    y: usize,
    start: usize,
    end: usize,
}

#[derive(Debug)]
pub enum Item {
    Symbol(char, Pos),
    Number(u32, Range),
}

impl Item {
    pub fn is_adjacent(&self, other: &Self) -> bool {
        let is_adjacent_impl = |pos: &Pos, range: &Range| {
            pos.y + 1 >= range.y
                && pos.y <= range.y + 1
                && pos.x + 1 >= range.start
                && pos.x <= range.end + 1
        };

        match self {
            Self::Symbol(_, pos) => match other {
                Self::Number(_, range) => is_adjacent_impl(pos, range),
                Self::Symbol(_, _) => false,
            },
            Self::Number(_, range) => match other {
                Self::Number(_, _) => false,
                Self::Symbol(_, pos) => is_adjacent_impl(pos, range),
            },
        }
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol(_, _))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_, _))
    }
}

pub fn input_generator(grid: &str) -> Vec<Item> {
    let mut items = vec![];

    grid.lines().enumerate().for_each(|(y, line)| {
        let mut number = 0;
        for (x, c) in line.chars().enumerate() {
            let mut do_push = true;
            match c {
                '.' => {}
                '0'..='9' => {
                    number = number * 10 + c.to_digit(10).unwrap();
                    do_push = false;
                }
                _ => items.push(Item::Symbol(c, Pos { x, y })),
            };

            let last_elt = !do_push && x + 1 == line.chars().count();
            if number != 0 && do_push || last_elt {
                let x = x + last_elt as usize;
                items.push(Item::Number(
                    number,
                    Range {
                        y,
                        start: x - number.to_string().len(),
                        end: x - 1,
                    },
                ));
                number = 0;
            }
        }
    });

    items
}

#[aoc(day3, part1)]
pub fn part1(grid: &str) -> u32 {
    let items = input_generator(grid);

    items
        .iter()
        .filter(|item| matches!(item, Item::Number(_, _)))
        .filter(|item| match item {
            Item::Number(_, _) => items
                .iter()
                .any(|other| other.is_symbol() && item.is_adjacent(other)),
            Item::Symbol(..) => unreachable!(),
        })
        .map(|item| match item {
            Item::Number(number, _) => number,
            Item::Symbol(..) => unreachable!(),
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(grid: &str) -> u32 {
    let items = input_generator(grid);

    items
        .iter()
        .filter(|item| matches!(item, Item::Symbol('*', _)))
        .map(|item| {
            items
                .iter()
                .filter(|other| other.is_number() && item.is_adjacent(other))
        })
        .filter(|iterator| iterator.clone().count() == 2)
        .map(|iterator| {
            iterator
                .map(|item| match item {
                    Item::Number(number, _) => number,
                    Item::Symbol(..) => unreachable!(),
                })
                .product::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 4361);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 467835);
    }

    static INPUT: &str = include_str!("../input/2023/day3.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 540131);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 86879020);
    }
}
