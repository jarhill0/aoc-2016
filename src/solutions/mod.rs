pub trait Solution {
    fn part1(&self, input: String);
    fn part2(&self, input: String);
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

pub fn look_up(day: u8) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(day01::Day1 {})),
        2 => Some(Box::new(day02::Day2 {})),
        3 => Some(Box::new(day03::Day3 {})),
        4 => Some(Box::new(day04::Day4 {})),
        5 => Some(Box::new(day05::Day5 {})),
        _ => None,
    }
}
