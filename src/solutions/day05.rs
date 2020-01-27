use md5;

use crate::solutions::Solution;

pub struct Day5 {}

impl Solution for Day5 {
    fn part1(&self, input: String) {
        println!("{}", password(&input));
    }

    fn part2(&self, _input: String) {}
}

fn hash(door: &str, num: u64) -> String {
    let input = format!("{}{}", door, num);
    let output = md5::compute(input);
    format!("{:x}", output)
}

fn matches(hash: &str) -> Option<char> {
    if hash[..5].chars().all(|c| c == '0') {
        hash.chars().nth(5)
    } else {
        None
    }
}

fn password(door: &str) -> String {
    (0..std::u64::MAX)
        .map(|num| matches(&hash(door, num)))
        .filter(|result| result.is_some())
        .map(|result| result.unwrap())
        .take(8)
        .collect()
}
