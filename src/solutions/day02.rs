use crate::solutions::Solution;

pub struct Day2 {}

impl Solution for Day2 {
    fn part1(&self, input: String) {
        let mut code = String::new();
        let mut key_pad = KeyPad::new();

        for sequence in parse_input(&input) {
            key_pad.steps(sequence);
            code.push(key_pad.as_char());
        }

        println!("{}", code);
    }

    fn part2(&self, input: String) {
        let mut code = String::new();
        let mut key_pad = KeyPad::new();

        for sequence in parse_input(&input) {
            key_pad.steps(sequence);
            code.push(key_pad.as_char());
        }

        println!("{}", code.to_ascii_uppercase());
    }
}

#[derive(Copy, Clone)]
struct KeyPad {
    digit: u32,
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

    fn as_char(&self) -> char {
        std::char::from_digit(self.digit, 16).unwrap()
    }

    fn up(&mut self) {
        self.digit = match self.digit {
            5 | 2 | 1 | 4 | 9 => self.digit,
            3 => 1,
            6 | 7 | 8 | 0xa | 0xb | 0xc => self.digit - 4,
            0xd => 0xb,
            _ => panic!("unreachable"),
        }
    }

    fn down(&mut self) {
        self.digit = match self.digit {
            5 | 0xa | 0xd | 0xc | 9 => self.digit,
            0xB => 0xD,
            2 | 3 | 4 | 6 | 7 | 8 => self.digit + 4,
            1 => 3,
            _ => panic!("unreachable"),
        }
    }

    fn right(&mut self) {
        self.digit = match self.digit {
            1 | 4 | 9 | 0xC | 0xD => self.digit,
            8 | 7 | 6 | 5 | 3 | 2 | 0xA | 0xB => self.digit + 1,
            _ => panic!("unreachable"),
        }
    }

    fn left(&mut self) {
        self.digit = match self.digit {
            1 | 2 | 5 | 0xA | 0xD => self.digit,
            3 | 4 | 6 | 7 | 8 | 9 | 0xB | 0xC => self.digit - 1,
            _ => panic!("unreachable"),
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
