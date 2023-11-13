use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Room {
    name: String,
    sector: i32,
    checksum: String,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Room> {
    input
        .split('\n')
        .map(|line| Room {
            name: line[0..line.rfind('-').unwrap()].to_string(),
            sector: line[line.rfind('-').unwrap() + 1..line.rfind('[').unwrap()]
                .parse()
                .unwrap(),
            checksum: line[line.rfind('[').unwrap() + 1..line.rfind(']').unwrap()].to_string(),
        })
        .collect_vec()
}

#[aoc(day4, part1)]
pub fn part1(rooms: &[Room]) -> i32 {
    let mut count = 0;
    for room in rooms {
        let checksum = room
            .name
            .chars()
            .sorted()
            .dedup_with_count()
            .filter(|(_, c)| c.is_alphabetic())
            .sorted_by(|a, b| {
                if a.0 != b.0 {
                    a.0.cmp(&b.0).reverse()
                } else {
                    a.1.cmp(&b.1)
                }
            })
            .take(5)
            .map(|(_, c)| c)
            .collect::<String>();
        if checksum == room.checksum {
            count += room.sector;
        }
    }
    count
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Part2Res {
    name: String,
    sector: i32,
}

impl std::fmt::Display for Part2Res {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.name, self.sector)
    }
}

#[aoc(day4, part2)]
pub fn part2(rooms: &[Room]) -> Part2Res {
    for room in rooms {
        let name = room
            .name
            .chars()
            .map(|c| {
                if c == '-' {
                    return ' ';
                }
                let mut c = c as u8 - b'a';
                c = ((c as i32 + room.sector) % 26) as u8;
                c += b'a';
                c as char
            })
            .collect::<String>();

        if name == "northpole object storage" || name == "very encrypted name" {
            return Part2Res {
                name,
                sector: room.sector,
            };
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 1514);
    }

    #[test]
    fn sample2() {
        assert_eq!(
            part2(&input_generator("qzmt-zixmtkozy-ivhz-343[]")).name,
            "very encrypted name"
        );
    }
    static INPUT: &str = include_str!("../input/2016/day4.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 361724);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(&input_generator(INPUT)).sector, 482);
    }
}
