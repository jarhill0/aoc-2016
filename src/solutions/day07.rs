use crate::solutions::Solution;

pub struct Day7 {}

impl Solution for Day7 {
    fn part1(&self, input: String) {
        let answer = input.split('\n').filter(supports_tls).count();
        println!("{}", answer);
    }

    fn part2(&self, _input: String) {}
}

fn outsides(input: &str) -> impl Iterator<Item = &str> {
    input.split(|c| c == '[' || c == ']').step_by(2)
}

fn insides(input: &str) -> impl Iterator<Item = &str> {
    input.split(|c| c == '[' || c == ']').skip(1).step_by(2)
}

fn has_abba(seq: &str) -> bool {
    seq.chars()
        .zip(seq.chars().skip(1))
        .zip(seq.chars().skip(2))
        .zip(seq.chars().skip(3))
        .map(|(((a, b), c), d)| (a, b, c, d))
        .any(|(a, b, c, d)| a == d && b == c && a != b)
}

fn supports_tls(address: &&str) -> bool {
    outsides(address).any(has_abba) && !insides(address).any(has_abba)
}
