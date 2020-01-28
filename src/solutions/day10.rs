use std::collections::HashMap;

use crate::solutions::Solution;

pub struct Day10 {}

impl Solution for Day10 {
    fn part1(&self, input: String) {
        let mut bots = Bots::new(true);
        for instruction in input.split('\n').map(Instruction::new) {
            bots.take_instruction(instruction);
        }
    }

    fn part2(&self, input: String) {
        let mut bots = Bots::new(false);
        for instruction in input.split('\n').map(Instruction::new) {
            bots.take_instruction(instruction);
        }
        let product = bots.output_0.unwrap() * bots.output_1.unwrap() * bots.output_2.unwrap();
        println!("{}", product);
    }
}

struct Bots {
    bots: HashMap<u32, Bot>,
    crucial_pair: (u32, u32),
    print_crucial: bool,

    output_0: Option<u32>,
    output_1: Option<u32>,
    output_2: Option<u32>,
}

impl Bots {
    fn new(print_crucial: bool) -> Bots {
        Bots {
            bots: HashMap::new(),
            crucial_pair: (61, 17),
            print_crucial,

            output_0: None,
            output_1: None,
            output_2: None,
        }
    }

    #[allow(clippy::map_entry)]
    fn get_mut(&mut self, num: u32) -> &mut Bot {
        if !self.bots.contains_key(&num) {
            self.bots
                .insert(num, Bot::new(num, self.crucial_pair, self.print_crucial));
        }
        self.bots.get_mut(&num).unwrap()
    }

    fn take_instruction(&mut self, instruction: Instruction) {
        let mut pending = vec![instruction];

        while !pending.is_empty() {
            let instruction = pending.pop().unwrap();
            let result = match instruction {
                Instruction::Give { chip, target } => match target {
                    Target::Bot(bot) => self.get_mut(bot).accept(chip),
                    Target::Output(output) => {
                        match output {
                            0 => self.output_0 = Some(chip),
                            1 => self.output_1 = Some(chip),
                            2 => self.output_2 = Some(chip),
                            _ => (),
                        }
                        None
                    }
                },
                Instruction::SetTargets { bot, low, high } => {
                    self.get_mut(bot).set_targets(low, high)
                }
            };

            if let Some((gift_1, gift_2)) = result {
                let gift_1 = Instruction::from(gift_1);
                let gift_2 = Instruction::from(gift_2);
                pending.push(gift_1);
                pending.push(gift_2);
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
    print_interesting: bool,
    number: u32,
}

type Gifts = ((Target, u32), (Target, u32));

impl Bot {
    fn new(number: u32, interesting_chips: (u32, u32), print_interesting: bool) -> Bot {
        Bot {
            chip_a: None,
            chip_b: None,
            targets: None,
            number,
            interesting_chips: pair_sort(interesting_chips.0, interesting_chips.1),
            print_interesting,
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
                if self.print_interesting && (low, high) == self.interesting_chips {
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
