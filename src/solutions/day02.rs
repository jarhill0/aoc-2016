use crate::solutions::Solution;

pub struct Day2 {}

impl Day2 {
    fn run(&self, input: String, layout: impl Layout) {
        let mut code = String::new();
        let mut key_pad = KeyPad::new(layout);

        for sequence in parse_input(&input) {
            key_pad.steps(sequence);
            code.push(key_pad.as_char());
        }

        println!("{}", code.to_ascii_uppercase());
    }
}

impl Solution for Day2 {
    fn part1(&self, input: String) {
        self.run(input, Square::new());
    }

    fn part2(&self, input: String) {
        self.run(input, Diamond::new());
    }
}

#[derive(Copy, Clone)]
struct KeyPad<T: Layout> {
    digit: u32,
    layout: T,
}

impl<T: Layout> KeyPad<T> {
    fn new(layout: T) -> KeyPad<T> {
        KeyPad { digit: 5, layout }
    }

    fn steps(&mut self, directions: impl Iterator<Item = Direction>) {
        for direction in directions {
            self.step(direction)
        }
    }

    fn step(&mut self, direction: Direction) {
        use Direction::*;

        self.digit = match direction {
            Up => self.layout.up(self.digit),
            Down => self.layout.down(self.digit),
            Right => self.layout.right(self.digit),
            Left => self.layout.left(self.digit),
        }
    }

    fn as_char(&self) -> char {
        std::char::from_digit(self.digit, 16).unwrap()
    }
}

trait Layout {
    fn up(&self, digit: u32) -> u32;
    fn down(&self, digit: u32) -> u32;
    fn left(&self, digit: u32) -> u32;
    fn right(&self, digit: u32) -> u32;
}

struct Square {}

impl Square {
    fn new() -> Square {
        Square {}
    }
}

impl Layout for Square {
    fn up(&self, digit: u32) -> u32 {
        if digit > 3 {
            digit - 3
        } else {
            digit
        }
    }

    fn down(&self, digit: u32) -> u32 {
        if digit < 7 {
            digit + 3
        } else {
            digit
        }
    }

    fn right(&self, digit: u32) -> u32 {
        if digit % 3 != 0 {
            digit + 1
        } else {
            digit
        }
    }

    fn left(&self, digit: u32) -> u32 {
        if digit % 3 != 1 {
            digit - 1
        } else {
            digit
        }
    }
}

struct Diamond {}

impl Diamond {
    fn new() -> Diamond {
        Diamond {}
    }
}

impl Layout for Diamond {
    fn up(&self, digit: u32) -> u32 {
        match digit {
            5 | 2 | 1 | 4 | 9 => digit,
            3 => 1,
            6 | 7 | 8 | 0xA | 0xB | 0xC => digit - 4,
            0xD => 0xB,
            _ => panic!("unreachable"),
        }
    }

    fn down(&self, digit: u32) -> u32 {
        match digit {
            5 | 0xA | 0xD | 0xC | 9 => digit,
            0xB => 0xD,
            2 | 3 | 4 | 6 | 7 | 8 => digit + 4,
            1 => 3,
            _ => panic!("unreachable"),
        }
    }

    fn right(&self, digit: u32) -> u32 {
        match digit {
            1 | 4 | 9 | 0xC | 0xD => digit,
            8 | 7 | 6 | 5 | 3 | 2 | 0xA | 0xB => digit + 1,
            _ => panic!("unreachable"),
        }
    }

    fn left(&self, digit: u32) -> u32 {
        match digit {
            1 | 2 | 5 | 0xA | 0xD => digit,
            3 | 4 | 6 | 7 | 8 | 9 | 0xB | 0xC => digit - 1,
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
