use std::str::Chars;

use crate::solutions::Solution;

pub struct Day9 {}

impl Solution for Day9 {
    fn part1(&self, input: String) {
        println!("{}", decompress1_len(&input));
    }

    fn part2(&self, input: String) {
        println!("{}", decompress2_len(&input));
    }
}

fn decompress2_len(inp: &str) -> usize {
    let mut inp = inp.chars();
    let mut total = 0;
    loop {
        let c = inp.next();
        if c.is_none() {
            break;
        }
        let c = c.unwrap();

        if c == '(' {
            let (len, rep) = parse_marker(&mut inp);
            let sub_slice = &inp.as_str()[..len];
            (0..len).for_each(|_| {
                inp.next();
            });
            let len = decompress2_len(sub_slice);
            total += len * rep;
        } else {
            total += 1;
        }
    }
    total
}

fn parse_marker(inp: &mut Chars) -> (usize, usize) {
    let mut len = String::new();
    let mut rep = String::new();

    loop {
        let c = inp.next().unwrap(); // well-formed input hasn't ended
        if c == 'x' {
            break;
        }
        len.push(c);
    }
    loop {
        let c = inp.next().unwrap(); // well-formed input hasn't ended
        if c == ')' {
            break;
        }
        rep.push(c);
    }

    let len = len.parse().unwrap();
    let rep = rep.parse().unwrap();
    (len, rep)
}

fn decompress1_len(inp: &str) -> usize {
    let mut out = 0;
    let mut inp = inp.chars();

    loop {
        let c = inp.next();
        if c.is_none() {
            break;
        }
        let c = c.unwrap();

        if c == '(' {
            let (len, rep) = parse_marker(&mut inp);
            out += len * rep;

            for _ in 0..len {
                inp.next();
            }
        } else {
            out += 1;
        }
    }
    out
}
