#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Lens {
    label: String,
    focal: usize,
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
    let mut boxes = vec![vec![]; 256];

    for str in input.split(',') {
        if str.contains('=') {
            let (label, focal) = str.split_once('=').unwrap();
            let focal = focal.parse::<usize>().unwrap();

            let box_idx = hash(label);
            let bx = &mut boxes[box_idx];

            let replace_lens = bx.iter_mut().find(|l: &&mut Lens| l.label == label);

            if let Some(replace_lens) = replace_lens {
                replace_lens.focal = focal;
            } else {
                bx.push(Lens {
                    label: label.to_owned(),
                    focal,
                });
            }
        } else {
            let label = str.trim_end_matches('-');

            let box_idx = hash(label);
            let bx = &mut boxes[box_idx];

            bx.retain(|l| l.label != label);
        }
    }

    let mut sum = 0;
    for (bx_idx, bx) in boxes.into_iter().enumerate() {
        for (i, l) in bx.iter().enumerate() {
            sum += (bx_idx + 1) * l.focal * (i + 1);
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 1320);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 145);
    }

    static INPUT: &str = include_str!("../input/2023/day15.txt");
    #[test]
    fn input_part1() {
        assert_eq!(part1(INPUT), 509152);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(INPUT), 244403);
    }
}
