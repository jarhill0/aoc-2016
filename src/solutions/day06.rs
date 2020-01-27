use std::collections::HashMap;

use crate::solutions::Solution;

pub struct Day6 {}

impl Day6 {
    fn solve(&self, input: &str, solver: impl Fn(usize) -> char) {
        let length = input.split('\n').next().unwrap().len();
        let answer: String = (0..length).map(solver).collect();
        println!("{}", answer);
    }
}

impl Solution for Day6 {
    fn part1(&self, input: String) {
        let lines = line_list(&input);
        self.solve(&input, |pos| most_freq_at_pos(lines.iter(), pos))
    }

    fn part2(&self, input: String) {
        let lines = line_list(&input);
        self.solve(&input, |pos| least_freq_at_pos(lines.iter(), pos))
    }
}

fn line_list(input: &str) -> Vec<&str> {
    input.split('\n').collect()
}

fn most_freq_at_pos<'a>(lines: impl Iterator<Item = &'a &'a str>, pos: usize) -> char {
    *count(lines, pos)
        .iter()
        .map(|(c, num)| (num, c))
        .max()
        .unwrap()
        .1
}

fn count<'a>(lines: impl Iterator<Item = &'a &'a str>, pos: usize) -> HashMap<char, u32> {
    let mut count = HashMap::new();
    for c in lines.map(|l| l.chars().nth(pos).unwrap()) {
        count.insert(c, 1 + count.get(&c).unwrap_or(&0));
    }
    count
}

fn least_freq_at_pos<'a>(lines: impl Iterator<Item = &'a &'a str>, pos: usize) -> char {
    *count(lines, pos)
        .iter()
        .map(|(c, num)| (num, c))
        .min()
        .unwrap()
        .1
}
