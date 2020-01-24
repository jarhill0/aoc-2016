use crate::solutions::Solution;
use crate::util::int;

pub struct Day1 {}

impl Solution for Day1 {
    fn part1(&self, input: String) {
        use Direction::*;

        let mut x_offset = 0;
        let mut y_offset = 0;
        let mut direction = North;

        for movement in input.split(", ") {
            let (turn, magnitude) = movement.split_at(1);
            let magnitude = int(magnitude).unwrap();

            match turn {
                "R" => direction = direction.right(),
                "L" => direction = direction.left(),
                _ => panic!("Bad direction."),
            }

            match direction {
                East => x_offset += magnitude,
                West => x_offset -= magnitude,
                North => y_offset += magnitude,
                South => y_offset -= magnitude,
            }
        }
        println!("{}", x_offset.abs() + y_offset.abs());
    }

    fn part2(&self, input: String) {}
}

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
