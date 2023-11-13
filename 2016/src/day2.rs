use std::str;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let mut res = 0;
    let mut button = 5;
    for line in input.lines() {
        for c in line.chars() {
            match c {
                'U' => if button > 3 { button -= 3; },
                'D' => if button < 7 { button += 3; },
                'L' => if button % 3 != 1 { button -= 1; },
                'R' => if button % 3 != 0 { button += 1; },
                _ => (),
            }
        }
        res = res * 10 + button;
    }
    res
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> String {
    let pad = vec![
        "    1    ",
        "  2 3 4  ",
        "5 6 7 8 9",
        "  A B C  ",
        "    D    ",
    ];

    let (w, h) = (pad[0].len() as i32, pad.len() as i32);

    let (mut x, mut y) = (0, 2);
    let mut res = vec![];

    for line in input.lines() {
        for c in line.chars() {
            let (dx, dy) = match c {
                'U' => ( 0, -1),
                'D' => ( 0,  1),
                'L' => (-2,  0),
                'R' => ( 2,  0),
                _ => (0, 0),
            };
            let (nx, ny): (i32, i32) = (x + dx, y + dy);
            if nx < 0 || nx >= w || ny < 0 || ny >= h {
                continue;
            }
            if pad[ny as usize].as_bytes()[nx as usize] == b' ' {
                continue;
            }
            (x, y) = (nx, ny);
        }
        res.push(pad[y as usize].as_bytes()[x as usize]);
    }
    str::from_utf8(&res).unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "ULL
    RRDDD
    LURDL
    UUUUD";
    #[test]
    fn sample1() {
        assert_eq!(part1(EXAMPLE), 1985);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(EXAMPLE), "5DB3");
    }
}
