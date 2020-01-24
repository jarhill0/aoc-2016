use std::env::args;
use std::io::{stdin, Read};
use std::process::exit;

mod solutions;
mod util;

fn main() {
    let mut args = args();
    if args.len() == 3 {
        let day_num = args.nth(1).unwrap().parse::<u8>();
        let part_num = args.nth(0).unwrap().parse::<u8>(); // index 2

        match (day_num, part_num) {
            (Ok(day_num), Ok(part_num)) => match solutions::look_up(day_num) {
                Some(solver) => match part_num {
                    1 => solver.part1(input()),
                    2 => solver.part2(input()),
                    _ => quit("Bad partnum. Should be 1 or 2."),
                },
                None => quit("Unknown daynum."),
            },
            _ => quit("Error parsing daynum and/or partnum as int."),
        }
    } else {
        quit(&format!(
            "Usage: {} <daynum> <partnum>",
            args.nth(0).unwrap()
        ));
    }
}

fn quit(message: &str) -> ! {
    eprintln!("{}", message);
    exit(1);
}

fn input() -> String {
    let mut inp = String::new();
    if let Err(message) = stdin().read_to_string(&mut inp) {
        quit(&format!("Error reading input: {}", message));
    }
    inp.trim().to_string()
}
