use std::cmp::Ordering;
use std::collections::HashMap;

use crate::solutions::Solution;
use crate::util::int;

pub struct Day4 {}

impl Solution for Day4 {
    fn part1(&self, input: String) {
        let total: i32 = input
            .split('\n')
            .map(Room::new)
            .filter(|room| room.calculate_checksum() == room.checksum)
            .map(|room| room.sector_id)
            .sum();
        println!("{}", total);
    }

    fn part2(&self, input: String) {
        for room in input.split('\n').map(Room::new) {
            let shift_by = room.sector_id;
            let shifted: String = room.name.chars().map(|c| shift(c, shift_by)).collect();
            if "northpole object storage" == shifted {
                println!("{}", room.sector_id);
                break;
            }
        }
    }
}

fn shift(c: char, by: i32) -> char {
    if c == '-' {
        return ' ';
    }
    let c = c as i32 - 'a' as i32;
    let c = (c + by) % 26;
    ('a' as i32 + c) as u8 as char
}

struct Room<'a> {
    name: &'a str,
    checksum: &'a str,
    sector_id: i32,
}

impl Room<'_> {
    fn new(room: &str) -> Room {
        let mut parts = room.rsplitn(2, '-');
        let mut end = parts.next().unwrap().split('[');
        let name = parts.next().unwrap();
        let sector_id = int(end.next().unwrap()).unwrap();
        let checksum = end.next().unwrap();
        let checksum = &checksum[..checksum.len() - 1];
        Room {
            name,
            checksum,
            sector_id,
        }
    }

    fn calculate_checksum(&self) -> String {
        let mut counts = HashMap::new();
        for c in self.name.chars().filter(|&c| c != '-') {
            counts.insert(c, 1 + counts.get(&c).unwrap_or(&0));
        }

        let mut counts: Vec<(&u32, &char)> = counts.iter().map(|(a, b)| (b, a)).collect();
        counts.sort_unstable_by(|(an, ac), (bn, bc)| {
            let num_cmp = bn.cmp(an);
            if let Ordering::Equal = num_cmp {
                ac.cmp(bc)
            } else {
                num_cmp
            }
        });

        counts[..5].iter().map(|(_, &letter)| letter).collect()
    }
}
