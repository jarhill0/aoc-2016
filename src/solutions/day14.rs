use md5;

use crate::solutions::Solution;

pub struct Day14 {}

impl Solution for Day14 {
    fn part1(&self, input: String) {
        let answer = (0..std::u64::MAX)
            .filter(|&ind| is_otp_key(&input, ind))
            .nth(63)
            .unwrap();
        println!("{}", answer);
    }

    fn part2(&self, _input: String) {}
}

fn hash(base: &str, index: u64) -> String {
    let input = format!("{}{}", base, index);
    let output = md5::compute(input);
    format!("{:x}", output)
}

fn triplet(hash: &str) -> Option<char> {
    let mut rpt = ' ';
    let mut run = 0;
    for c in hash.chars() {
        if c == rpt {
            run += 1;
            if run == 3 {
                return Some(c);
            }
        } else {
            rpt = c;
            run = 1;
        }
    }
    None
}

fn is_otp_key(base: &str, index: u64) -> bool {
    match triplet(&hash(base, index)) {
        None => false,
        Some(repeat) => has_five(repeat, base, index + 1),
    }
}

/// Check if one of the next 1000 hashes starting at the index has 5 of the
/// repeat character.
fn has_five(repeat: char, base: &str, index: u64) -> bool {
    (index..index + 1000).any(|ind| is_five(repeat, &hash(base, ind)))
}

/// Check whether this hash has five of the repeat character.
fn is_five(r: char, hash: &str) -> bool {
    let pattern = format!("{}{}{}{}{}", r, r, r, r, r);
    hash.contains(&pattern)
}
