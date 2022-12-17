use itertools::Itertools;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i32 {
    let mut values = vec![1, 1];
    let mut value = 1;
    input
        .lines()
        .into_iter()
        .map(|line| line.split_whitespace().collect_vec())
        .for_each(|line| {
            values.push(value);
            if line[0] == "addx" {
                value += line[1].parse::<i32>().unwrap();
                values.push(value);
            }
        });

    (20..=220).step_by(40).map(|i| values[i] * i as i32).sum()
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> String {
    let mut res = String::new();
    let mut show = |step, value| {
        let step_x = step % 40;
        if step_x == 0 {
            res.push('\n');
        }
        res.push(if i32::abs(step_x as i32 - value) <= 1 {
            '#'
        } else {
            ' '
        });
    };

    let mut value = 1;
    let mut step = 0;
    input
        .lines()
        .into_iter()
        .map(|line| line.split_whitespace().collect_vec())
        .for_each(|line| {
            show(step, value);
            step += 1;
            if line[0] == "addx" {
                show(step, value);
                step += 1;
                value += line[1].parse::<i32>().unwrap();
            }
        });
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(part1(EXAMPLE), 13140);
    }

    #[test]
    fn sample2() {
        let res = "
##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     ";
        assert_eq!(part2(EXAMPLE), res.to_owned());
    }

    static EXAMPLE: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
