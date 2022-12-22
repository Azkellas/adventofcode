use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use itertools::Itertools;

#[derive(Default)]
struct Number {
    val: i64,
    prev: RefCell<Weak<Number>>,
    next: RefCell<Weak<Number>>,
}

impl Number {
    fn set_prev(&self, prev: Weak<Number>) {
        *self.prev.borrow_mut() = prev;
    }
    fn set_next(&self, next: Weak<Number>) {
        *self.next.borrow_mut() = next;
    }
    fn get_val(&self) -> i64 {
        self.val
    }
    fn get_prev(&self) -> Weak<Number> {
        self.prev.borrow().clone()
    }
    fn get_next(&self) -> Weak<Number> {
        self.next.borrow().clone()
    }
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> i64 {
    let numbers = input
        .lines()
        .map(|v| v.parse::<i64>().unwrap())
        .map(|val| Number {
            val,
            ..Default::default()
        })
        .map(|n| RefCell::new(Rc::new(n)))
        .collect_vec();

    let ncount = numbers.len();
    for idx in 0..ncount {
        let prev = Rc::downgrade(&numbers[(ncount + idx - 1) % ncount].borrow());
        let next = Rc::downgrade(&numbers[(idx + 1) % ncount].borrow());
        numbers[idx].borrow().set_prev(prev);
        numbers[idx].borrow().set_next(next);
    }

    for idx in 0..ncount {
        let mut spot = Rc::downgrade(&numbers[idx].borrow());
        let mut moves = numbers[idx].borrow().get_val();
        if moves == 0 {
            continue;
        }

        // move out elt
        let from = Rc::downgrade(&numbers[idx].borrow());
        let prev = from.upgrade().unwrap().get_prev();
        let next = from.upgrade().unwrap().get_next();
        prev.upgrade().unwrap().set_next(next.clone());
        next.upgrade().unwrap().set_prev(prev.clone());

        moves = moves % (ncount as i64 - 1);

        if moves < 0 {
            for _ in 0..=i64::abs(moves) {
                spot = spot.upgrade().unwrap().get_prev();
            }
        }
        if moves > 0 {
            for _ in 0..i64::abs(moves) {
                spot = spot.upgrade().unwrap().get_next();
            }
        }

        let prev = spot.clone();
        let next = spot.upgrade().unwrap().get_next();
        prev.upgrade().unwrap().set_next(from.clone());
        next.upgrade().unwrap().set_prev(from.clone());
        from.upgrade().unwrap().set_prev(prev.clone());
        from.upgrade().unwrap().set_next(next.clone());
        // eprintln!(
        //     "set {} between {} and {}",
        //     from.upgrade().unwrap().get_val(),
        //     prev.upgrade().unwrap().get_val(),
        //     next.upgrade().unwrap().get_val()
        // );
    }

    let spot = numbers.iter().find(|&n| n.borrow().get_val() == 0).unwrap();
    let mut spot = Rc::downgrade(&spot.borrow());
    let mut res = 0;
    for step in 1..=3000 {
        spot = spot.upgrade().unwrap().get_next();
        if step % 1000 == 0 {
            println!("found {}", spot.upgrade().unwrap().get_val());
            res += spot.upgrade().unwrap().get_val();
        }
    }

    res
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> i64 {
    let key = 811589153;
    let numbers = input
        .lines()
        .map(|v| v.parse::<i64>().unwrap() * key)
        .map(|val| Number {
            val,
            ..Default::default()
        })
        .map(|n| RefCell::new(Rc::new(n)))
        .collect_vec();

    let ncount = numbers.len();
    for idx in 0..ncount {
        let prev = Rc::downgrade(&numbers[(ncount + idx - 1) % ncount].borrow());
        let next = Rc::downgrade(&numbers[(idx + 1) % ncount].borrow());
        numbers[idx].borrow().set_prev(prev);
        numbers[idx].borrow().set_next(next);
    }

    for _step in 0..10 {
        for idx in 0..ncount {
            let mut spot = Rc::downgrade(&numbers[idx].borrow());
            let mut moves = numbers[idx].borrow().get_val();
            moves = moves % (ncount as i64 - 1);
            if moves == 0 {
                continue;
            }

            if moves < 0 {
                for _ in 0..=i64::abs(moves) {
                    spot = spot.upgrade().unwrap().get_prev();
                }
            }
            if moves > 0 {
                for _ in 0..i64::abs(moves) {
                    spot = spot.upgrade().unwrap().get_next();
                }
            }

            // move out elt
            let from = Rc::downgrade(&numbers[idx].borrow());
            let prev = from.upgrade().unwrap().get_prev();
            let next = from.upgrade().unwrap().get_next();
            prev.upgrade().unwrap().set_next(next.clone());
            next.upgrade().unwrap().set_prev(prev.clone());

            let prev = spot.clone();
            let next = spot.upgrade().unwrap().get_next();
            prev.upgrade().unwrap().set_next(from.clone());
            next.upgrade().unwrap().set_prev(from.clone());
            from.upgrade().unwrap().set_prev(prev.clone());
            from.upgrade().unwrap().set_next(next.clone());
            // eprintln!("set {} between {} and {}", from.upgrade().unwrap().get_val(), prev.upgrade().unwrap().get_val(), next.upgrade().unwrap().get_val());
        }
    }

    let spot = numbers.iter().find(|&n| n.borrow().get_val() == 0).unwrap();
    let mut spot = Rc::downgrade(&spot.borrow());
    let mut res = 0;
    for step in 1..=3000 {
        spot = spot.upgrade().unwrap().get_next();
        if step % 1000 == 0 {
            println!("found {}", spot.upgrade().unwrap().get_val());
            res += spot.upgrade().unwrap().get_val();
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
1
2
-3
3
-2
0
4";

    #[test]
    fn sample1() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(EXAMPLE), 1623178306);
    }

    static INPUT: &str = include_str!("../input/2022/day20.txt");
    #[test]
    fn sample3() {
        assert_eq!(part1(INPUT), 2203);
    }

    #[test]
    fn sample4() {
        assert_eq!(part2(INPUT), 6641234038999);
    }
}
