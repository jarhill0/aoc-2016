use std::collections::HashMap;

use crate::solutions::Solution;

pub struct Day6 {}

impl Solution for Day6 {
    fn part1(&self, input: String) {
        let lines = line_list(&input);
        let length = lines[0].len();
        let answer: String = (0..length)
            .map(|pos| most_freq_at_pos(lines.iter(), pos))
            .collect();
        println!("{}", answer);
    }

    fn part2(&self, _input: String) {}
}

fn line_list(input: &str) -> Vec<&str> {
    input.split('\n').collect()
}

fn most_freq_at_pos<'a>(lines: impl Iterator<Item = &'a &'a str>, pos: usize) -> char {
    let mut count = HashMap::new();
    for c in lines.map(|l| l.chars().nth(pos).unwrap()) {
        count.insert(c, 1 + count.get(&c).unwrap_or(&0));
    }
    *count.iter().map(|(c, num)| (num, c)).max().unwrap().1
}
