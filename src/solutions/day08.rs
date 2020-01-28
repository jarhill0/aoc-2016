use crate::solutions::Solution;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

pub struct Day8 {}

impl Solution for Day8 {
    fn part1(&self, input: String) {
        let mut screen = Screen::new();
        input
            .split('\n')
            .filter_map(Operation::new)
            .for_each(|op| screen.operate(op));
        println!("{}", screen.count_on());
    }

    fn part2(&self, _input: String) {}
}

struct Screen {
    pixels: [[bool; WIDTH]; HEIGHT],
}

impl Screen {
    fn new() -> Screen {
        Screen {
            pixels: [[false; WIDTH]; HEIGHT],
        }
    }

    fn operate(&mut self, op: Operation) {
        use Operation::*;

        match op {
            Rect { w, h } => self.rect(w, h),
            RotateRow { y, by } => self.rotate_row(y, by),
            RotateColumn { x, by } => self.rotate_column(x, by),
        }
    }

    fn rect(&mut self, w: usize, h: usize) {
        for y in 0..h {
            for x in 0..w {
                self.pixels[y][x] = true;
            }
        }
    }

    fn rotate_row(&mut self, y: usize, by: usize) {
        let mut row = self.pixels[y].to_vec();
        let by = by % row.len();
        row.rotate_right(by);
        self.pixels[y] = to_row_arr(row);
    }

    fn rotate_column(&mut self, x: usize, by: usize) {
        let mut col: Vec<bool> = self.pixels.iter().map(|row| row[x]).collect();
        let by = by % col.len();
        col.rotate_right(by);
        for (y, result) in col.iter().enumerate() {
            self.pixels[y][x] = *result;
        }
    }

    fn count_on(&self) -> usize {
        self.pixels
            .iter()
            .map(|row| row.iter())
            .flatten()
            .filter(|&&b| b)
            .count()
    }
}

fn to_row_arr(items: Vec<bool>) -> [bool; WIDTH] {
    let mut arr = [false; WIDTH];
    for (i, x) in items.iter().enumerate() {
        arr[i] = *x;
    }
    arr
}

#[derive(Copy, Clone, Debug)]
enum Operation {
    Rect { w: usize, h: usize },
    RotateRow { y: usize, by: usize },
    RotateColumn { x: usize, by: usize },
}

impl Operation {
    fn new(text: &str) -> Option<Operation> {
        let mut text = text.split_whitespace();
        match text.next()? {
            "rect" => {
                let mut dims = text.next()?.split('x');
                Some(Operation::Rect {
                    w: dims.next()?.parse2u()?,
                    h: dims.next()?.parse2u()?,
                })
            }
            "rotate" => match text.next()? {
                "row" => {
                    let y = text.next()?[2..].parse2u()?;
                    let by = text.nth(1)?.parse2u()?;
                    Some(Operation::RotateRow { y, by })
                }
                "column" => {
                    let x = text.next()?[2..].parse2u()?;
                    let by = text.nth(1)?.parse2u()?;
                    Some(Operation::RotateColumn { x, by })
                }
                _ => None,
            },
            _ => None,
        }
    }
}

trait ParseToUsize {
    fn parse2u(&self) -> Option<usize>;
}

impl ParseToUsize for str {
    fn parse2u(&self) -> Option<usize> {
        match self.parse() {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }
}
