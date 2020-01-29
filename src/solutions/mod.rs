pub trait Solution {
    fn part1(&self, input: String);
    fn part2(&self, input: String);
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day12;
mod day13;
mod day14;

pub fn look_up(day: u8) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(day01::Day1 {})),
        2 => Some(Box::new(day02::Day2 {})),
        3 => Some(Box::new(day03::Day3 {})),
        4 => Some(Box::new(day04::Day4 {})),
        5 => Some(Box::new(day05::Day5 {})),
        6 => Some(Box::new(day06::Day6 {})),
        7 => Some(Box::new(day07::Day7 {})),
        8 => Some(Box::new(day08::Day8 {})),
        9 => Some(Box::new(day09::Day9 {})),
        10 => Some(Box::new(day10::Day10 {})),
        12 => Some(Box::new(day12::Day12 {})),
        13 => Some(Box::new(day13::Day13 {})),
        14 => Some(Box::new(day14::Day14 {})),
        _ => None,
    }
}
