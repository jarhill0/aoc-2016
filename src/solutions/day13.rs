use std::collections::{HashSet, VecDeque};

use crate::solutions::Solution;

pub struct Day13 {}

impl Solution for Day13 {
    fn part1(&self, input: String) {
        let fav_num = input.parse().unwrap();
        println!("{}", bfs(fav_num));
    }

    fn part2(&self, _input: String) {}
}

fn bfs(fav_num: u32) -> u32 {
    let mut visited = HashSet::new();
    visited.insert((1, 1));
    let mut queue = VecDeque::new();
    queue.push_back(((1, 1), 0));
    loop {
        let (pos, dist) = queue.pop_front().unwrap();
        if pos == (31, 39) {
            return dist;
        }

        for adjacent in surroundings(pos)
            .iter()
            .filter_map(|&x| x)
            .filter(|&adj| !is_wall(adj, fav_num))
        {
            if !visited.contains(&adjacent) {
                visited.insert(adjacent);
                queue.push_back((adjacent, dist + 1));
            }
        }
    }
}

fn surroundings(coord: (u32, u32)) -> [Option<(u32, u32)>; 4] {
    let (x, y) = coord;
    [
        Some((x + 1, y)),
        Some((x, y + 1)),
        if x > 0 { Some((x - 1, y)) } else { None },
        if y > 0 { Some((x, y - 1)) } else { None },
    ]
}

fn is_wall(coord: (u32, u32), fav_num: u32) -> bool {
    let (x, y) = coord;
    (x * x + 3 * x + 2 * x * y + y + y * y + fav_num).count_ones() % 2 == 1
}
