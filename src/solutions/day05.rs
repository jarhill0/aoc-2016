use md5;

use crate::solutions::Solution;

pub struct Day5 {}

impl Solution for Day5 {
    fn part1(&self, input: String) {
        println!("{}", password(&input));
    }

    fn part2(&self, input: String) {
        let mut password = [None; 8];

        for (ind, c) in (0..std::u64::MAX)
            .map(|num| matches2(&hash(&input, num)))
            .filter(|result| result.is_some())
            .map(|result| result.unwrap())
        {
            if password[ind].is_none() {
                password[ind] = Some(c);
                if password.iter().all(|item| item.is_some()) {
                    // finished
                    break;
                }
            }
        }

        println!(
            "{}",
            password.iter().map(|c| c.unwrap()).collect::<String>()
        );
    }
}

fn pos(c: char) -> Option<usize> {
    if (c as u8) < b'0' {
        None
    } else {
        let val = c as u8 - b'0';
        if val <= 7 {
            Some(val as usize)
        } else {
            None
        }
    }
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

fn matches2(hash: &str) -> Option<(usize, char)> {
    if hash[..5].chars().all(|c| c == '0') {
        let position = pos(hash.chars().nth(5).unwrap());
        if let Some(position) = position {
            Some((position, hash.chars().nth(6).unwrap()))
        } else {
            None
        }
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
