use std::collections::HashSet;

use crate::solutions::Solution;
use crate::util::int;

pub struct Day1 {}

impl Solution for Day1 {
    fn part1(&self, input: String) {
        let mut walker = Walker::new();

        for (turn, magnitude) in movements(&input) {
            walker.turn(turn);
            walker.walk(magnitude);
        }

        println!("{}", walker.manhattan_dist());
    }

    fn part2(&self, input: String) {
        let mut walker = Walker::new();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        'outer: for (turn, magnitude) in movements(&input) {
            walker.turn(turn);
            for _ in 0..magnitude {
                if !visited.insert((walker.x, walker.y)) {
                    // already present
                    break 'outer;
                }

                walker.walk(1);
            }
        }
        println!("{}", walker.manhattan_dist());
    }
}

fn movements(input: &str) -> impl Iterator<Item = (&str, i32)> {
    input.split(", ").map(|movement| {
        let (turn, magnitude) = movement.split_at(1);
        (turn, int(magnitude).unwrap())
    })
}

struct Walker {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Walker {
    fn new() -> Walker {
        Walker {
            x: 0,
            y: 0,
            direction: Direction::North,
        }
    }

    fn turn(&mut self, turn: &str) {
        match turn {
            "R" => self.direction = self.direction.right(),
            "L" => self.direction = self.direction.left(),
            _ => panic!("Bad direction."),
        }
    }

    fn walk(&mut self, magnitude: i32) {
        use Direction::*;

        match self.direction {
            East => self.x += magnitude,
            West => self.x -= magnitude,
            North => self.y += magnitude,
            South => self.y -= magnitude,
        }
    }

    fn manhattan_dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn right(self) -> Direction {
        use Direction::*;

        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn left(self) -> Direction {
        use Direction::*;

        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
}
