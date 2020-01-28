use std::collections::HashMap;

use crate::solutions::Solution;

pub struct Day10 {}

impl Solution for Day10 {
    fn part1(&self, input: String) {
        let mut bots = Bots::new();
        for instruction in input.split('\n').map(Instruction::new) {
            bots.take_instruction(instruction);
        }
    }

    fn part2(&self, _input: String) {}
}

struct Bots {
    bots: HashMap<u32, Bot>,
    pending: Vec<Instruction>,
    crucial_pair: (u32, u32),
}

impl Bots {
    fn new() -> Bots {
        Bots {
            bots: HashMap::new(),
            pending: Vec::new(),
            crucial_pair: (61, 17),
        }
    }

    #[allow(clippy::map_entry)]
    fn get_mut(&mut self, num: u32) -> &mut Bot {
        if !self.bots.contains_key(&num) {
            self.bots.insert(num, Bot::new(num, self.crucial_pair));
        }
        self.bots.get_mut(&num).unwrap()
    }

    fn take_instruction(&mut self, instruction: Instruction) {
        self.pending.push(instruction);

        while !self.pending.is_empty() {
            let instruction = self.pending.pop().unwrap();
            let result = match instruction {
                Instruction::Give { chip, target } => {
                    if let Target::Bot(bot) = target {
                        self.get_mut(bot).accept(chip)
                    } else {
                        None
                    }
                }
                Instruction::SetTargets { bot, low, high } => {
                    self.get_mut(bot).set_targets(low, high)
                }
            };

            if let Some((gift_1, gift_2)) = result {
                let gift_1 = Instruction::from(gift_1);
                let gift_2 = Instruction::from(gift_2);
                self.pending.push(gift_1);
                self.pending.push(gift_2);
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Give { chip: u32, target: Target },
    SetTargets { bot: u32, low: Target, high: Target },
}

#[derive(Debug, Copy, Clone)]
enum Target {
    Bot(u32),
    Output(u32),
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let mut line = line.split_whitespace();
        match line.next().unwrap() {
            "value" => {
                let chip = line.next().unwrap().parse().unwrap();
                let target = Target::Bot(line.nth(3).unwrap().parse().unwrap());
                Instruction::Give { chip, target }
            }
            "bot" => {
                let bot = line.next().unwrap().parse().unwrap();
                let low_is_bot = line.nth(3).unwrap() == "bot";
                let low_num = line.next().unwrap().parse().unwrap();
                let low = if low_is_bot {
                    Target::Bot(low_num)
                } else {
                    Target::Output(low_num)
                };
                let high_is_bot = line.nth(3).unwrap() == "bot";
                let high_num = line.next().unwrap().parse().unwrap();
                let high = if high_is_bot {
                    Target::Bot(high_num)
                } else {
                    Target::Output(high_num)
                };
                Instruction::SetTargets { bot, low, high }
            }
            _ => panic!("Bad instruction."),
        }
    }

    fn from(gift: (Target, u32)) -> Instruction {
        Instruction::Give {
            target: gift.0,
            chip: gift.1,
        }
    }
}

#[derive(Debug)]
struct Bot {
    chip_a: Option<u32>,
    chip_b: Option<u32>,

    targets: Option<(Target, Target)>,
    interesting_chips: (u32, u32),
    number: u32,
}

type Gifts = ((Target, u32), (Target, u32));

impl Bot {
    fn new(number: u32, interesting_chips: (u32, u32)) -> Bot {
        Bot {
            chip_a: None,
            chip_b: None,
            targets: None,
            number,
            interesting_chips: pair_sort(interesting_chips.0, interesting_chips.1),
        }
    }

    fn accept(&mut self, chip: u32) -> Option<Gifts> {
        match (self.chip_a, self.chip_b) {
            (None, None) => self.chip_a = Some(chip),
            (Some(_), None) => self.chip_b = Some(chip),
            _ => panic!("Not ready to accept a chip."),
        }

        self.attempt_give()
    }

    fn set_targets(&mut self, low: Target, high: Target) -> Option<Gifts> {
        self.targets = Some((low, high));

        self.attempt_give()
    }

    fn attempt_give(&mut self) -> Option<Gifts> {
        if let Some((low_target, high_target)) = self.targets {
            if let (Some(a), Some(b)) = (self.chip_a, self.chip_b) {
                let (low, high) = pair_sort(a, b);
                if (low, high) == self.interesting_chips {
                    println!("{}", self.number);
                }

                self.chip_a = None;
                self.chip_b = None;
                self.targets = None;

                return Some(((low_target, low), (high_target, high)));
            }
        }
        None
    }
}

fn pair_sort<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
