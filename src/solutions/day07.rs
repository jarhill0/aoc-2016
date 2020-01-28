use std::collections::HashSet;

use crate::solutions::Solution;

pub struct Day7 {}

impl Solution for Day7 {
    fn part1(&self, input: String) {
        let answer = input.split('\n').filter(supports_tls).count();
        println!("{}", answer);
    }

    fn part2(&self, input: String) {
        let answer = input.split('\n').filter(supports_ssl).count();
        println!("{}", answer);
    }
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

fn is_aba(slice: &&str) -> bool {
    if slice.len() == 3 {
        let mut slice = slice.chars();
        let a = slice.next().unwrap();
        let b = slice.next().unwrap();
        let c = slice.next().unwrap();
        a == c && a != b
    } else {
        false
    }
}

fn section_abas(section: &str) -> HashSet<&str> {
    (0..section.len() - 2)
        .map(|i| &section[i..i + 3])
        .filter(is_aba)
        .collect()
}

fn all_abas<'a>(site: impl Iterator<Item = &'a str>) -> HashSet<&'a str> {
    site.map(section_abas).flatten().collect()
}

fn supports_ssl(address: &&str) -> bool {
    let abas = all_abas(outsides(address));
    let babs = all_abas(insides(address));
    abas.iter()
        .any(|aba| babs.iter().any(|bab| aba_match(aba, bab)))
}

/// Assumes that aba and bab are both ABAs with len 3.
fn aba_match(aba: &str, bab: &str) -> bool {
    aba.chars().next().unwrap() == bab.chars().nth(1).unwrap()
        && aba.chars().nth(1).unwrap() == bab.chars().next().unwrap()
}
