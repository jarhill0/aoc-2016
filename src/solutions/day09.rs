use crate::solutions::Solution;

pub struct Day9 {}

impl Solution for Day9 {
    fn part1(&self, input: String) {
        println!("{}", decompress1_len(&input));
    }

    fn part2(&self, _input: String) {}
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
            let rep: usize = rep.parse().unwrap();

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
