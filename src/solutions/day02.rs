use crate::solutions::Solution;

pub struct Day2 {}

impl Solution for Day2 {
    fn part1(&self, input: String) {
        let mut code = String::new();
        let mut key_pad = KeyPad::new();

        for sequence in parse_input(&input) {
            key_pad.steps(sequence);
            code += &key_pad.digit.to_string();
        }

        println!("{}", code);
    }

    fn part2(&self, input: String) {}
}

#[derive(Copy, Clone)]
struct KeyPad {
    digit: u8,
}

impl KeyPad {
    fn new() -> KeyPad {
        KeyPad { digit: 5 }
    }

    fn steps(&mut self, directions: impl Iterator<Item = Direction>) {
        for direction in directions {
            self.step(direction)
        }
    }

    fn step(&mut self, direction: Direction) {
        use Direction::*;

        match direction {
            Up => self.up(),
            Down => self.down(),
            Right => self.right(),
            Left => self.left(),
        }
    }

    fn up(&mut self) {
        if self.digit > 3 {
            self.digit -= 3;
        }
    }

    fn down(&mut self) {
        if self.digit < 7 {
            self.digit += 3;
        }
    }

    fn right(&mut self) {
        if self.digit % 3 != 0 {
            self.digit += 1;
        }
    }

    fn left(&mut self) {
        if self.digit % 3 != 1 {
            self.digit -= 1;
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from(l: char) -> Direction {
        use Direction::*;

        match l {
            'L' => Left,
            'R' => Right,
            'U' => Up,
            'D' => Down,
            _ => panic!("{:?} is an invalid Direction literal.", l),
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = impl Iterator<Item = Direction> + '_> + '_ {
    input
        .split('\n')
        .map(|line| line.chars().map(Direction::from))
}
