use crate::solutions::Solution;

pub struct Day12 {}

impl Solution for Day12 {
    fn part1(&self, input: String) {
        let mut vm = VM::new(input.split('\n').filter_map(Instruction::new).collect());
        vm.run();
        println!("{}", vm.a);
    }

    fn part2(&self, input: String) {
        let mut vm = VM::new(input.split('\n').filter_map(Instruction::new).collect());
        vm.c = 1;
        vm.run();
        println!("{}", vm.a);
    }
}

#[derive(Debug)]
struct VM {
    a: i32,
    b: i32,
    c: i32,
    d: i32,

    instructions: Vec<Instruction>,
    pc: isize,
}

impl VM {
    fn new(instructions: Vec<Instruction>) -> VM {
        VM {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            instructions,
            pc: 0,
        }
    }

    fn run(&mut self) {
        while self.pc >= 0 && self.pc < self.instructions.len() as isize {
            let instruction = self.instructions[self.pc as usize];
            match instruction {
                Instruction::Cpy { source, dest } => self.set_val(dest, self.source_val(source)),
                Instruction::Inc { reg } => self.set_val(reg, 1 + self.val_of(reg)),
                Instruction::Dec { reg } => self.set_val(reg, self.val_of(reg) - 1),
                Instruction::Jnz { cond, offset } => {
                    if self.source_val(cond) != 0 {
                        self.pc += offset - 1 // -1 to account for increment below
                    }
                }
            }
            self.pc += 1;
        }
    }

    fn source_val(&self, source: Source) -> i32 {
        match source {
            Source::Reg(reg) => self.val_of(reg),
            Source::Imm(imm) => imm,
        }
    }

    fn val_of(&self, reg: Register) -> i32 {
        use Register::*;

        match reg {
            A => self.a,
            B => self.b,
            C => self.c,
            D => self.d,
        }
    }

    fn set_val(&mut self, reg: Register, val: i32) {
        use Register::*;

        match reg {
            A => self.a = val,
            B => self.b = val,
            C => self.c = val,
            D => self.d = val,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Cpy { source: Source, dest: Register },
    Inc { reg: Register },
    Dec { reg: Register },
    Jnz { cond: Source, offset: isize },
}

impl Instruction {
    fn new(line: &str) -> Option<Instruction> {
        use Instruction::*;

        let mut line = line.split_whitespace();
        match line.next()? {
            "cpy" => {
                let source = Source::new(line.next()?)?;
                let dest = Register::new(line.next()?)?;
                Some(Cpy { source, dest })
            }
            "inc" => Some(Inc {
                reg: Register::new(line.next()?)?,
            }),
            "dec" => Some(Dec {
                reg: Register::new(line.next()?)?,
            }),
            "jnz" => {
                let cond = Source::new(line.next()?)?;
                match line.next()?.parse() {
                    Ok(offset) => Some(Jnz { cond, offset }),
                    Err(_) => None,
                }
            }
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Register {
    A,
    B,
    C,
    D,
}

impl Register {
    fn new(name: &str) -> Option<Register> {
        use Register::*;

        match name {
            "a" => Some(A),
            "b" => Some(B),
            "c" => Some(C),
            "d" => Some(D),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Source {
    Reg(Register),
    Imm(i32),
}

impl Source {
    fn new(thing: &str) -> Option<Source> {
        use Source::*;

        match Register::new(thing) {
            Some(reg) => Some(Reg(reg)),
            None => match thing.parse() {
                Ok(result) => Some(Imm(result)),
                Err(_) => None,
            },
        }
    }
}
